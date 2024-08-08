use actix_web::Responder;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn get_users() -> impl Responder {
    format!("Hello from auth!")
}
