use axum::{
    Router,
    routing::MethodRouter
};

pub use axum::routing::{get, post, put, delete};

#[derive(Debug, Clone)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE
}

#[derive(Debug, Clone)]
struct RouteLog {
    method: Method,
    pub path: String
}

impl RouteLog {
    pub fn log(self) -> String {
        let method  = match self.method {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
        };
    
        format!("{} @ {}", method, self.path)
    }

    pub fn add_prefix(&mut self, prefix: &str) {
        self.path = prefix.to_owned() + self.path.as_str();
    }
}


/// Container for router to track all registered routes.
#[derive(Debug, Clone, Default)]
pub struct RouterInspector {
    router: Router,
    route_logs: Vec<RouteLog> 
}

impl RouterInspector {
    
    pub fn add(mut self, path: &str, method: Method, handler: MethodRouter) -> Self {
        self.route_logs.push(RouteLog {
            method,
            path: path.to_owned(),
        });
        self.router = self.router.clone().merge(axum::Router::new().route(path, handler));
        self
    }

    pub fn nest(mut self, path: &str, routers: Vec<RouterInspector>) -> Self {
        for router in routers.into_iter() {
            // Merge the routers directly into the existing router
            self.router = self.router.nest(path, router.router);
            
            // Extend the route logs with the nested route logs
            self.route_logs.extend(router.route_logs);
            self.route_logs.iter_mut().for_each(|rl| rl.add_prefix(path.clone()));
        }

        self
    }

    pub fn routes(self) -> Router {
        self.route_logs.iter().for_each(|rl| {
            let clone = rl.clone();
            tracing::info!("➕ {}", clone.log());
        });
        self.router
    }
}
