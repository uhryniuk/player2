use axum::{response::Html, Json};
use crate::web::router::{RouterInspector, Method::GET, get};


async fn hello_world() -> Html<&'static str> {

    let html = 
        "<h1>\
            Hello from the auth endpoints!\
        </h1>
        ";

    Html(html)
}

async fn other() -> Html<&'static str> {
    let html = 
        "<h1>\
            YEAAAAH BOOOOI!\
        </h1>
        ";
    Html(html)
}

async fn login() -> Json<String> {
    Json(String::from("Hello world"))
}

pub fn inspector() -> RouterInspector {
    let router_inspector = RouterInspector::default()
        .add("/", GET, get(hello_world))
        .add("/other", GET, get(other))
        .add("/login", GET, get(login));
    
    router_inspector
}


