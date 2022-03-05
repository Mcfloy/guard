use std::io;
use clap::{Parser, Subcommand};
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
    let Guard::Namespaces(args) = Guard::parse();
    match args {
        NamespaceDerive::Allow { subject, namespace } => {
            let mut repository = guard_postgres::PostgresRepository::new().await;
            let permission = Permission {
                subject: subject.clone(),
                namespace: "guard".to_string(),
                domain: namespace.clone(),
                object: "permission".to_string(),
                action: "edit".to_string()
            };
            if let true = repository.contains_permission(&permission).await.unwrap() {
                eprintln!("Permission has already been granted.");
                return Ok(());
            }
            repository.grant_permission(&permission).await.unwrap();
            println!(
                "Permission granted for {} to edit permissions of namespace {}",
                subject,
                namespace
            );
            return Ok(());
        }
        NamespaceDerive::List { .. } => {}
    }
    println!("{:?}", args);
    Ok(())
}
