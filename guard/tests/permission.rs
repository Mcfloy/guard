mod repository;

#[cfg(test)]
mod permission_should {
    use guard::permission::{Permission, PermissionRepository};
    use guard::namespace::NamespaceRepository;

    use crate::repository as test_repository;

    #[tokio::test]
    async fn return_false_when_enforce_unknown_permission() {
        let repository = test_repository::InMemoryRepository::new();

        let diona_permission = Permission {
            subject: "diona-test".to_string(),
            namespace: "namespace-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };

        let result = repository.enforce(&diona_permission).await.unwrap();
        assert_eq!(false, result);
    }

    #[tokio::test]
    async fn return_true_when_enforce_existing_permission() {
        let mut repository = test_repository::InMemoryRepository::new();

        let eula_permission = Permission {
            subject: "eula-test".to_string(),
            namespace: "namespace-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        let authorization = repository.grant_permission(&eula_permission).await;
        assert!(authorization.is_ok());

        let result = repository.enforce(&eula_permission).await.unwrap();
        assert_eq!(true, result);
    }

    #[tokio::test]
    async fn return_true_when_enforce_permission_with_wildcard() {
        let mut repository = test_repository::InMemoryRepository::new();

        let fischl_wildcard_permission = Permission {
            subject: "fischl-test".to_string(),
            namespace: "namespace-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "*".to_string()
        };
        let authorization = repository.grant_permission(&fischl_wildcard_permission).await;
        assert!(authorization.is_ok());

        let fischl_permission = Permission {
            subject: "fischl-test".to_string(),
            namespace: "namespace-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        let result = repository.enforce(&fischl_permission).await.unwrap();
        assert_eq!(true, result);
    }

    #[tokio::test]
    async fn return_error_when_authorize_already_existing_permission() {
        let mut repository = test_repository::InMemoryRepository::new();

        let successful_authorization = repository.grant_permission(&Permission {
            subject: "Tony Stark".to_string(),
            namespace: "Avengers".to_string(),
            domain: "*".to_string(),
            object: "Mark I".to_string(),
            action: "fly".to_string()
        }).await;
        assert!(successful_authorization.is_ok());

        let failed_authorization = repository.grant_permission(&Permission {
            subject: "Tony Stark".to_string(),
            namespace: "Avengers".to_string(),
            domain: "*".to_string(),
            object: "Mark I".to_string(),
            action: "fly".to_string()
        }).await;

        assert!(failed_authorization.is_err());
    }

    #[tokio::test]
    async fn return_ok_when_authorize_permission() {
        let mut repository = test_repository::InMemoryRepository::new();

        let request = repository.grant_permission(&Permission {
            subject: "Tony Stark".to_string(),
            namespace: "Avengers".to_string(),
            domain: "*".to_string(),
            object: "Mark I".to_string(),
            action: "fly".to_string()
        }).await;
        assert!(request.is_ok());

        let namespaces = repository
            .get_namespaces_of_subject("Tony Stark")
            .await
            .unwrap();

        assert_eq!(namespaces, vec![
            "Avengers"
        ]);
    }

    #[tokio::test]
    async fn return_vec_of_permissions_when_get_permission() {
        let mut repository = test_repository::InMemoryRepository::new();

        let ganyu_permission = &Permission {
            subject: "ganyu-test".to_string(),
            namespace: "namespace1-test".to_string(),
            domain: "*".to_string(),
            object: "object".to_string(),
            action: "action".to_string()
        };

        let authorization = repository.grant_permission(&ganyu_permission).await;
        assert!(authorization.is_ok());

        let permissions = repository.list_permissions_from_namespace("namespace1-test").await.unwrap();
        assert_eq!(1, permissions.len());
        assert_eq!(ganyu_permission, permissions.get(0).unwrap());
    }
}
