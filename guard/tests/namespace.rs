mod repository;

#[cfg(test)]
mod namespaces_should {
    use guard::access::{Access, AccessRepository};
    use guard::namespace::NamespaceRepository;
    use crate::repository as test_repository;

    #[tokio::test]
    async fn return_namespaces_when_retrieve_namespaces() {
        let mut repository = test_repository::InMemoryRepository::new();
        let alice_access = Access {
            subject: "alice".to_string(),
            namespace: "namespace1".to_string(),
            domain: "domain".to_string(),
            object: "object".to_string(),
            action: "action".to_string()
        };
        repository.authorize_access(&alice_access).await.unwrap();

        let bob_access = Access {
            subject: "bob".to_string(),
            namespace: "namespace2".to_string(),
            domain: "domain".to_string(),
            object: "object".to_string(),
            action: "action".to_string()
        };
        repository.authorize_access(&bob_access).await.unwrap();

        let charles_access = Access {
            subject: "charles".to_string(),
            namespace: "namespace3".to_string(),
            domain: "domain".to_string(),
            object: "object".to_string(),
            action: "action".to_string()
        };
        repository.authorize_access(&charles_access).await.unwrap();

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

        let alice_access = Access {
            subject: "alice".to_string(),
            namespace: "namespace1".to_string(),
            domain: "domain".to_string(),
            object: "object".to_string(),
            action: "action".to_string()
        };
        repository.authorize_access(&alice_access).await.unwrap();

        let bob_access = Access {
            subject: "bob".to_string(),
            namespace: "namespace2".to_string(),
            domain: "domain".to_string(),
            object: "object".to_string(),
            action: "action".to_string()
        };
        repository.authorize_access(&bob_access).await.unwrap();

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
