#[cfg(test)]
mod permission_should {
    use guard::permission::{Permission, PermissionRepository};
    use guard_postgres::PostgresRepository;

    #[tokio::test]
    async fn return_false_when_enforce_unknown_permission() {
        let repository = PostgresRepository::new().await;

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
        let mut repository = PostgresRepository::new().await;

        let eula_permission = Permission {
            subject: "eula-test".to_string(),
            namespace: "namespace-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        repository.grant_permission(&eula_permission).await.unwrap();

        let result = repository.enforce(&eula_permission).await.unwrap();
        assert_eq!(true, result);

        repository.remove_permission(&eula_permission).await.unwrap();
    }

    #[tokio::test]
    async fn return_true_when_enforce_permission_with_wildcard() {
        let mut repository = PostgresRepository::new().await;

        let fischl_wildcard_permission = Permission {
            subject: "fischl-test".to_string(),
            namespace: "namespace-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "*".to_string()
        };
        repository.grant_permission(&fischl_wildcard_permission).await.unwrap();

        let fischl_permission = Permission {
            subject: "fischl-test".to_string(),
            namespace: "namespace-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        let result = repository.enforce(&fischl_permission).await.unwrap();
        assert_eq!(true, result);

        repository.remove_permission(&fischl_wildcard_permission).await.unwrap();
    }

    #[tokio::test]
    async fn return_ok_when_authorize_permission() {
        let mut repository = PostgresRepository::new().await;

        let chongyun_permission = Permission {
            subject: "chongyun-test".to_string(),
            namespace: "namespace1-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        let result = repository.grant_permission(&chongyun_permission).await;

        assert!(result.is_ok());

        repository.remove_permission(&chongyun_permission).await.unwrap();
    }

    #[tokio::test]
    async fn return_err_when_authorize_permission_twice() {
        let mut repository = PostgresRepository::new().await;

        let diluc_permission = Permission {
            subject: "diluc-test".to_string(),
            namespace: "namespace1-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        let result = repository.grant_permission(&diluc_permission).await;

        assert!(result.is_ok());

        let result = repository.grant_permission(&diluc_permission).await;

        assert!(result.is_err());

        repository.remove_permission(&diluc_permission).await.unwrap();
    }
}
