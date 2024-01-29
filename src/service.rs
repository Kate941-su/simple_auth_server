use std::fmt::format;
use crate::{
    model::{
        otp::{DisableOTPSchema, GenerateOTPSchema, VerifyOTPSchema},
        user::{User, UserLoginSchema, UserRegisterSchema},
    },
    response::response::{GenericResponse, UserData, UserResponse},
};

use actix_web::{get, post, web, HttpResponse, Responder};
use base32;
use chrono::prelude::*;
use rand::Rng;
use serde_json::json;
use totp_rs::{Algorithm, Secret, TOTP};
use uuid::Uuid;
use crate::model::user::AppState;

fn user_to_response(user: &User) -> UserData {
    UserData {
        id: user.id.to_owned().unwrap(),
        name: user.name.to_owned(),
        email: user.email.to_owned(),
        otp_auth_url: user.otp_auth_url.to_owned(),
        otp_base32: user.otp_base32.to_owned(),
        otp_enabled: user.otp_is_enabled.to_owned().unwrap(),
        otp_verified: user.otp_is_verified.to_owned().unwrap(),
        createdAt: user.createdAt.to_owned().unwrap(),
        updatedAt: user.updatedAt.to_owned().unwrap(),
    }
}

#[post("/auth/register")]
async fn register_user_handler(
    body: web::Json<UserRegisterSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    // TODO: Modify unwrap function when publishing.
    let mut vec = data.db.lock().unwrap();

    for user in vec.iter() {
        if user.email == body.email.to_lowercase() {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("User with email {} already exists", user.email)
            };
            // status code 409
           return HttpResponse::Conflict().json(error_response);
        }
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now();

    let user = User {
        id: Some(uuid_id.to_string()),
        email: body.email.to_owned(),
        name: body.name.to_owned(),
        password: body.password.to_owned(),
        otp_is_enabled: Some(false),
        otp_is_verified: Some(false),
        otp_base32: None,
        otp_auth_url: None,
        createdAt: Some(datetime),
        updatedAt: Some(datetime),
    };

    vec.push(user);

    HttpResponse::Ok().json(json!({"status": "success", "message": "Registered successfully, please login"}))

}