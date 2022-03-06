#[cfg(test)]
mod namespaces_should {
    use guard::permission::{Permission, PermissionRepository};
    use guard::namespace::NamespaceRepository;
    use guard_postgres::PostgresRepository;

    #[tokio::test]
    async fn return_namespaces_when_retrieve_namespaces() {
        let mut repository = PostgresRepository::new().await;

        let albedo_permission = Permission {
            subject: "albedo-test".to_string(),
            namespace: "namespace1-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        repository.grant_permission(&albedo_permission).await.unwrap();

        let amber_permission = Permission {
            subject: "amber-test".to_string(),
            namespace: "namespace2-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        repository.grant_permission(&amber_permission).await.unwrap();

        let barbara_permission = Permission {
            subject: "barbara-test".to_string(),
            namespace: "namespace3-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        repository.grant_permission(&barbara_permission).await.unwrap();

        let namespaces = repository
            .get_namespaces()
            .await
            .unwrap();

        assert_eq!(namespaces, vec![
            "namespace1-test",
            "namespace2-test",
            "namespace3-test"
        ]);

        repository.remove_permission(&albedo_permission).await.unwrap();
        repository.remove_permission(&amber_permission).await.unwrap();
        repository.remove_permission(&barbara_permission).await.unwrap();
    }

    #[tokio::test]
    async fn return_namespaces_of_user_when_retrieve_namespaces_of_user() {
        let mut repository = PostgresRepository::new().await;

        let beidou_permission = Permission {
            subject: "beidou-test".to_string(),
            namespace: "namespace1-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        repository.grant_permission(&beidou_permission).await.unwrap();

        let bennett_permission = Permission {
            subject: "bennett-test".to_string(),
            namespace: "namespace2-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        repository.grant_permission(&bennett_permission).await.unwrap();

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

        repository.remove_permission(&beidou_permission).await.unwrap();
        repository.remove_permission(&bennett_permission).await.unwrap();
    }
}
