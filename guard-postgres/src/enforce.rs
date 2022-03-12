use guard::enforce::{EnforceRepository, EnforceRequest};
use guard::error::GuardError;
use crate::PostgresRepository;
use async_trait::async_trait;

#[async_trait]
impl EnforceRepository for PostgresRepository {
    async fn enforce(&self, request: &EnforceRequest) -> Result<bool, GuardError> {
        // Check if namespace exists first
        let role_table_name = format!("role_{}", request.namespace);
        let namespace_table_name = format!("namespace_{}", request.namespace);
        let query: String = format!(
            "SELECT true FROM {namespace_name} LEFT JOIN {role_name} ON {role_name}.role = {namespace_name}.role \
            WHERE {role_name}.subject='{subject}' AND {namespace_name}.object='{object}' AND {namespace_name}.action='{action}' \
            AND {namespace_name}.domain IN ('{domain}', '*') AND {role_name}.domain IN ('{domain}', '*') LIMIT 1;",
            namespace_name = namespace_table_name,
            role_name = role_table_name,
            subject = request.subject,
            object = request.object,
            action = request.action,
            domain = request.domain
        );
        let result: Result<Option<(bool,)>, sqlx::Error> = sqlx::query_as(&query)
            .fetch_optional(&self.pool)
            .await;

        match result {
            Ok(record) => {
                Ok(record.unwrap_or((false,)).0)
            },
            Err(error) => Err(GuardError::CannotEnforce(error.to_string()))
        }
    }
}
