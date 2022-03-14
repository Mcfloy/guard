use async_trait::async_trait;
use crate::GuardError;

#[async_trait]
pub trait NamespaceRepository: Send + Sync + 'static {
    async fn create_namespace(&mut self, namespace: &str) -> Result<(), GuardError>;

    async fn get_namespaces(&self) -> Result<Vec<String>, GuardError>;

    async fn does_namespace_exists(&self, namespace: &str) -> Result<bool, GuardError>;
}
