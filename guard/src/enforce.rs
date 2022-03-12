use async_trait::async_trait;
#[cfg(feature = "with-poem")]
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

use crate::GuardError;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "with-poem", derive(Object))]
pub struct EnforceRequest {
    pub subject: String,
    pub namespace: String,
    pub domain: String,
    pub object: String,
    pub action: String
}

// TODO: Introduce the notion of Permission Builder for guard
// - Assign role to someone
// - Add namespace scoped role permission

#[async_trait]
pub trait EnforceRepository: Send + Sync + 'static {
    async fn enforce(&self, request: &EnforceRequest) -> Result<bool, GuardError>;
}
