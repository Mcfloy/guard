mod repository;

#[cfg(test)]
mod namespace_should {
    use guard::permission::{Permission, PermissionRepository};
    use guard::namespace::NamespaceRepository;
    use crate::repository as test_repository;

    #[tokio::test]
    async fn return_namespaces_when_retrieve_namespaces() {
        let mut repository = test_repository::InMemoryRepository::new();
        let alice_permission = Permission {
            subject: "alice".to_string(),
            namespace: "namespace1".to_string(),
            domain: "domain".to_string(),
            object: "object".to_string(),
            action: "action".to_string()
        };
        repository.grant_permission(&alice_permission).await.unwrap();

        let bob_permission = Permission {
            subject: "bob".to_string(),
            namespace: "namespace2".to_string(),
            domain: "domain".to_string(),
            object: "object".to_string(),
            action: "action".to_string()
        };
        repository.grant_permission(&bob_permission).await.unwrap();

        let charles_permission = Permission {
            subject: "charles".to_string(),
            namespace: "namespace3".to_string(),
            domain: "domain".to_string(),
            object: "object".to_string(),
            action: "action".to_string()
        };
        repository.grant_permission(&charles_permission).await.unwrap();

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

        let alice_permission = Permission {
            subject: "alice".to_string(),
            namespace: "namespace1".to_string(),
            domain: "domain".to_string(),
            object: "object".to_string(),
            action: "action".to_string()
        };
        repository.grant_permission(&alice_permission).await.unwrap();

        let bob_permission = Permission {
            subject: "bob".to_string(),
            namespace: "namespace2".to_string(),
            domain: "domain".to_string(),
            object: "object".to_string(),
            action: "action".to_string()
        };
        repository.grant_permission(&bob_permission).await.unwrap();

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
