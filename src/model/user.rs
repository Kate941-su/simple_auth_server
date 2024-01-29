use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use actix_web::App;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Option<String>,
    pub email: String, // perhaps, unnecessary
    pub name: String,
    pub password: String,

    pub otp_is_enabled: Option<bool>,
    pub otp_is_verified: Option<bool>,
    pub otp_auth_url: Option<String>,
    pub otp_base32: Option<String>,

    pub createdAt: Option<DateTime<Utc>>,
    pub updatedAt: Option<DateTime<Utc>>,

}

// Store user login information in memory.
pub struct AppState {
    pub db: Arc<Mutex<Vec<User>>>,
}

impl AppState {
    pub fn init() -> AppState {
        AppState {
            db: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UserLoginSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserRegisterSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}