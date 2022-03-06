use thiserror::Error;

#[derive(Error, Debug)]
pub enum GuardError {
    #[error("Namespace error: {0}")]
    NamespaceError(String),
    #[error("Permission already exists")]
    PermissionAlreadyExists,
    #[error("Cannot remove permission")]
    CannotRemovePermission,
    #[error("Cannot get permission")]
    CannotGetPermission,
    #[error("Cannot enforce: {0}")]
    CannotEnforce(String),
    #[error("{0}")]
    PermissionError(String)
}
