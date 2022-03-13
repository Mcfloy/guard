#[cfg(test)]
mod permission_should {
    use guard::enforce::{EnforceRepository, EnforceRequest};
    use guard::permission::{Permission, PermissionRepository};
    use guard::role::{Role, RoleRepository};
    use guard_postgres::PostgresRepository;

    #[tokio::test]
    async fn return_false_when_enforce_unknown_permission() {
        let repository = PostgresRepository::new().await;

        let diona_permission = EnforceRequest {
            subject: "diona-test".to_owned(),
            namespace: "namespace-test".to_owned(),
            domain: "domain-test".to_owned(),
            object: "object-test".to_owned(),
            action: "action-test".to_owned()
        };

        let result = repository.enforce(&diona_permission).await.unwrap();
        assert_eq!(false, result);
    }

    #[tokio::test]
    async fn return_true_when_enforce_existing_permission() {
        let mut repository = PostgresRepository::new().await;

        let eula_permission = Permission {
            role: "owner".to_owned(),
            domain: "domain-test".to_owned(),
            object: "object-test".to_owned(),
            action: "action-test".to_owned()
        };
        repository.grant_permission("namespace-test", &eula_permission).await.unwrap();
        
        let eula_role_assignment = Role {
            subject: "eula-test".to_owned(),
            domain: "domain-test".to_owned(),
            role: "owner".to_owned()
        };
        repository.assign_role("namespace-test", &eula_role_assignment).await.unwrap();

        let request = EnforceRequest {
            subject: "eula-test".to_owned(),
            namespace: "namespace-test".to_owned(),
            domain: "domain-test".to_owned(),
            object: "object-test".to_owned(),
            action: "action-test".to_owned()
        };

        let result = repository.enforce(&request).await.unwrap();
        assert_eq!(true, result);

        repository.remove_permission("namespace-test", &eula_permission).await.unwrap();
    }

    #[tokio::test]
    async fn return_ok_when_authorize_permission() {
        let mut repository = PostgresRepository::new().await;

        let chongyun_permission = Permission {
            role: "owner-chongyun".to_owned(),
            domain: "domain-test".to_owned(),
            object: "object-test".to_owned(),
            action: "action-test".to_owned()
        };
        let result = repository.grant_permission("namespace-test", &chongyun_permission).await;

        assert!(result.is_ok());

        repository.remove_permission("namespace-test", &chongyun_permission).await.unwrap();
    }

    #[tokio::test]
    async fn return_err_when_authorize_permission_twice() {
        let mut repository = PostgresRepository::new().await;

        let diluc_permission = Permission {
            role: "owner-diluc".to_owned(),
            domain: "domain-test".to_owned(),
            object: "object-test".to_owned(),
            action: "action-test".to_owned()
        };
        let result = repository.grant_permission("namespace1-test", &diluc_permission).await;

        assert!(result.is_ok());

        let result = repository.grant_permission("namespace1-test", &diluc_permission).await;

        assert!(result.is_err());

        repository.remove_permission("namespace1-test", &diluc_permission).await.unwrap();
    }
}
