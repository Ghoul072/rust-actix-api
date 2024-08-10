use actix_web::{ get, Error, web, HttpResponse, Responder };
use serde::{ Deserialize, Serialize };

use crate::{
    auth,
    database::connection::get_connection,
    utils::auth::get_user_by_username,
    AppContext,
};

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

#[get("/login")]
async fn login(
    req: Result<web::Json<LoginRequest>, Error>,
    data: web::Data<AppContext>
) -> Result<impl Responder, Error> {
    match req {
        Ok(body) => {
            let connection = get_connection(&data.pool);

            match get_user_by_username(&body.username, connection) {
                Ok(user) => {
                    if bcrypt::verify(&body.password, &user.password).is_err() {
                        return Ok(
                            HttpResponse::Unauthorized().body("Invalid username or password")
                        );
                    }

                    let access_token = match auth::jwt::create_token(user.uuid, "access") {
                        Ok(token) => token,
                        Err(err) => {
                            return Err(actix_web::error::ErrorInternalServerError(err));
                        }
                    };

                    let refresh_token = match auth::jwt::create_token(user.uuid, "refresh") {
                        Ok(token) => token,
                        Err(err) => {
                            return Err(actix_web::error::ErrorInternalServerError(err));
                        }
                    };

                    Ok(
                        HttpResponse::Ok().json(LoginResponse {
                            access_token: access_token,
                            refresh_token: refresh_token,
                        })
                    )
                }
                Err(diesel::result::Error::NotFound) => {
                    Ok(HttpResponse::Unauthorized().body("Invalid username or password"))
                }
                Err(_) => { Ok(HttpResponse::InternalServerError().body("Server error")) }
            }
        }
        Err(_) => { Ok(HttpResponse::BadRequest().body("Username or Password missing")) }
    }
}
