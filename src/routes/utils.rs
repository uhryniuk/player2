use serde::{Deserialize, Serialize};
use axum::{
    Router,
    http::Method,
};


pub struct RouterInfo {
    pub router: Router,
    pub paths: Vec<(String, Method)>
}


impl RouterInfo {

    pub fn nest(key: &str, routers: Vec<RouterInfo>) -> RouterInfo {

        let mut router = Router::new();
        let mut paths: Vec<(String, Method)> = Vec::new();

        for r in routers {
            router = router.nest(key, r.router);
            paths.extend(Self::prefix_paths(key, r.paths));
        }
        

        RouterInfo {
            router,
            paths,
        }
    }

    fn prefix_paths(key: &str, paths: Vec<(String, Method)>) -> Vec<(String, Method)> {
        let new_paths: Vec<(String, Method)> = paths.iter().map(|(k,m)| {
            (format!("{}{}", key, k), m.clone())
        }).collect();
        new_paths
    }
}

/// Object container WebSocket Subscription Information.
/// Useful for interacting with certain subscribed services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription<T> {
    key: String,
    data: T 
}


