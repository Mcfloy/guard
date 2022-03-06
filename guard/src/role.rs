use async_trait::async_trait;
#[cfg(feature = "poem")]
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use crate::GuardError;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "poem", derive(Object))]
pub struct Role {
    pub subject: String,
    pub name: String,
    pub namespace: String,
    pub domain: String
}

#[async_trait]
pub trait RoleRepository: Send + Sync + 'static {
    async fn add_role(&self, role: &Role) -> Result<(), GuardError>;

    async fn remove_role(&self, role: &Role) -> Result<(), GuardError>;

    async fn list_roles(&self, subject: Option<String>) -> Result<Vec<Role>, GuardError>;
}
