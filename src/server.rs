//  Notes
//  https://docs.rs/http/latest/http/request/struct.Request.html -> How to de and se reqs.
//  https://docs.rs/axum/latest/axum/response/index.html -> All sorts of goodies for responses.

use crate::config::Config;

use axum::{
    http::{header, Method},
    Router,
};

use std::net::{IpAddr, SocketAddr};
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug)]
pub struct HttpServer {
    pub config: Config,
}

impl HttpServer {
    pub fn from(config: Config) -> HttpServer {
        HttpServer { config }
    }

    pub async fn start(self, router: Router) {
        let _span = tracing::span!(tracing::Level::DEBUG, "server_config").entered();
        let mut host = IpAddr::V4(self.config.host);
        let other_host: [u8; 4] = [127, 0, 0, 1];
        host = IpAddr::from(other_host);
        let addr = SocketAddr::new(host, self.config.port);
        let final_router = router.layer(HttpServer::create_cors_layer());
        tracing::info!("Starting server on {}", addr);
        
        axum::Server::bind(&addr)
            .serve(final_router.into_make_service()).await.unwrap();
    }

    fn create_cors_layer() -> CorsLayer {
        // Options, and Content-Type are required for the Cors to work.
        // This is because there is a preflight request sent from the client.
        tracing::info!("CORS layer added");
        return CorsLayer::new()
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
            ])
            .allow_headers([header::CONTENT_TYPE])
            .allow_origin(Any);
    }
}
