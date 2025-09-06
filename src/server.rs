use std::net::SocketAddr;

use http_body_util::Full;
use hyper::{
    Request, Response, StatusCode, body::Bytes, server::conn::http1,
    service::service_fn,
};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use crate::{error::Result, specs::specs_struct::SharedSpecs};

pub struct Server {
    listener: TcpListener,
    specs: SharedSpecs,
}

impl Server {
    pub async fn new<A>(addr: A, specs: SharedSpecs) -> Result<Self>
    where
        A: Into<SocketAddr>,
    {
        Ok(Self {
            listener: TcpListener::bind(addr.into()).await?,
            specs,
        })
    }

    pub async fn run(&self) -> Result<()> {
        loop {
            let (tcp, _) = self.listener.accept().await?;
            let specs = self.specs.clone();

            tokio::task::spawn(async move {
                let conn = http1::Builder::new().serve_connection(
                    TokioIo::new(tcp),
                    service_fn(move |req| {
                        Server::handle_request(req, specs.clone())
                    }),
                );
                if let Err(e) = conn.await {
                    eprintln!("Error serving connection: {e}");
                }
            });
        }
    }

    async fn handle_request(
        req: Request<impl hyper::body::Body>,
        specs: SharedSpecs,
    ) -> Result<Response<Full<Bytes>>> {
        let specs = specs.read().await;

        if let Some(spec) = specs.0.iter().find(|s| {
            hyper::Method::from(s.method.clone()) == req.method()
                && s.url == req.uri().path()
        }) {
            let body = serde_json::to_string(&spec.response.body).unwrap();
            Ok(Response::builder()
                .status(spec.response.status.0)
                .header("content-type", "application/json")
                .body(Full::new(Bytes::from(body)))
                .unwrap())
        } else {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::from("Not found")))
                .unwrap())
        }
    }
}
