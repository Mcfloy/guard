use async_trait::async_trait;
use guard::permission::{Permission, PermissionRepository};
use guard::error::GuardError;
use crate::PostgresRepository;

#[async_trait]
impl PermissionRepository for PostgresRepository {
    async fn enforce(&self, access: &Permission) -> Result<bool, GuardError> {
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
            Err(error) => Err(GuardError::CannotEnforce(error.to_string()))
        }
    }

    async fn grant_permission(&mut self, permission: &Permission) -> Result<(), GuardError> {
        let result = sqlx::query!(
            "INSERT INTO guard (ptype, v0, v1, v2, v3, v4) VALUES ($1, $2, $3, $4, $5, $6);",
            "p",
            permission.subject,
            permission.namespace,
            permission.domain,
            permission.object,
            permission.action
        )
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(GuardError::PermissionAlreadyExists)
        }
    }

    async fn remove_permission(&mut self, access: &Permission) -> Result<(), GuardError> {
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
            Err(_) => Err(GuardError::CannotRemoveAccess)
        }
    }

    async fn contains_permission(&mut self, permission: &Permission) -> Result<bool, GuardError> {
        let result = sqlx::query!(
            "SELECT COUNT(1) FROM guard WHERE ptype='p' AND v0=$1 AND v1=$2 AND v2=$3 AND v3=$4 AND v4=$5",
            permission.subject,
            permission.namespace,
            permission.domain,
            permission.object,
            permission.action
        )
            .fetch_one(&self.pool)
            .await;

        match result {
            Ok(record) => Ok(record.count.unwrap_or(0) > 0),
            Err(_) => Err(GuardError::CannotGetAccess)
        }
    }

    async fn list_permissions_from_namespace(&mut self, namespace: &str) -> Result<Vec<Permission>, GuardError> {
        let result = sqlx::query!(
            "SELECT * FROM guard WHERE ptype='p' AND v1=$1",
            namespace
        )
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(records) => Ok(
                records.iter()
                    .map(move |record| {
                        let action = String::from(record.v4.as_ref().unwrap());
                        Permission {
                            subject: record.v0.clone(),
                            namespace: record.v1.clone(),
                            domain: record.v2.clone(),
                            object: record.v3.clone(),
                            action
                        }
                    })
                    .collect()
            ),
            Err(_) => Err(GuardError::CannotGetAccess)
        }
    }
}
