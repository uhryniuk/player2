mod config;
mod games;
mod models;
mod routes;
mod server;

use config::Config;
use server::HttpServer;

use tracing::{span, Level};
use tracing_subscriber::fmt::format::FmtSpan;


#[tokio::main]
async fn main() {

    // Simple Log config. For more precise impl check linkies.
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_span_events(FmtSpan::ACTIVE)
        .init();
    
    let _span = span!(Level::DEBUG, "main").entered();

    tracing::info!("Parsing config");
    let config = Config::init();
    
    tracing::info!("Creating routes");
    let routes = routes::get_routes();
    routes.paths.iter().for_each(|(p, m)| tracing::debug!("-> {} @ {}", m, p));
    
    let server: HttpServer = HttpServer::from(config);
    server.start(routes.router).await;
}
