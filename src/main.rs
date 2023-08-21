extern crate pico_args;

mod config;
mod games;
mod algos;

use config::environment::Config;
use games::connect4::Board;
use algos::minimax::{Minimax, Eval};

use axum::{
    routing::{get, post},
    http::StatusCode,
    extract,
    http::Error,
    http::Request,
    http::Method,
    http::Response,
    http::header,
    http::HeaderValue,
    http::HeaderMap,
    response::IntoResponse,
    Json, Router};
use std::net::SocketAddr;
use std::future::Future;

use serde::Serialize;
use serde_json::json;

use tower_http::cors::{Any, CorsLayer, Cors};

//  Notes
//  https://docs.rs/http/latest/http/request/struct.Request.html -> How to de and se reqs.
//  https://docs.rs/axum/latest/axum/response/index.html -> All sorts of goodies for responses.

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        // Options, and Content-Type are required for the Cors to work.
        // This is because there is a preflight request sent from the client.
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE])
        .allow_origin(Any);

    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/api/minimax/connect4/move", post(c4_response))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn c4_response(payload: Json<Board>) -> impl IntoResponse {
    // TODO Eventually, have match where parse game type (minimax, xyz).
    
    let m = Minimax::new();
    m.compare();

    // Note This is just to validate a simple response.    
    let new_slots  = payload.slots.clone()
        .iter_mut()
        .map(|v| *v + 1)
        .collect::<Vec<i32>>();


    (StatusCode::OK, Json(json!({"slots": new_slots})))
}

