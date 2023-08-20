pub mod environment { 

    use std::env;
    use dotenv::dotenv;
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Config {
        _map: HashMap<String, String>,
    }
    
    impl Config {
        pub fn init() -> Config {
            dotenv().ok(); // Read the .env file
            Config { _map: HashMap::new() }
        }

        pub fn frontend(&mut self) -> String {
            const KEY: &str = "FRONTEND";
            const OPTS: [&str; 2] = ["WEB", "LOCAL"];
            const DEFAULT: &str = OPTS[0];
            
            match env::var(KEY) {
                Ok(v) => {
                    if OPTS.contains(&v.as_str()) {
                        return String::from(v)
                    }
                    String::from(DEFAULT) 
                },
                Err(e) => {
                    println!("Error parsing \'FRONTEND\': {:?}", e);
                    println!("Defaulting to {:?}", DEFAULT);
                    String::from(DEFAULT)
                }
            }
        }
    }
}

