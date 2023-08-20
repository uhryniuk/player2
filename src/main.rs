extern crate pico_args;

mod config;
mod games;

use config::environment::Config;
use games::connect4::Board;

use axum::{
    routing::{get, post},
    http::StatusCode,
    extract,
    http::Request,
    http::Response,
    response::IntoResponse,
    Json, Router};
use serde::Deserialize;
use serde_json::from_str;
use std::net::SocketAddr;

//  Notes
//  https://docs.rs/http/latest/http/request/struct.Request.html -> How to de and se reqs.

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/api/minimax/connect4/move", post(handle_post));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_post(extract::Json(payload): extract::Json<Board>) -> Json<Board> {
    println!("Data Received: {:?}", payload);
    
    Json(payload)
}


use serde::de;

fn deserialize<T>(req: Request<Vec<u8>>) -> serde_json::Result<Request<T>> where for<'de> T: de::Deserialize<'de>,
{
    let (parts, body) = req.into_parts();
    let body = serde_json::from_slice(&body)?;
    Ok(Request::from_parts(parts, body))
}
