use thiserror::Error;

use super::global::SERVER_ERROR;

pub const USERNAME_ALREADY_EXISTS: &str = "Username already exists";
pub const EMAIL_ALREADY_EXISTS: &str = "Email already exists";
pub const INVALID_EMAIL: &str = "Invalid email";
pub const PASSWORD_HASH_ERROR: &str = "Password hash error";

#[derive(Debug, Error)]
pub enum RegisterError {
    #[error("Username already exists")]
    UsernameAlreadyExists,

    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Invalid email")]
    InvalidEmail,

    #[error("Password hash error")]
    PasswordHashError,

    #[error("Database error")] DatabaseError(#[from] diesel::result::Error),
}

impl RegisterError {
    pub fn message(&self) -> &'static str {
        match self {
            RegisterError::UsernameAlreadyExists => USERNAME_ALREADY_EXISTS,
            RegisterError::EmailAlreadyExists => EMAIL_ALREADY_EXISTS,
            RegisterError::InvalidEmail => INVALID_EMAIL,
            RegisterError::PasswordHashError => PASSWORD_HASH_ERROR,
            RegisterError::DatabaseError(_) => SERVER_ERROR,
        }
    }
}
