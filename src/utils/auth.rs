use std::env;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
use r2d2::PooledConnection;
use regex::Regex;
use ::uuid::Uuid;

use crate::database::models::user::{ CreatedUser, NewUser, User };
use crate::database::schema::users::dsl::*;
use crate::errors::login::LoginError;
use crate::errors::register::RegisterError;

fn is_valid_email(email_val: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email_val)
}

pub fn get_user_id_from_uuid(
    uuid_val: Uuid,
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>
) -> i32 {
    let result: User = users
        .filter(uuid.eq(uuid_val))
        .select(User::as_select())
        .first(connection)
        .expect(&format!("Error finding user with UUID: {}", uuid_val));

    return result.id;
}

pub fn get_user_by_username(
    username_val: &str,
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>
) -> Result<User, LoginError> {
    let result = users
        .filter(username.eq(username_val))
        .select(User::as_select())
        .first(connection)
        .map_err(|e| {
            match e {
                Error::NotFound => LoginError::InvalidCredentials,
                _ => LoginError::DatabaseError(e),
            }
        });

    return result;
}

pub fn get_user_by_email(
    email_val: &str,
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>
) -> Result<User, Error> {
    return users.filter(email.eq(email_val)).select(User::as_select()).first(connection);
}

pub fn create_user(
    username_val: &str,
    email_val: &str,
    password_val: &str,
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>
) -> Result<CreatedUser, RegisterError> {
    let cost = env::var("BCRYPT_COST").expect("BCRYPT_COST must be set").parse().unwrap();
    let hashed_password = bcrypt
        ::hash(password_val, cost)
        .map_err(|_| RegisterError::PasswordHashError)?;

    if get_user_by_username(username_val, connection).is_ok() {
        return Err(RegisterError::UsernameAlreadyExists);
    }

    if get_user_by_email(email_val, connection).is_ok() {
        return Err(RegisterError::EmailAlreadyExists);
    }

    if !is_valid_email(email_val) {
        return Err(RegisterError::InvalidEmail);
    }

    let new_user = NewUser {
        username: username_val.to_string(),
        email: email_val.to_string(),
        password: hashed_password,
    };

    return diesel
        ::insert_into(users)
        .values(&new_user)
        .get_result(connection)
        .map(|user: User| CreatedUser {
            username: user.username,
            email: user.email,
        })
        .map_err(RegisterError::DatabaseError);
}
