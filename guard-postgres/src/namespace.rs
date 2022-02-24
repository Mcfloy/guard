use async_trait::async_trait;
use std::error::Error;
use guard::error::GuardError;
use guard::namespace::NamespaceRepository;
use crate::PostgresRepository;

#[async_trait]
impl NamespaceRepository for PostgresRepository {
    async fn get_namespaces(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let result = sqlx::query!("SELECT DISTINCT v1 FROM guard WHERE ptype=$1;", "p")
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(records) => Ok(records.iter().map(|r| r.v1.to_string()).collect::<Vec<String>>()),
            Err(_) => Err(Box::new(GuardError::NamespaceError("Cannot get namespaces".to_string())))
        }
    }

    async fn get_namespaces_of_subject(&self, subject: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let result = sqlx::query!(
            "SELECT DISTINCT v1 FROM guard WHERE ptype='p' AND v0=$1;", subject
        )
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(records) => Ok(records.iter().map(|r| r.v1.to_string()).collect::<Vec<String>>()),
            Err(_) => Err(Box::new(GuardError::NamespaceError("Cannot get namespaces".to_string())))
        }
    }
}
