//  Notes
//  https://docs.rs/http/latest/http/request/struct.Request.html -> How to de and se reqs.
//  https://docs.rs/axum/latest/axum/response/index.html -> All sorts of goodies for responses.
use crate::config::Config;

use axum::{Router, http::{Method, header}};
use tower_http::cors::{Any, CorsLayer, Cors};
use std::net::{SocketAddr, IpAddr};
use serde::Serialize;
use serde_json::json;

#[derive(Debug)]
pub struct HttpServer {
    pub config: Config,
    // TODO what else should be here? maybe a logger?
}

impl HttpServer {
    pub fn from(config: Config) -> HttpServer {
        // TODO figure out what the hell this thing does. 
        tracing_subscriber::fmt::init();
        HttpServer {
            config,
        } 
    }

    pub async fn start(self, router: Router) {
        let host = IpAddr::V4(self.config.host);
        let addr = SocketAddr::new(host, self.config.port);
        tracing::info!("listening on {}", addr);
        axum::Server::bind(&addr).serve(router.clone().into_make_service()).await.unwrap();
    }


    fn create_cors_layer() -> CorsLayer {
        // Options, and Content-Type are required for the Cors to work.
        // This is because there is a preflight request sent from the client.

        return CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
            .allow_headers([header::CONTENT_TYPE])
            .allow_origin(Any);
    }
}

