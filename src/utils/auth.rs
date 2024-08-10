use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
use r2d2::PooledConnection;
use ::uuid::Uuid;

use crate::database::models::user::User;
use crate::database::schema::users::dsl::*;
use crate::errors::login::LoginError;

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
