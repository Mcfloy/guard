use async_trait::async_trait;

pub struct Access {
    pub subject: String,
    pub namespace: String,
    pub domain: String,
    pub object: String,
    pub action: String
}

impl Access {
    pub fn to_parameters(&self) -> (String, String, String, String, String) {
        (
            self.subject.clone(),
            self.namespace.clone(),
            self.domain.clone(),
            self.object.clone(),
            self.action.clone()
        )
    }
}

#[async_trait]
pub trait AccessRepository: Send + Sync + 'static {
    async fn authorize_access(&mut self, access: &Access) -> Result<(), Box<dyn std::error::Error>>;

    async fn remove_access(&mut self, access: &Access) -> Result<(), Box<dyn std::error::Error>>;
}
