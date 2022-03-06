use crate::error::GuardError;

pub mod error;
pub mod jwt;
pub mod role;
pub mod namespace;
pub mod permission;

pub type GuardResult<T> = std::result::Result<T, GuardError>;
