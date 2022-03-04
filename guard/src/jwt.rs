use std::error::Error;

use jsonwebtoken::{DecodingKey, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

const GUARD_PRIVATE_KEY: &str = "GUARD_PRIVATE_KEY";

lazy_static! {
    static ref SECRET_KEY: String = std::env::var(GUARD_PRIVATE_KEY).unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Principal {
    pub sub: String,
    pub namespace: String,
    pub exp: u64
}

pub fn decode(token: &str) -> Result<Principal, Box<dyn Error>> {
    match jsonwebtoken::decode::<Principal>(
        token,
        &DecodingKey::from_secret(SECRET_KEY.as_ref()),
        &Validation::default()
    ) {
        Ok(token_data) => Ok(token_data.claims),
        Err(error) => Err(Box::new(error))
    }
}
