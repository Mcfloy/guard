use thiserror::Error;

#[derive(Error, Debug)]
pub enum GuardError {
    #[error("Namespace error {0}")]
    NamespaceError(String),
    #[error("Access already exists")]
    AccessAlreadyExists,
    #[error("Cannot remove access")]
    CannotRemoveAccess,
    #[error("Cannot enforce: {0}")]
    CannotEnforce(String)
}
