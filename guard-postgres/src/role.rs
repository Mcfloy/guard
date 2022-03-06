use async_trait::async_trait;
use guard::error::GuardError;
use guard::role::{Role, RoleRepository};
use crate::PostgresRepository;

#[async_trait]
impl RoleRepository for PostgresRepository {
    async fn add_role(&self, role: &Role) -> Result<(), GuardError> {
        let result = sqlx::query!(
            "INSERT INTO guard (ptype, v0, v1, v2, v3) VALUES ('g', $1, $2, $3, $4)",
            role.subject,
            role.name,
            role.namespace,
            role.domain
        )
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(GuardError::RoleError("Cannot add role".to_string()))
        }
    }

    async fn remove_role(&self, role: &Role) -> Result<(), GuardError> {
        let result = sqlx::query!(
            "DELETE FROM guard WHERE ptype='g' AND v0=$1 AND v1=$2 AND v2=$3 AND v3=$4",
            role.subject,
            role.name,
            role.namespace,
            role.domain
        )
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(GuardError::RoleError("Cannot remove role".to_string()))
        }
    }

    async fn list_roles(&self, subject: Option<String>) -> Result<Vec<Role>, GuardError> {
        let result = sqlx::query!(
            "SELECT * FROM guard WHERE ptype='g' AND v0 = $1",
            subject
        )
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(records) => Ok(
                records.iter()
                    .map(move |record| {
                        Role {
                            subject: record.v0.clone(),
                            name: record.v1.clone(),
                            namespace: record.v2.clone(),
                            domain: record.v3.clone()
                        }
                    })
                    .collect()
            ),
            Err(_) => Err(GuardError::RoleError("Cannot get roles".to_string()))
        }
    }
}
