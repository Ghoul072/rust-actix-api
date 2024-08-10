use thiserror::Error;

use super::global::SERVER_ERROR;

pub const INVALID_USERNAME_OR_PASSWORD: &str = "Invalid username or password";
pub const TOKEN_CREATION_ERROR: &str = "Error creating token";

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Token creation error")]
    TokenCreationError,

    #[error("Database error")] DatabaseError(#[from] diesel::result::Error),
}

impl LoginError {
    pub fn message(&self) -> &'static str {
        match self {
            LoginError::InvalidCredentials => INVALID_USERNAME_OR_PASSWORD,
            LoginError::TokenCreationError => TOKEN_CREATION_ERROR,
            LoginError::DatabaseError(_) => SERVER_ERROR,
        }
    }
}
