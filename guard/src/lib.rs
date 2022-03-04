use crate::error::GuardError;

pub mod access;
pub mod error;
pub mod namespace;
pub mod jwt;

pub type GuardResult<T> = std::result::Result<T, GuardError>;
