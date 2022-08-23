use std::{fmt, io};
use std::error::Error;
use clap::{Parser, Subcommand, Args};
use anyhow::{Context, Result};

#[derive(Debug, Parser)]
#[clap(name = "guard")]
#[clap(about = "CLI for managing Guard API", long_about = None)]
struct GuardCli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Debug)]
struct GuardError {
    details: String,
}

impl GuardError {
    fn new(msg: &str) -> GuardError {
        GuardError { details: msg.to_string() }
    }
}

impl fmt::Display for GuardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for GuardError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(arg_required_else_help = true)]
    Login {
        #[clap(value_parser)]
        // TODO: Don't use jwt token but redirect to SSO.
        jwt_token: String,
    },
    Namespace(Namespace),
    Role(Role),
}

#[derive(Debug, Args)]
struct Namespace {
    #[clap(subcommand)]
    command: NamespaceCommands,
}

#[derive(Debug, Subcommand)]
enum NamespaceCommands {
    List,
    Allow { subject: String, namespace: String },
}

#[derive(Debug, Args)]
struct Role {
    #[clap(subcommand)]
    command: RoleCommands,
}

#[derive(Debug, Subcommand)]
enum RoleCommands {
    List
}

fn check_jwt(jwt: &str) -> Result<(), GuardError> {
    match jwt.is_empty() {
        true => Err(GuardError::new("Not logged in")),
        false => Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let args = GuardCli::parse();
    let mut stored_jwt: String = String::new();

    match args.commands {
        Commands::Login { jwt_token } => {
            stored_jwt = String::clone(&jwt_token);
            println!("Successfully logged in!");
        }
        Commands::Namespace(namespace) => {
            check_jwt(&stored_jwt)?;
            match namespace.command {
                NamespaceCommands::List => {
                    println!("Hello list")
                }
                NamespaceCommands::Allow { subject, namespace } => {
                    println!("Allow user")
                }
            }
        }
        Commands::Role(_) => {
            check_jwt(&stored_jwt)?;
            todo!()
        }
    }
    Ok(())
}

