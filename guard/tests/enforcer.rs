mod repository;

#[cfg(test)]
mod enforcer_should {
    use guard::enforce::{EnforceRepository, EnforceRequest};
    use guard::namespace::NamespaceRepository;
    use guard::permission::{Permission, PermissionRepository};
    use guard::role::{Role, RoleRepository};
    use crate::repository as test_repository;

    #[tokio::test]
    async fn return_false_when_enforce_unknown_permission() {
        let mut repository = test_repository::InMemoryRepository::new();

        repository.create_namespace("namespace-test").await.unwrap();

        let diona_request = EnforceRequest {
            subject: "diona-test".to_string(),
            namespace: "namespace-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };

        let result = repository.enforce(&diona_request).await.unwrap();
        assert_eq!(false, result);
    }

    #[tokio::test]
    async fn return_true_when_enforce_existing_permission() {
        let mut repository = test_repository::InMemoryRepository::new();

        let creation_result = repository.create_namespace("namespace-test").await;
        assert!(creation_result.is_ok());

        let permission = Permission {
            role: "owner".to_owned(),
            domain: "namespace-test".to_owned(),
            object: "object-test".to_owned(),
            action: "action-test".to_owned()
        };

        let granting_result = repository.grant_permission("namespace-test", &permission).await;
        assert!(granting_result.is_ok());

        let eula_role = Role {
            subject: "eula-test".to_owned(),
            domain: "domain-test".to_owned(),
            role: "owner".to_owned()
        };
        let assignment_result = repository.assign_role("namespace-test", &eula_role).await;
        assert!(assignment_result.is_ok());

        let eula_request = EnforceRequest {
            subject: "eula-test".to_string(),
            namespace: "namespace-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };

        let result = repository.enforce(&eula_request).await.unwrap();
        assert_eq!(true, result);
    }

    #[tokio::test]
    async fn return_true_when_enforce_permission_with_wildcard_domain() {
        let mut repository = test_repository::InMemoryRepository::new();

        let creation_result = repository.create_namespace("namespace-test").await;
        assert!(creation_result.is_ok());

        let permission = Permission {
            role: "owner".to_owned(),
            domain: "*".to_owned(),
            object: "object-test".to_owned(),
            action: "action-test".to_owned()
        };

        let granting_result = repository.grant_permission("namespace-test", &permission).await;
        assert!(granting_result.is_ok());

        let eula_role = Role {
            subject: "fischl-test".to_owned(),
            domain: "domain-test".to_owned(),
            role: "owner".to_owned()
        };
        let assignment_result = repository.assign_role("namespace-test", &eula_role).await;
        assert!(assignment_result.is_ok());

        let fischl_permission = EnforceRequest {
            subject: "fischl-test".to_string(),
            namespace: "namespace-test".to_string(),
            domain: "domain-test".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        let result = repository.enforce(&fischl_permission).await;
        assert!(result.is_ok());

        let fischl_wildcard_permission = EnforceRequest {
            subject: "fischl-test".to_string(),
            namespace: "namespace-test".to_string(),
            domain: "*".to_string(),
            object: "object-test".to_string(),
            action: "action-test".to_string()
        };
        let result = repository.enforce(&fischl_wildcard_permission).await;
        assert!(result.is_ok());
    }
}
