#[cfg(test)]
mod namespaces_should {
    use guard::access::{Access, AccessRepository};
    use guard::namespace::NamespaceRepository;
    use guard_postgres::PostgresRepository;

    #[tokio::test]
    async fn return_namespaces_when_retrieve_namespaces() {
        let mut repository = PostgresRepository::new().await;

        let albedo_access = Access {
            subject: "albedo-test".to_string(),
            namespace: "namespace1-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        repository.authorize_access(&albedo_access).await.unwrap();

        let amber_access = Access {
            subject: "amber-test".to_string(),
            namespace: "namespace2-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        repository.authorize_access(&amber_access).await.unwrap();

        let barbara_access = Access {
            subject: "barbara-test".to_string(),
            namespace: "namespace3-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        repository.authorize_access(&barbara_access).await.unwrap();

        let namespaces = repository
            .get_namespaces()
            .await
            .unwrap();

        assert_eq!(namespaces, vec![
            "namespace1-test",
            "namespace2-test",
            "namespace3-test"
        ]);

        repository.remove_access(&albedo_access).await.unwrap();
        repository.remove_access(&amber_access).await.unwrap();
        repository.remove_access(&barbara_access).await.unwrap();
    }

    #[tokio::test]
    async fn return_namespaces_of_user_when_retrieve_namespaces_of_user() {
        let mut repository = PostgresRepository::new().await;

        let beidou_access = Access {
            subject: "beidou-test".to_string(),
            namespace: "namespace1-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        repository.authorize_access(&beidou_access).await.unwrap();

        let bennett_access = Access {
            subject: "bennett-test".to_string(),
            namespace: "namespace2-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        repository.authorize_access(&bennett_access).await.unwrap();

        let namespaces = repository
            .get_namespaces_of_subject("beidou-test")
            .await
            .unwrap();

        assert_eq!(namespaces, vec![
            "namespace1-test"
        ]);

        let namespaces = repository
            .get_namespaces_of_subject("bennett-test")
            .await
            .unwrap();

        assert_eq!(namespaces, vec![
            "namespace2-test"
        ]);

        repository.remove_access(&beidou_access).await.unwrap();
        repository.remove_access(&bennett_access).await.unwrap();
    }
}
