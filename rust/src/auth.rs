use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn register_user(user: web::Json<RegisterRequest>) -> impl Responder {
    println!("Registering user: {:?}", user);
    HttpResponse::Created().json(user.0)
}

pub async fn login(user: web::Json<LoginRequest>) -> impl Responder {
    println!("Logging in user: {:?}", user);
    HttpResponse::Ok().json(user.0)
}
