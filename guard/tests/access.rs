mod repository;

#[cfg(test)]
mod access_should {
    use guard::access::{Access, AccessRepository};
    use guard::namespace::NamespaceRepository;

    use crate::repository as test_repository;

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
