/*
 *  Todo list:
 *  1. Create the generic Board
 *  2. Create some impls for working with the Board
 *  3. Research the To and From impls more for structs.
 *      - Need to parse from 2D arr into our struct
 *  4. Implement to the eval trait to compare 2 boards together
 *      - Potentially N boards too? That could simplify a lot.
 *
 *
 *
 */
use std::result::Result::{Ok, Err};
use serde::Deserialize;


// Specific impl for minimax.
// NOTE: May not be neede for some super genric stuff.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Board {
    pub slots: Vec<i32>,
}


impl Board {
    pub fn new() -> Board {
        Board { slots: [0; 42].to_vec() }
    }

    // incorrect, this should parse the string from the request.
    pub fn from(slots: &Vec<i32>) -> Result<Board, String> {
        match Self::validate_board(slots.to_vec()) {
            true => Ok(Board { slots: slots.to_vec() }),
            false => Err("Cannot create board from provided slots.".to_string())
        }
    }

    fn validate_board(slots: Vec<i32>) -> bool {
        true  // TODO need to finish this implementation.
    }

    pub fn get_slots(self) -> Vec<i32> {
        self.slots.clone()
    }

    pub fn pprint(self) {
        for i in 0..(self.slots.len()) {
            if i % 7 == 0&& i != 0 {
                println!("");
            }
            print!(" {:?} ", self.slots[i]);
        };

        println!("");  // Ensure new line at end of pretty print.
    }
}
