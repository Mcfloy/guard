use async_trait::async_trait;
use sqlx::{Error, Row};
use sqlx::postgres::{PgQueryResult};

use guard::error::GuardError;
use guard::role::{Role, RoleRepository};

use crate::PostgresRepository;

#[async_trait]
impl RoleRepository for PostgresRepository {
    async fn assign_role(&mut self, namespace: &str, role: &Role) -> Result<(), GuardError> {
        // TODO: Check namespace existence first ?
        let role_table_name = format!("role_{}", namespace);
        let query = format!(
            "INSERT INTO {} VALUES ($1, $2, $3)",
            role_table_name
        );
        let result = self.execute_role_query(&role, &query).await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(GuardError::RoleError("Cannot add role".to_string()))
        }
    }

    async fn remove_role(&mut self, namespace: &str, role: &Role) -> Result<(), GuardError> {
        let role_table_name = format!("role_{}", namespace);
        let query = format!(
            "DELETE FROM {} WHERE subject=$1 AND domain=$2 AND role=$3",
            role_table_name
        );
        let result = self.execute_role_query(&role, &query).await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(GuardError::RoleError("Cannot remove role".to_string()))
        }
    }

    async fn list_roles(&self, namespace: &str, domain: &str, subject: &str) -> Result<Vec<Role>, GuardError> {
        let query = format!(
            "SELECT * FROM role_{} WHERE subject=$1 AND domain=$2",
            namespace
        );
        let result = sqlx::query(&query)
            .bind(&subject)
            .bind(&domain)
            .fetch_all(&self.pool)
            .await;

        if result.is_err() {
            return Err(GuardError::RoleError("Cannot get roles".to_string()));
        }

        let rows = result.unwrap();
        let mut roles = vec![];

        let parsing_error = GuardError::RoleError("Cannot parse row".to_owned());

        for row in rows {
            let subject = row.try_get("subject")
                .map_err(|_| parsing_error.clone())?;
            let domain = row.try_get("domain")
                .map_err(|_| parsing_error.clone())?;
            let role = row.try_get("role")
                .map_err(|_| parsing_error.clone())?;

            roles.push(Role {
                subject,
                domain,
                role,
            });
        }
        Ok(roles)
    }
}

impl PostgresRepository {
    async fn execute_role_query(&self, role: &&Role, query: &String) -> Result<PgQueryResult, Error> {
        let result = sqlx::query(&query)
            .bind(&role.subject)
            .bind(&role.domain)
            .bind(&role.role)
            .execute(&self.pool)
            .await;
        result
    }
}
