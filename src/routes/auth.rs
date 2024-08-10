use actix_web::{ get, post, web, Error, HttpResponse, Responder };
use serde::{ Deserialize, Serialize };

use crate::{
    auth,
    database::connection::get_connection,
    errors::login::INVALID_USERNAME_OR_PASSWORD,
    utils::{ self, auth::get_user_by_username },
    AppContext,
};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[post("/register")]
async fn register(
    req: Result<web::Json<RegisterRequest>, Error>,
    data: web::Data<AppContext>
) -> Result<impl Responder, Error> {
    match req {
        Ok(body) => {
            let mut connection = get_connection(&data.pool);

            match
                utils::auth::create_user(
                    &body.username,
                    &body.email,
                    &body.password,
                    &mut connection
                )
            {
                Ok(user) => { Ok(HttpResponse::Created().json(user)) }
                Err(err) => { Ok(HttpResponse::Conflict().body(err.message())) }
            }
        }
        Err(_) => { Ok(HttpResponse::BadRequest().body("Invalid payload")) }
    }
}

#[get("/login")]
async fn login(
    req: Result<web::Json<LoginRequest>, Error>,
    data: web::Data<AppContext>
) -> Result<impl Responder, Error> {
    match req {
        Ok(body) => {
            let mut connection = get_connection(&data.pool);

            match get_user_by_username(&body.username, &mut connection) {
                Ok(user) => {
                    if bcrypt::verify(&body.password, &user.password).is_err() {
                        return Ok(HttpResponse::Unauthorized().body(INVALID_USERNAME_OR_PASSWORD));
                    }

                    let access_token = match auth::jwt::create_token(user.uuid, "access") {
                        Ok(token) => token,
                        Err(err) => {
                            return Ok(HttpResponse::InternalServerError().body(err.message()));
                        }
                    };

                    let refresh_token = match auth::jwt::create_token(user.uuid, "refresh") {
                        Ok(token) => token,
                        Err(err) => {
                            return Ok(HttpResponse::InternalServerError().body(err.message()));
                        }
                    };

                    Ok(
                        HttpResponse::Ok().json(LoginResponse {
                            access_token: access_token,
                            refresh_token: refresh_token,
                        })
                    )
                }
                Err(err) => { Ok(HttpResponse::Unauthorized().body(err.message())) }
            }
        }
        Err(_) => { Ok(HttpResponse::BadRequest().body("Username or Password missing")) }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(register);
}
