#[cfg(test)]
mod access_should {
    use guard::access::{Access, AccessRepository};
    use guard_postgres::PostgresRepository;

    #[tokio::test]
    async fn return_false_when_enforce_unknown_access() {
        let repository = PostgresRepository::new().await;

        let diona_access = Access {
            subject: "diona-test".to_string(),
            namespace: "namespace-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };

        let result = repository.enforce(&diona_access).await.unwrap();
        assert_eq!(false, result);
    }

    #[tokio::test]
    async fn return_true_when_enforce_existing_access() {
        let mut repository = PostgresRepository::new().await;

        let eula_access = Access {
            subject: "eula-test".to_string(),
            namespace: "namespace-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        repository.authorize_access(&eula_access).await.unwrap();

        let result = repository.enforce(&eula_access).await.unwrap();
        assert_eq!(true, result);

        repository.remove_access(&eula_access).await.unwrap();
    }

    #[tokio::test]
    async fn return_true_when_enforce_access_with_wildcard() {
        let mut repository = PostgresRepository::new().await;

        let fischl_wildcard_access = Access {
            subject: "fischl-test".to_string(),
            namespace: "namespace-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "*".to_string()
        };
        repository.authorize_access(&fischl_wildcard_access).await.unwrap();

        let fischl_access = Access {
            subject: "fischl-test".to_string(),
            namespace: "namespace-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        let result = repository.enforce(&fischl_access).await.unwrap();
        assert_eq!(true, result);

        repository.remove_access(&fischl_wildcard_access).await.unwrap();
    }

    #[tokio::test]
    async fn return_ok_when_authorize_access() {
        let mut repository = PostgresRepository::new().await;

        let chongyun_access = Access {
            subject: "chongyun-test".to_string(),
            namespace: "namespace1-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        let result = repository.authorize_access(&chongyun_access).await;

        assert!(result.is_ok());

        repository.remove_access(&chongyun_access).await.unwrap();
    }

    #[tokio::test]
    async fn return_err_when_authorize_access_twice() {
        let mut repository = PostgresRepository::new().await;

        let diluc_access = Access {
            subject: "diluc-test".to_string(),
            namespace: "namespace1-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        let result = repository.authorize_access(&diluc_access).await;

        assert!(result.is_ok());

        let result = repository.authorize_access(&diluc_access).await;

        assert!(result.is_err());

        repository.remove_access(&diluc_access).await.unwrap();
    }
}
