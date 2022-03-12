use async_trait::async_trait;
#[cfg(feature = "with-poem")]
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

use crate::GuardError;

/// Permission from a namespace's perspective.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "with-poem", derive(Object))]
pub struct Permission {
    pub role: String,
    pub domain: String,
    pub object: String,
    pub action: String
}

#[async_trait]
pub trait PermissionRepository: Send + Sync + 'static {
    async fn grant_permission(&mut self, namespace: &str, permission: &Permission) -> Result<(), GuardError>;

    async fn remove_permission(&mut self, namespace: &str, permission: &Permission) -> Result<(), GuardError>;

    async fn contains_permission(&mut self, namespace: &str, permission: &Permission) -> Result<bool, GuardError>;

    async fn list_permissions(&mut self, namespace: &str) -> Result<Vec<Permission>, GuardError>;
}
