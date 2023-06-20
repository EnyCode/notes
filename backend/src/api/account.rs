use crate::api::database::NotesDB;
use crate::structs::BackendResult;
use crate::structs::account::*;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use mongodb::bson::doc;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use bcrypt::{hash, verify};
use std::env;

#[post(
    "/account/create",
    format = "application/json",
    data = "<account_data>"
)]
pub async fn create_account(
    connection: Connection<NotesDB>,
    account_data: Json<AccountData>,
) -> Json<BackendResult> {
    // Set up database
    let db = connection.into_inner().database("notes");
    let collection = db.collection::<UserData>("userData");

    // Check if username is taken
    let filter = doc! { "username": account_data.name.to_owned() };
    match collection.find_one(filter, None).await {
        Ok(result) =>  {
            if result.is_some() {
                return Json(BackendResult::Error {
                    message: "Username already exists.".to_owned(),
                    error_code: 1,
                });
            }
        }
        Err(_) => {
            return Json(BackendResult::Error {
                message: "Unable to contact database.".to_owned(),
                error_code: 2,
            });
        }
    }

    // Generate a confirmation code and hash it
    let confirmation_code = format!("{:x}", rand::random::<i32>());
    let hashed = hash(confirmation_code.as_bytes(), 8);

    // Set up an email and credentials
    let email = Message::builder()
        .from(env::var("EMAIL_ADDRESS").unwrap().parse().unwrap())
        .to(account_data.email.parse().unwrap())
        .subject("Register for Notes")
        .header(ContentType::TEXT_HTML)
        .body(format!("Hello, your email has been registered for <a href=\"{}\">notes</a>. If this was you, enter the following code: {}.  If not, ignore this email. ", env::var("NOTES_URL").unwrap(), confirmation_code))
        .unwrap();

    let creds = Credentials::new(
        env::var("EMAIL_USERNAME").unwrap(),
        env::var("EMAIL_PASSWORD").unwrap(),
    );

    // Get the mailer and send the email
    let mailer = SmtpTransport::relay(&(env::var("SMTP_SERVER").unwrap()))
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Err(e) => panic!("Could not send email: {:?}", e),
        _ => (),
    }

    // Update the database with the new user.
    collection.insert_one(
        UserData {
            username: account_data.name.to_owned(),
            email: account_data.email.to_owned(),
            auth_code: hashed.expect("Failed to encrypt the auth code. "),
            valid_tokens: vec![],
            verified: false
        },
        None
    ).await.expect("Failed to insert new user profile");

    Json(BackendResult::Success)
}

#[post(
    "/account/verify",
    format = "application/json",
    data = "<verification>"
)]
pub async fn verify_account(
    connection: Connection<NotesDB>,
    verification: Json<VerificationData>,
) -> Json<BackendResult> {
    // Set up database
    let db = connection.into_inner().database("notes");
    let collection = db.collection::<UserData>("userData");

    // Get document and check if suername exists
    let filter = doc! { "username": verification.name.to_owned() };
    match collection.find_one(filter.clone(), None).await {
        Ok(result) =>  {
            if result.is_some() {
                let data = result.unwrap();
                // Check if the account is already verified
                if data.verified {
                    return Json(BackendResult::Error {
                        message: "This account has already been verified.".to_owned(),
                        error_code: 4,
                    });
                }
                // If the code passes the check, update the document
                if verify(&verification.auth_code.as_bytes(), &data.auth_code).unwrap() {
                    let update = collection.update_one(filter, doc! { "$set": {"verified": true, "auth_code": "" } }, None).await;
                    if update.is_err() {
                        println!("{}", update.err().unwrap().to_string());
                        return Json(BackendResult::Error {
                            message: "A database error occured.".to_owned(),
                            error_code: 3,
                        });
                    }
                } else {
                    // If it fails, error
                    return Json(BackendResult::Error {
                        message: "Unauthenticated".to_owned(),
                        error_code: 5,
                    });
                }
            }
        }
        Err(_) => {
            return Json(BackendResult::Error {
                message: "Unable to contact database.".to_owned(),
                error_code: 2,
            });
        }
    }

    Json(BackendResult::Success)
}
