use std::io;
use clap::{Parser, Subcommand};
use guard::role::{Role, RoleRepository};
use guard::permission::{Permission, PermissionRepository};

#[derive(Parser, Debug)]
enum Guard {
    #[clap(subcommand)]
    Namespaces(NamespaceDerive)
}

#[derive(Subcommand, Debug)]
enum NamespaceDerive {
    List { namespace: Option<String> },
    Allow { subject: String, namespace: String }
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    dotenv::dotenv().ok();
    let mut repository = guard_postgres::PostgresRepository::new().await;
    for i in 10000..1000000 {
        let permission = Permission {
            role: format!("owner-{}", i),
            domain: "test".to_owned(),
            object: "permission".to_owned(),
            action: "edit".to_owned()
        };
        repository.grant_permission("guard", &permission).await;
        let role = Role {
            subject: format!("test-{}", i),
            role: format!("owner-{}", i).to_owned(),
            domain: "test".to_owned()
        };
        repository.assign_role("guard", &role).await;
    }
    Ok(())
}

