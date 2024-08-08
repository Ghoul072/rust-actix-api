use std::env;
use diesel::prelude::*;
use diesel::{ ExpressionMethods, QueryDsl };
use jsonwebtoken::{ decode, encode, errors::Result, DecodingKey, EncodingKey, Header, Validation };
use serde::{ Deserialize, Serialize };
use ::uuid::Uuid;

use crate::create_connection;
use crate::database::models::user::User;
use crate::database::schema::users::dsl::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_uuid: Uuid,
    pub exp: usize,
}

pub fn create_token(user_uuid: Uuid) -> Result<String> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let expiration = chrono::Utc
        ::now()
        .checked_add_signed(chrono::Duration::seconds(3600))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        user_uuid: user_uuid.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref()))
}

pub fn verify_auth_token(token: &str) -> Result<i32> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default()
    )?;

    let mut connection: PgConnection = create_connection();
    let result: Vec<User> = users
        .filter(uuid.eq(token.claims.user_uuid))
        .limit(1)
        .select(User::as_select())
        .load(&mut connection)
        .expect("Error loading posts");

    return Ok(result[0].id);
}
