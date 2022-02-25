mod repository;

#[cfg(test)]
mod access_should {
    use guard::access::{Access, AccessRepository};
    use guard::namespace::NamespaceRepository;

    use crate::repository as test_repository;

    #[tokio::test]
    async fn return_false_when_enforce_unknown_access() {
        let mut repository = test_repository::InMemoryRepository::new();

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
        let mut repository = test_repository::InMemoryRepository::new();

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
    }

    #[tokio::test]
    async fn return_true_when_enforce_access_with_wildcard() {
        let mut repository = test_repository::InMemoryRepository::new();

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
    }

    #[tokio::test]
    async fn return_error_when_authorize_already_existing_access() {
        let mut repository = test_repository::InMemoryRepository::new();

        let successful_request = repository.authorize_access(&Access {
            subject: "Tony Stark".to_string(),
            namespace: "Avengers".to_string(),
            domain: "*".to_string(),
            object: "Mark I".to_string(),
            action: "fly".to_string()
        }).await;
        assert!(successful_request.is_ok());

        let failed_request = repository.authorize_access(&Access {
            subject: "Tony Stark".to_string(),
            namespace: "Avengers".to_string(),
            domain: "*".to_string(),
            object: "Mark I".to_string(),
            action: "fly".to_string()
        }).await;

        assert!(failed_request.is_err());
    }

    #[tokio::test]
    async fn return_ok_when_authorize_access() {
        let mut repository = test_repository::InMemoryRepository::new();

        let request = repository.authorize_access(&Access {
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
}
