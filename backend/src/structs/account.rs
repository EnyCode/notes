use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AccountData {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AccountConfirmation {
    pub user_code: String,
    pub confirmation_code: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserData {
    pub username: String,
    pub email: String,
    pub auth_code: String,
    pub valid_tokens: Vec<String>,
    pub verified: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct VerificationData {
    pub name: String,
    pub auth_code: String,
}
