use std::collections::HashMap;

use axum::{
    body::Body,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::{Path, Query},
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};

use crate::{games::connect4::GameState, models::minimax::Minimax};
use tracing::{self, info};

use serde_json::{self, json};

use super::utils::{self, RouterInfo};

const conn_pool: i8 = 13;
// Create Arc HashMap to send data between functions?

pub async fn raw(
    Path((one, two)): Path<(String, String)>,
    queries: Query<HashMap<String, String>>,
) -> (StatusCode, Json<serde_json::Value>) {
    println!("q params {:?}", queries);
    (
        StatusCode::OK,
        Json(serde_json::json!({"first": one, "second": two})),
    )
}

async fn get_move(payload: Json<GameState>) -> (StatusCode, Json<GameState>) {
    let return_move = payload.get_move(31, true);
    (
        StatusCode::OK,
        Json(GameState {
            board: return_move,
            value: None,
        }),
    )
}

async fn handler(queries: Query<HashMap<String, String>>, ws: WebSocketUpgrade) -> impl IntoResponse {
    // NOTE use queries to do the auto matching games
    // NOTE use path params to specify connecting via username.
    println!("{:?}", queries);
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            println!("{:?}", Json::from(msg));
            Message::Text(json!({"board": GameState::blank_board(), "value": 123}).to_string())
        } else {
            // client disconnected
            println!("Disconnect:  {:?}", 123);
            return;
        };

        if socket.send(msg).await.is_err() {
            // client disconnected
            //tracing::info!("Disconnect:  {:?}", 123);
            return;
        }
    }
}

pub fn get_routes() -> RouterInfo {
    let mut router = Router::new();
    let mut paths: Vec<(String, Method)> = Vec::new();

    let routes = Vec::from([
        ("/make-move", post(get_move), Method::POST),
        ("/raw/:burger/:bother", get(raw), Method::GET),
        ("/ws", get(handler), Method::GET),
    ]);

    for (path, handler, method) in routes {
        paths.push((path.to_owned(), method));
        router = router.route(path, handler);
    }
    
    RouterInfo {
        router,
        paths,
    }
}
