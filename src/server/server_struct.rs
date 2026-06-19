use std::{collections::HashMap, time::Duration};

use http_body_util::{BodyExt, Full};
use hyper::{
    Request, Response, StatusCode,
    body::{Bytes, Incoming},
    server::conn::http1,
    service::service_fn,
};
use hyper_util::rt::TokioIo;
use log::{debug, error, info, warn};
use tokio::{net::TcpListener, time::sleep};

use crate::{
    error::Result,
    server::{endpoint::Endpoint, router::SharedRouter, url::var::UrlVar},
    specs::body::body::Body,
};

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
        req: Request<Incoming>,
        router: SharedRouter,
    ) -> Result<Response<Full<Bytes>>> {
        let router = router.read().await;
        let mut vars = HashMap::new();

        let method = req.method().clone();
        let url = req.uri().path().to_string();
        let Some(Endpoint { request, response }) =
            router.find(&method, &url, &mut vars)
        else {
            info!("Request {} {} -> response 404.", method, url);
            return Ok(router.not_found.clone());
        };

        let response = response.get();
        if let Some(delay) = response.delay {
            sleep(Duration::from_millis(delay)).await;
        }

        if let Some(exp_body) = request
            && let Err(res) =
                Server::validate_req(req, exp_body, &vars, &router.templates)
                    .await
        {
            info!("Request {} {} -> Failed body validation.", method, url);
            return Ok(res);
        }

        let hyper_res = response.to_http_response(&vars, &router.templates)?;
        debug!(
            "Request: {} {} -> response {}",
            method, url, response.status.0
        );
        Ok(hyper_res)
    }

    async fn validate_req(
        req: Request<Incoming>,
        exp_body: &Body,
        vars: &HashMap<String, UrlVar>,
        templates: &HashMap<String, Body>,
    ) -> std::result::Result<(), Response<Full<Bytes>>> {
        let inc_bytes = match req.into_body().collect().await {
            Ok(collected) => collected.to_bytes(),
            Err(e) => {
                error!("Failed to read request body: {}", e);
                let err_res = hyper::Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Full::new(Bytes::from("Failed to read body")))
                    .unwrap();
                return Err(err_res);
            }
        };

        let inc_json: serde_yaml::Value = if inc_bytes.is_empty() {
            serde_yaml::Value::Null
        } else {
            serde_yaml::from_slice(&inc_bytes).unwrap_or_else(|_| {
                warn!("Incoming request body is not valid JSON/YAML.");
                serde_yaml::Value::Null
            })
        };

        if !exp_body.validate(&inc_json, vars, templates) {
            let bad_req = hyper::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Full::new(Bytes::from(
                    "Request body does not match expected scheme.",
                )))
                .unwrap();
            return Err(bad_req);
        }
        Ok(())
    }
}
