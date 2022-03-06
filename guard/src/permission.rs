use async_trait::async_trait;
#[cfg(feature = "poem")]
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

use crate::GuardError;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "poem", derive(Object))]
pub struct Permission {
    pub subject: String,
    pub namespace: String,
    pub domain: String,
    pub object: String,
    pub action: String
}

// TODO: Introduce the notion of Permission Builder for guard
// - Edit Permission
// - Edit Role ?
// - Is Owner of namespace ?

#[async_trait]
pub trait PermissionRepository: Send + Sync + 'static {
    async fn enforce(&self, permission: &Permission) -> Result<bool, GuardError>;

    async fn grant_permission(&mut self, permission: &Permission) -> Result<(), GuardError>;

    async fn remove_permission(&mut self, permission: &Permission) -> Result<(), GuardError>;

    async fn contains_permission(&mut self, permission: &Permission) -> Result<bool, GuardError>;

    async fn list_permissions_from_namespace(&mut self, namespace: &str) -> Result<Vec<Permission>, GuardError>;
}
