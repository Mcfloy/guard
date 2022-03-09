use std::error::Error;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
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

pub fn encode(principal: &Principal) -> Result<String, Box<dyn Error>> {
    match jsonwebtoken::encode::<Principal>(
        &Header::default(),
        principal,
        &EncodingKey::from_secret(SECRET_KEY.as_ref())) {
        Ok(token) => Ok(token),
        Err(error) => Err(Box::new(error))
    }
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
