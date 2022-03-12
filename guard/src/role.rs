use async_trait::async_trait;
#[cfg(feature = "with-poem")]
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

use crate::GuardError;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "with-poem", derive(Object))]
pub struct Role {
    pub subject: String,
    pub domain: String,
    pub role: String
}

#[async_trait]
pub trait RoleRepository: Send + Sync + 'static {
    async fn assign_role(&mut self, namespace: &str, role: &Role) -> Result<(), GuardError>;

    async fn remove_role(&mut self, namespace: &str, role: &Role) -> Result<(), GuardError>;

    async fn list_roles(&self, namespace: &str, domain: &str, subject: &str) -> Result<Vec<Role>, GuardError>;
}
