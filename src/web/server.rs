//  Notes
//  https://docs.rs/http/latest/http/request/struct.Request.html -> How to de and se reqs.
//  https://docs.rs/axum/latest/axum/response/index.html -> All sorts of goodies for responses.

use axum::{
    http::{header, Method},
    Router,
};

use std::net::{IpAddr, SocketAddr, Ipv4Addr};
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug)]
pub struct HttpServer {}

impl HttpServer {

    pub async fn start_http(router: Router) {
        let _span = tracing::span!(tracing::Level::DEBUG, "server").entered();
        let host = IpAddr::V4(Ipv4Addr::LOCALHOST);
        let addr = SocketAddr::new(host, 8080);
        let final_router = router.layer(HttpServer::create_cors_layer());
        tracing::info!("Listening at http://{}", addr);
        
        axum::Server::bind(&addr)
            .serve(final_router.into_make_service()).await.unwrap();
    }

    pub async fn start_https(self, router: Router) {
        // TODO Figure this one out.
    }

    fn create_cors_layer() -> CorsLayer {
        // Options, and Content-Type are required for the Cors to work.
        // This is because there is a preflight request sent from the client.
        tracing::info!("Adding cors layer");
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
