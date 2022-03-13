use async_trait::async_trait;
use sqlx::{Error, query};
use guard::error::GuardError;
use guard::namespace::NamespaceRepository;
use crate::PostgresRepository;

#[async_trait]
impl NamespaceRepository for PostgresRepository {
    async fn create_namespace(&mut self, namespace: &str) -> Result<(), GuardError> {
        // TODO: Use tracing warning inside the map_err methods.
        let mut transaction = self.pool.begin().await
            .map_err(|_| GuardError::NamespaceError("Can't begin transaction".to_owned()))?;

        // Namespace has only one field that is a primary key, so using this query
        // inside a transaction will cancel the next query.
        query!("INSERT INTO namespace VALUES ($1)", namespace)
            .execute(&mut *transaction)
            .await
            .map_err(|_| GuardError::NamespaceError("Can't add new namespace".to_owned()))?;

        let table_creation_query = format!("
            CREATE TABLE IF NOT EXISTS {namespace} (
                role   varchar(32),
                domain varchar(32) default '*',
                object varchar(32),
                action varchar(32),
                constraint namespace_{namespace}_pk primary key (role, domain, object, action)
            );
            CREATE TABLE IF NOT EXISTS role_{namespace} (
                subject varchar(128),
                domain varchar(32) default '*',
                role varchar(32),
                constraint role_{namespace}_pk primary key (subject, domain, role)
            );
            CREATE INDEX IF NOT EXISTS role_{namespace}_domain_role_index ON role_{namespace} (domain, role);
            CREATE INDEX IF NOT EXISTS role_{namespace}_subject_domain_index ON role_{namespace} (subject, domain)",
            namespace = namespace
        );
        query(&table_creation_query)
            .execute(&mut *transaction)
            .await
            .map_err(|_| GuardError::NamespaceError("Cannot execute transaction".to_owned()))?;

        transaction.commit().await
            .map_err(|_| GuardError::NamespaceError("Cannot commit transaction".to_owned()))?;

        Ok(())
    }

    async fn get_namespaces(&self) -> Result<Vec<String>, GuardError> {
        let result = sqlx::query!("SELECT id FROM namespace")
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(records) => Ok(records.iter().map(|r| r.id.to_owned()).collect()),
            Err(_) => Err(GuardError::NamespaceError("Cannot get namespaces".to_owned()))
        }
    }

    async fn does_namespace_exists(&self, namespace: &str) -> Result<bool, GuardError> {
        let result = sqlx::query!("SELECT true FROM namespace WHERE id=$1", namespace)
            .fetch_one(&self.pool)
            .await;

        match result {
            Ok(record) => Ok(record.bool.unwrap_or(false)),
            Err(error) => match error {
                Error::RowNotFound => Ok(false),
                _ => Err(GuardError::NamespaceError("Can't determine if namespace exists.".to_owned()))
            }
        }
    }
}
