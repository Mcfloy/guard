mod repository;

#[cfg(test)]
mod permission_should {
    use guard::permission::{Permission, PermissionRepository};
    use guard::namespace::NamespaceRepository;

    use crate::repository as test_repository;

    #[tokio::test]
    async fn return_error_when_authorize_already_existing_permission() {
        let mut repository = test_repository::InMemoryRepository::new();

        let permission = Permission {
            role: "superhero".to_string(),
            domain: "new-york".to_string(),
            object: "shawarma".to_string(),
            action: "eat".to_string()
        };
        let successful_permission = repository.grant_permission("avengers", &permission).await;
        assert!(successful_permission.is_ok());

        let failed_authorization = repository.grant_permission("avengers", &permission).await;
        assert!(failed_authorization.is_err());
    }

    #[tokio::test]
    async fn return_vec_of_permissions_when_get_permission() {
        let mut repository = test_repository::InMemoryRepository::new();

        let permission = Permission {
            role: "superhero".to_string(),
            domain: "new-york".to_string(),
            object: "shawarma".to_string(),
            action: "eat".to_string()
        };

        let authorization = repository.grant_permission("avengers", &permission).await;
        assert!(authorization.is_ok());

        let permissions = repository.list_permissions("avengers").await.unwrap();
        assert_eq!(1, permissions.len());
        assert_eq!(&permission, permissions.get(0).unwrap());
    }
}
