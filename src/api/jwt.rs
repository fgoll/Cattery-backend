use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::Serialize;

#[derive(Serialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    pub fn new(sub: String) -> Self {
        let exp = SystemTime::now() + Duration::from_secs(15 * 24 * 60 * 60);

        let exp = exp.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;

        Self { sub, exp }
    }
}

pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}
