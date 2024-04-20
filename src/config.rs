use std::collections::HashMap;
use std::env;
use std::option::Option;
use std::sync::Mutex;
use once_cell::sync::Lazy;


type SingletonHashMap = HashMap<&'static str, &'static str>;
type EnvVar = &'static str;

/// This is a Lazy<Mutex<Map
///
/// Can be hard to access data inside of Lazy.
/// Therefore it's easy to wrap it in a mutex and access it.
/// This is okay with simple referencing, but anything async would go kablooie 
pub static SINGLETON: Lazy<Mutex<SingletonHashMap>> = Lazy::new({|| {
    let mut map = HashMap::new();
    map.insert("Blah", "LBoop");
    Mutex::new(map) 
}});

pub const PORT: EnvVar = "PORT";
pub const ENV_TYPE: EnvVar = "ENV";
pub const DB_PORT: EnvVar = "DB_PORT";
pub const DB_ADDR: EnvVar = "DB_ADDR";

/// Helper to parse env file and get a juicy option back.
///  
/// NOTE Consider looking at lazy loading these 
pub fn var(key: EnvVar) -> Option<String> {
    dotenv::dotenv().ok();
    return env::var(key).ok()
}
    
