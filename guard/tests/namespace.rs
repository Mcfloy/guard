mod repository;

#[cfg(test)]
mod namespaces_should {
    use guard::namespace::NamespaceRepository;
    use crate::repository as test_repository;

    #[tokio::test]
    async fn return_namespaces_when_retrieve_namespaces() {
        let mut repository = test_repository::InMemoryRepository::new();
        repository.init_values();

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

    #[tokio::test]
    async fn return_namespaces_of_user_when_retrieve_namespaces_of_user() {
        let mut repository = test_repository::InMemoryRepository::new();
        repository.init_values();
        let namespaces = repository
            .get_namespaces_of_subject("alice")
            .await
            .unwrap();

        assert_eq!(namespaces, vec![
            "namespace1"
        ]);

        let namespaces = repository
            .get_namespaces_of_subject("bob")
            .await
            .unwrap();

        assert_eq!(namespaces, vec![
            "namespace2"
        ]);
    }
}
