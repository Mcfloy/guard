use thiserror::Error;

#[derive(Error, Debug)]
pub enum GuardError {
    #[error("Namespace error {0}")]
    NamespaceError(String),
    #[error("Access already exists")]
    PermissionAlreadyExists,
    #[error("Cannot remove access")]
    CannotRemoveAccess,
    #[error("Cannot get access")]
    CannotGetAccess,
    #[error("Cannot enforce: {0}")]
    CannotEnforce(String)
}
