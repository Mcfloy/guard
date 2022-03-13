#[cfg(test)]
mod namespaces_should {
    use guard::permission::{Permission, PermissionRepository};
    use guard::namespace::NamespaceRepository;
    use guard_postgres::PostgresRepository;

    #[tokio::test]
    async fn return_namespaces_when_retrieve_namespaces() {
        let mut repository = PostgresRepository::new().await;

        let albedo_permission = Permission {
            role: "owner".to_owned(),
            domain: "domain-test".to_owned(),
            object: "object-test".to_owned(),
            action: "action-test".to_owned()
        };
        repository.grant_permission("namespace1-test", &albedo_permission).await.unwrap();

        let amber_permission = Permission {
            role: "owner".to_owned(),
            domain: "domain-test".to_owned(),
            object: "object-test".to_owned(),
            action: "action-test".to_owned()
        };
        repository.grant_permission("namespace2-test", &amber_permission).await.unwrap();

        let barbara_permission = Permission {
            role: "owner".to_owned(),
            domain: "domain-test".to_owned(),
            object: "object-test".to_owned(),
            action: "action-test".to_owned()
        };
        repository.grant_permission("namespace3-test", &barbara_permission).await.unwrap();

        let namespaces = repository
            .get_namespaces()
            .await
            .unwrap();

        assert_eq!(namespaces, vec![
            "namespace1-test",
            "namespace2-test",
            "namespace3-test"
        ]);

        repository.remove_permission("namespace1-test", &albedo_permission).await.unwrap();
        repository.remove_permission("namespace2-test", &amber_permission).await.unwrap();
        repository.remove_permission("namespace3-test", &barbara_permission).await.unwrap();
    }

    #[tokio::test]
    async fn return_namespaces_of_user_when_retrieve_namespaces_of_user() {
        let mut repository = PostgresRepository::new().await;

        let beidou_permission = Permission {
            role: "owner".to_owned(),
            domain: "domain-test".to_owned(),
            object: "object-test".to_owned(),
            action: "action-test".to_owned()
        };
        repository.grant_permission("namespace1-test", &beidou_permission).await.unwrap();

        let bennett_permission = Permission {
            role: "owner".to_owned(),
            domain: "domain-test".to_owned(),
            object: "object-test".to_owned(),
            action: "action-test".to_owned()
        };
        repository.grant_permission("namespace2-test", &bennett_permission).await.unwrap();

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

        repository.remove_permission("namespace1-test", &beidou_permission).await.unwrap();
        repository.remove_permission("namespace2-test", &bennett_permission).await.unwrap();
    }
}
