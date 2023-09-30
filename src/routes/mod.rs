mod connect4;

use axum::{
    Router,
};
use connect4::*;

pub fn get_routes() -> Router {
    Router::new()
        .nest("/connect4", add_connect4_routes())
        // NOTE add more game routes.

}


