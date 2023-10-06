use std::env;
use std::option::Option;
use std::str::FromStr;
use dotenv::dotenv;
use std::net::Ipv4Addr;


#[derive(Debug, Clone)]
pub struct Config {
    pub env: EnvironmentType,
    pub host: Ipv4Addr, 
    pub port: u16,
    pub tls_cert: Option<String>,
    pub tls_key: Option<String>,
}

impl Config {
    pub fn init() -> Config {
        dotenv().ok(); // Read the .env file
       
        let env = match env::var("ENV") {
            Ok(value) => EnvironmentType::from_str(value.as_str()).unwrap(),
            Err(_) =>  {
                EnvironmentType::PROD
            }
        };

        let host = match env {
            EnvironmentType::PROD => Ipv4Addr::BROADCAST,
            _ => Ipv4Addr::LOCALHOST,
        };

        let default_port: u16 = 9988;
        let port = match env::var("PORT") {
            Ok(port_string) => port_string.parse::<u16>().unwrap_or_else(|_| default_port),
            Err(_) => {
                // TODO add debug level log since port wasn't found.
                default_port
            }
        };

        Config { 
            env,
            host,
            port,
            tls_cert: None,
            tls_key: None
        }
    }
}

#[derive(Debug, Clone)]
pub enum EnvironmentType {
    PROD,
    DEV,
    LOCAL,
}

impl FromStr for EnvironmentType {
    type Err = EnvironmentType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PROD" => Ok(EnvironmentType::PROD),
            "DEV" => Ok(EnvironmentType::DEV),
            "LOCAL" => Ok(EnvironmentType::LOCAL),
            _ => Err(EnvironmentType::LOCAL),
        }
    }
}
