
use serde::{Deserialize, Serialize};


// May need a dumbie user for those who are playing without an account.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    id: i32, // Maybe a hash or something?
    username: String,
    password: String // the hashed version?
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Board {}





