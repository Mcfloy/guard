use std::error::Error;
use async_trait::async_trait;
use guard::access::{Access, AccessRepository};
use guard::error::GuardError;
use crate::PostgresRepository;

#[async_trait]
impl AccessRepository for PostgresRepository {
    async fn authorize_access(&mut self, access: &Access) -> Result<(), Box<dyn Error>> {
        let result = sqlx::query!(
            "INSERT INTO guard (ptype, v0, v1, v2, v3, v4) VALUES ($1, $2, $3, $4, $5, $6);",
            "p",
            access.subject,
            access.namespace,
            access.domain,
            access.object,
            access.action
        )
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(Box::new(GuardError::AccessAlreadyExists))
        }
    }

    async fn remove_access(&mut self, access: &Access) -> Result<(), Box<dyn Error>> {
        let result = sqlx::query!(
            "DELETE FROM guard WHERE ptype=$1 AND v0=$2 AND v1=$3 AND v2=$4 AND v3=$5 AND v4=$6;",
            "p",
            access.subject,
            access.namespace,
            access.domain,
            access.object,
            access.action
        )
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(Box::new(GuardError::CannotRemoveAccess))
        }
    }
}
