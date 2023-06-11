pub mod account;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum BackendResult {
    Success,
    Error { message: String, error_code: u8 },
}
 