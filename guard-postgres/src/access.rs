use std::error::Error;
use async_trait::async_trait;
use guard::access::{Access, AccessRepository};
use guard::error::GuardError;
use crate::PostgresRepository;

#[async_trait]
impl AccessRepository for PostgresRepository {
    async fn enforce(&self, access: &Access) -> Result<bool, Box<dyn Error>> {
        let result = sqlx::query!(
            "SELECT COUNT(*) FROM guard WHERE ptype='p' AND (v0 = $1 OR v0 IN (SELECT DISTINCT v1 FROM guard WHERE ptype='g' AND v0=$1 AND v2=$2 AND (v3=$3 OR v3='*'))) AND v1=$2 AND (v2=$3 OR v2='*') AND v3=$4 AND (v4=$5 OR v4='*');",
            access.subject,
            access.namespace,
            access.domain,
            access.object,
            access.action
        )
            .fetch_one(&self.pool)
            .await;

        match result {
            Ok(record) => Ok(record.count.unwrap_or(0) > 0),
            Err(error) => Err(Box::new(GuardError::CannotEnforce(error.to_string())))
        }
    }

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
