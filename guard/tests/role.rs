mod repository;

#[cfg(test)]
mod role_should {
    use guard::role::{Role, RoleRepository};
    use crate::repository as test_repository;

    #[tokio::test]
    async fn return_ok_when_add_role() {
        let mut repository = test_repository::InMemoryRepository::new();

        let role = Role {
            subject: "alice".to_owned(),
            domain: "domain".to_owned(),
            role: "owner".to_owned()
        };
        let result = repository.assign_role("namespace-test", &role).await;
        assert_eq!(result.is_ok());
    }

    #[tokio::test]
    async fn return_err_when_add_role() {
        let mut repository = test_repository::InMemoryRepository::new();

        let role = Role {
            subject: "alice".to_owned(),
            domain: "domain".to_owned(),
            role: "owner".to_owned(),
        };
        let successful_addition = repository.assign_role("namespace-test", &role).await;
        assert_eq!(successful_addition.is_ok());

        let failed_addition = repository.assign_role("namespace-test", &role).await;
        assert_eq!(failed_addition.is_err());
    }

    #[tokio::test]
    async fn return_ok_when_remove_role() {
        let mut repository = test_repository::InMemoryRepository::new();

        let role = Role {
            subject: "alice".to_owned(),
            domain: "domain".to_owned(),
            role: "owner".to_owned(),
        };
        let successful_addition = repository.assign_role("namespace-test", &role).await;
        assert_eq!(successful_addition.is_ok());

        let successful_removal = repository.remove_role("namespace-test", &role).await;
        assert_eq!(successful_removal.is_ok());
    }
}
