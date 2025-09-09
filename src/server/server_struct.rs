use std::collections::HashMap;

use http_body_util::Full;
use hyper::{
    Request, Response, body::Bytes, server::conn::http1, service::service_fn,
};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use crate::{error::Result, server::router::SharedRouter};

pub struct Server {
    listener: TcpListener,
    router: SharedRouter,
}

impl Server {
    /// Creates new server on the given address
    pub async fn new(addr: (&str, u16), router: SharedRouter) -> Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(addr).await?,
            router,
        })
    }

    /// Starts the server
    pub async fn run(&self) -> Result<()> {
        loop {
            let (tcp, _) = self.listener.accept().await?;
            let router = self.router.clone();

            tokio::task::spawn(async move {
                let conn = http1::Builder::new().serve_connection(
                    TokioIo::new(tcp),
                    service_fn(move |req| {
                        Server::handle_request(req, router.clone())
                    }),
                );
                if let Err(e) = conn.await {
                    eprintln!("Error serving connection: {e}");
                }
            });
        }
    }

    /// Handles the HTTP request and returns the corresponding response
    async fn handle_request(
        req: Request<impl hyper::body::Body>,
        router: SharedRouter,
    ) -> Result<Response<Full<Bytes>>> {
        let router = router.read().await;
        let mut vars = HashMap::new();
        Ok(router.find(req.method(), req.uri().path(), &mut vars))
    }
}
