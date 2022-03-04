use std::error::Error;

use async_trait::async_trait;
use linked_hash_set::LinkedHashSet;

use guard::access::{Access, AccessRepository};
use guard::error::GuardError;
use guard::namespace::NamespaceRepository;

pub struct InMemoryRepository {
    namespaces: LinkedHashSet<(String, String)>,
    /// Order of the parameters: Subject, Namespace, Domain, Object, Action
    accesses: LinkedHashSet<(String, String, String, String, String)>
}

impl InMemoryRepository {
    pub fn new() -> Self {
        InMemoryRepository {
            namespaces: LinkedHashSet::new(),
            accesses: LinkedHashSet::new()
        }
    }
}

#[async_trait]
impl NamespaceRepository for InMemoryRepository {
    async fn get_namespaces(&self) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(
            self.namespaces
                .iter()
                .map(|(ns, _sub)| ns)
                .cloned()
                .collect::<Vec<String>>()
        )
    }

    async fn get_namespaces_of_subject(&self, subject: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let owned_subject = String::from(subject);
        Ok(
            self.namespaces
                .iter()
                .filter_map(|(ns, sub)| {
                    match owned_subject.eq(sub) {
                        true => Some(ns),
                        false => None
                    }
                })
                .cloned()
                .collect::<Vec<String>>()
        )
    }
}

#[async_trait]
impl AccessRepository for InMemoryRepository {
    async fn enforce(&self, access: &Access) -> Result<bool, Box<dyn Error>> {
        let mut parameters = to_parameters(access);
        match self.accesses.contains(&parameters) {
            true => Ok(true),
            false => {
                parameters.4 = "*".to_string();
                match self.accesses.contains(&parameters) {
                    true => Ok(true),
                    false => {
                        parameters.2 = "*".to_string();
                        Ok(self.accesses.contains(&parameters))
                    }
                }
            }
        }
    }

    async fn authorize_access(&mut self, access: &Access) -> Result<(), Box<dyn Error>> {
        let parameters = to_parameters(access);
        if self.accesses.contains(&parameters) {
            Err(Box::new(GuardError::AccessAlreadyExists))
        } else {
            self.accesses.insert(to_parameters(access));
            self.namespaces.insert((access.namespace.clone(), access.subject.clone()));
            Ok(())
        }
    }

    async fn remove_access(&mut self, access: &Access) -> Result<(), Box<dyn Error>> {
        let parameters = to_parameters(access);
        if self.accesses.contains(&parameters) {
            self.accesses.remove(&parameters);
            Ok(())
        } else {
            Err(Box::new(GuardError::CannotRemoveAccess))
        }
    }
}

fn to_parameters(access: &Access) -> (String, String, String, String, String) {
    (
        access.subject.clone(),
        access.namespace.clone(),
        access.domain.clone(),
        access.object.clone(),
        access.action.clone()
    )
}

