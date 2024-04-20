use axum::Form;
use axum::{response::Html, Json};
use crate::web::router::{RouterInspector, Method::{GET, POST}, get, post};
use crate::db::models::NewUser;

use serde_json::{json, Value};


async fn hello_world() -> Html<&'static str> {

    let html = 
        "<h1>\
            Hello from the auth endpoints!\
        </h1>
        ";

    Html(html)
}

async fn login(Form(user): Form<NewUser>) -> Json<Value> {
    
    let new_user: NewUser = user.try_into().unwrap();
    Json(json!({"hello": new_user.username}))
}

pub fn inspector() -> RouterInspector {
    let router_inspector = RouterInspector::default()
        .add("/", GET, get(hello_world))
        .add("/login", POST, post(login));
    
    router_inspector
}


