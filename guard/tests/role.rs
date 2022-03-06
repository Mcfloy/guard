mod repository;

#[cfg(test)]
mod role_should {
    use guard::role::{Role, RoleRepository};
    use crate::repository as test_repository;

    #[tokio::test]
    async fn return_ok_when_add_role() {
        let mut repository = test_repository::InMemoryRepository::new();

        let role = Role {
            subject: "alice".to_string(),
            name: "owner".to_string(),
            namespace: "namespace1".to_string(),
            domain: "domain".to_string()
        };
        let result = repository.add_role(&role).await;
        assert_eq!(result.is_ok());
    }

    #[tokio::test]
    async fn return_err_when_add_role() {
        let mut repository = test_repository::InMemoryRepository::new();

        let role = Role {
            subject: "alice".to_string(),
            name: "owner".to_string(),
            namespace: "namespace1".to_string(),
            domain: "domain".to_string()
        };
        let successful_addition = repository.add_role(&role).await;
        assert_eq!(successful_addition.is_ok());

        let failed_addition = repository.add_role(&role).await;
        assert_eq!(failed_addition.is_err());
    }

    #[tokio::test]
    async fn return_ok_when_remove_role() {
        let mut repository = test_repository::InMemoryRepository::new();

        let role = Role {
            subject: "alice".to_string(),
            name: "owner".to_string(),
            namespace: "namespace1".to_string(),
            domain: "domain".to_string()
        };
        let successful_addition = repository.add_role(&role).await;
        assert_eq!(successful_addition.is_ok());

        let successful_removal = repository.remove_role(&role).await;
        assert_eq!(successful_removal.is_ok());
    }
}
