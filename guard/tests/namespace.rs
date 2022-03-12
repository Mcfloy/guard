mod repository;

#[cfg(test)]
mod namespace_should {
    use guard::permission::{Permission, PermissionRepository};
    use guard::namespace::NamespaceRepository;
    use crate::repository as test_repository;

    #[tokio::test]
    async fn return_ok_when_create_namespace() {
        let mut repository = test_repository::InMemoryRepository::new();

        let result = repository.create_namespace("namespace1").await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn return_err_when_create_twice_namespace() {
        let mut repository = test_repository::InMemoryRepository::new();

        let successful_creation = repository.create_namespace("namespace1").await;
        assert!(successful_creation.is_ok());

        let failed_creation = repository.create_namespace("namespace1").await;
        assert!(failed_creation.is_err());
    }

    #[tokio::test]
    async fn return_namespaces_when_retrieve_namespaces() {
        let mut repository = test_repository::InMemoryRepository::new();

        let namespace1_creation_result = repository.create_namespace("namespace1").await;
        assert!(namespace1_creation_result.is_ok());

        let namespace2_creation_result = repository.create_namespace("namespace2").await;
        assert!(namespace2_creation_result.is_ok());

        let namespace3_creation_result = repository.create_namespace("namespace3").await;
        assert!(namespace3_creation_result.is_ok());

        let namespaces = repository
            .get_namespaces()
            .await
            .unwrap();

        assert_eq!(namespaces, vec![
            "namespace1",
            "namespace2",
            "namespace3"
        ]);
    }
}
