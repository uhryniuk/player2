extern crate pico_args;


use pico_args::Error;
use std::env;
use dotenv::dotenv;

const DEFAULT: &str = "WEB";

fn main() {
// Create a cli arg parse to decide whether we play local game or start the server.
    // We could start the server anyways????
// Then we need a generic version of Minimax (we can pass whatever struct into it.)
    // Need some seed information like who is making the first move.
    // It will need an interface where it can compare which move is better in relation to others
    // Some form of calling "evaluate" to run a unique version for each game impl.
    // Other than that, it should be done.

// After this we can make an impl of reinforcement learning?
// or some other methodology.
//
//
    dotenv().ok(); // Read the .env file



    let mut args = pico_args::Arguments::from_env();
    
    let _data: Result<Vec<String>, Error>  = args.values_from_str::<&str, String>("--frontend");

    let outcome: String = match _data { 
        Ok(val) => {
            let mut return_val = String::from(DEFAULT);
            if val.len() > 0 {
                println!("Changing return_val assignment");
                return_val = val.get(0).unwrap().to_string();
            }
            return_val
        },
        Err(e) => {
            println!("Error occured parsing \'--frontend\' args: {}", e);
            println!("Resuming with default value: {:?}", DEFAULT);
            String::from(DEFAULT) 
        }
    };

    println!("Resolved Match: {:?}", outcome);


}
