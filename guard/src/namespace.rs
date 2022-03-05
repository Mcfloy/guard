use async_trait::async_trait;

#[async_trait]
pub trait NamespaceRepository: Send + Sync + 'static {
    async fn get_namespaces(&self) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>>;

    async fn get_namespaces_of_subject(&self, subject: &str) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>>;
}
