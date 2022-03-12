use thiserror::Error;

// TODO: Split into a module Error
#[derive(Error, Debug, Clone)]
pub enum GuardError {
    #[error("Namespace not found: {0}")]
    NamespaceNotFound(String),
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
    PermissionError(String),
    #[error("{0}")]
    RoleError(String),
    #[error("{0}")]
    EnforceError(String)
}
