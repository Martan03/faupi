use std::{collections::HashMap, time::Duration};

use http_body_util::Full;
use hyper::{
    Request, Response, body::Bytes, server::conn::http1, service::service_fn,
};
use hyper_util::rt::TokioIo;
use log::{debug, error, info};
use tokio::{net::TcpListener, time::sleep};

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
        let addr = self
            .listener
            .local_addr()
            .map(|a| a.to_string())
            .unwrap_or("-".to_owned());
        info!("Server started on {addr}.");

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
                    error!("Serving connection: {e}.");
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

        let method = req.method();
        let url = req.uri().path();
        if let Some(res) = router.find(method, url, &mut vars) {
            if let Some(delay) = res.delay {
                sleep(Duration::from_millis(delay)).await;
            }

            let hyper_res = res.to_http_response(&vars)?;
            debug!("Request: {} {} -> response {}", method, url, res.status.0);
            Ok(hyper_res)
        } else {
            info!("Request {} {} -> response 404.", method, url);
            Ok(router.not_found.clone())
        }
    }
}
