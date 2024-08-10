use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
use diesel::{ ExpressionMethods, QueryDsl };
use r2d2::PooledConnection;
use ::uuid::Uuid;

use crate::database::models::user::User;
use crate::database::schema::users::dsl::*;

pub fn get_user_id_from_uuid(
    user_uuid: Uuid,
    mut connection: PooledConnection<ConnectionManager<PgConnection>>
) -> i32 {
    let result: User = users
        .filter(uuid.eq(user_uuid))
        .select(User::as_select())
        .first(&mut connection)
        .expect(&format!("Error finding user with UUID: {}", user_uuid));

    return result.id;
}

pub fn get_user_by_username(
    user_name: &str,
    mut connection: PooledConnection<ConnectionManager<PgConnection>>
) -> Result<User, Error> {
    return users.filter(username.eq(user_name)).select(User::as_select()).first(&mut connection);
}
