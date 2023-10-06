use std::collections::HashMap;

use axum::{
    Router,
    http::{
        StatusCode,
        Request,
        Response,
        header::SET_COOKIE,
        Method,
    },
    routing::{get, post},
    response::{
        Html,
        Json, IntoResponse, 
    },
    body::Body, extract::{Path, Query}
};

use tracing;
use crate::{
    games::connect4::GameState, 
    models::minimax::{Minimax}
};

use serde_json;

pub async fn raw(Path((one, two)): Path<(String, String)>, queries: Query<HashMap<String, String>>) -> (StatusCode, Json<serde_json::Value>){
    println!("q params {:?}", queries);
    (StatusCode::OK, Json(serde_json::json!({"first": one, "second": two})))
}


async fn get_move(payload: Json<GameState>) -> (StatusCode, Json<GameState>) {
    let return_move = payload.get_move(31, true);
    (StatusCode::OK, Json(GameState {board: return_move, value: None}))
}


pub fn add_connect4_routes() -> Router {
    let mut local_router = Router::new();
    let routes = Vec::from([
        ("/make-move", post(get_move), Method::POST),
        ("/raw/:burger/:bother", get(raw), Method::GET),
    ]);

    for (path, handler, method) in routes {
        tracing::info!("Adding {method} @ {path}");
        local_router = local_router.route(path, handler);
    }
    local_router
}

