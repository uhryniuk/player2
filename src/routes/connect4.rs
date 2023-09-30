use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
    response::{
        Html,
        Json, 
    }
};

use crate::{games::connect4::GameState, models::minimax::Minimax};


pub async fn hello_world() -> Result<Json<&'static str>, StatusCode> {
    Ok(Json("Hello world!"))
}

pub async fn some_html() -> Html<String>{
    Html("<h1> Howdy partner!</h1>".to_string())
}

async fn get_move(Json(payload): Json<GameState>) -> (StatusCode, Json<GameState>) {
    let return_move = payload.get_move(4, true);
    (StatusCode::OK, Json(GameState {board: return_move, value: None}))
}



pub fn add_connect4_routes() -> Router {
    let mut local_router = Router::new();
    let routes = Vec::from([
        ("/hello", get(hello_world)),
        ("/howdy", get(some_html)),
        ("/make-move", post(get_move)),
    ]);


    for (path, handler) in routes {
        local_router = local_router.route(path, handler);
    }
    local_router
}

