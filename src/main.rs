mod config;
mod web;

use web::{
    auth,
    connect4,
    server::HttpServer,
    router::RouterInspector
};

use mongodb::{bson, Client};

use config::{ENV_TYPE, var, SINGLETON};

use tracing::{span, Level};
use tracing_subscriber::fmt::format::FmtSpan;


#[tokio::main]
async fn main() {
    // NOTE Sets logger, but consider making soem conditional for 'with_max_level'.
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_span_events(FmtSpan::ACTIVE)
        .init();
    
    tracing::info!("🏁 Initializing application");
    //let _span = span!(Level::DEBUG, "main").entered();
    
    // TODO Connect to database.
    
    tracing::info!("🧱 Building routers");
    let ri = RouterInspector::default()
        .nest("/api/v1", vec![
            auth::inspector(),
            connect4::inspector(),
        ]);

    HttpServer::start_http(ri.routes()).await;
}
