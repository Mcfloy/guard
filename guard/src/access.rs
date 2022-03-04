use async_trait::async_trait;

#[derive(Debug)]
pub struct Access {
    pub subject: String,
    pub namespace: String,
    pub domain: String,
    pub object: String,
    pub action: String
}

#[async_trait]
pub trait AccessRepository: Send + Sync + 'static {
    async fn enforce(&self, access: &Access) -> Result<bool, Box<dyn std::error::Error>>;

    async fn authorize_access(&mut self, access: &Access) -> Result<(), Box<dyn std::error::Error>>;

    async fn remove_access(&mut self, access: &Access) -> Result<(), Box<dyn std::error::Error>>;
}
