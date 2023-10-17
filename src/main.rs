mod config;
mod server;
mod models;
mod games;
mod routes;

use axum::Router;
use models::minimax::{Minimax};
use server::HttpServer;
use config::Config;

use crate::games::connect4::GameState;


#[tokio::main]
async fn main() {
    let config = Config::init();
    let server: HttpServer = HttpServer::from(config.clone());
    let router = Router::new()
        .nest("/api", routes::get_routes());
    server.start(router).await;
    
}

