use async_trait::async_trait;
use sqlx::Row;
use guard::permission::{Permission, PermissionRepository};
use guard::error::GuardError;
use crate::PostgresRepository;

#[async_trait]
impl PermissionRepository for PostgresRepository {
    async fn grant_permission(&mut self, namespace: &str, permission: &Permission) -> Result<(), GuardError> {
        let query = format!(
            "INSERT INTO namespace_{} VALUES ($1, $2, $3, $4)",
            namespace
        );

        let result = sqlx::query(&query)
            .bind(&permission.role)
            .bind(&permission.domain)
            .bind(&permission.object)
            .bind(&permission.action)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(GuardError::PermissionAlreadyExists)
        }
    }

    async fn remove_permission(&mut self, namespace: &str, permission: &Permission) -> Result<(), GuardError> {
        let query = format!(
            "DELETE FROM namespace_{} WHERE role=$1 AND domain=$2 AND object=$3 AND action=$4;",
            namespace
        );
        let result = sqlx::query(&query)
            .bind(&permission.role)
            .bind(&permission.domain)
            .bind(&permission.object)
            .bind(&permission.action)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(GuardError::CannotRemovePermission)
        }
    }

    async fn contains_permission(&mut self, namespace: &str, permission: &Permission) -> Result<bool, GuardError> {
        let query = format!(
            "SELECT true FROM namespace_{} WHERE role=$1 AND domain=$2 AND object=$3 AND action=$4;",
            namespace
        );

        let result: Result<(bool, ), sqlx::Error> = sqlx::query_as(&query)
            .bind(&permission.role)
            .bind(&permission.domain)
            .bind(&permission.object)
            .bind(&permission.action)
            .fetch_one(&self.pool)
            .await;

        match result {
            Ok(record) => Ok(record.0),
            Err(_) => Err(GuardError::CannotGetPermission)
        }
    }

    async fn list_permissions(&mut self, namespace: &str) -> Result<Vec<Permission>, GuardError> {
        let query = format!("SELECT * FROM namespace_{};", namespace);
        let result = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await;

        if result.is_err() {
            return Err(GuardError::CannotGetPermission);
        }

        let rows = result.unwrap();
        let mut permissions = vec![];

        let parsing_error = GuardError::PermissionError("Cannot parse row.".to_owned());

        for row in rows {
            let role = row.try_get("role")
                .map_err(|_| parsing_error.clone())?;
            let domain = row.try_get("domain")
                .map_err(|_| parsing_error.clone())?;
            let object = row.try_get("object")
                .map_err(|_| parsing_error.clone())?;
            let action = row.try_get("action")
                .map_err(|_| parsing_error.clone())?;

            permissions.push(Permission {
                role,
                domain,
                object,
                action,
            })
        }

        Ok(permissions)
    }
}
