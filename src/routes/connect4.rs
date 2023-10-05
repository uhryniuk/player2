use axum::{
    Router,
    http::{
        StatusCode,
        Request,
        Response,
        header::SET_COOKIE,
        Method
    },
    routing::{get, post},
    response::{
        Html,
        Json, 
    },
};

use tracing;
use crate::{
    games::connect4::GameState, 
    models::minimax::Minimax
};


async fn get_move(payload: Json<GameState>) -> (StatusCode, Json<GameState>) {
    println!("{:?}", payload);
    let return_move = payload.get_move(5, true);
    (StatusCode::OK, Json(GameState {board: return_move, value: None}))
}


pub fn add_connect4_routes() -> Router {
    let mut local_router = Router::new();
    let routes = Vec::from([
        ("/make-move", post(get_move), Method::POST)
    ]);

    for (path, handler, method) in routes {
        tracing::info!("Adding {method} @ {path}");
        local_router = local_router.route(path, handler);
    }
    local_router
}

