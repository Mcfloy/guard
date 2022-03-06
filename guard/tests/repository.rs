use std::collections::HashMap;
use std::error::Error;

use async_trait::async_trait;
use linked_hash_set::LinkedHashSet;

use guard::permission::{Permission, PermissionRepository};
use guard::error::GuardError;
use guard::role::{Role, RoleRepository};
use guard::namespace::NamespaceRepository;

pub struct InMemoryRepository {
    namespaces: LinkedHashSet<(String, String)>,
    /// Order of the parameters: Subject, Namespace, Domain, Object, Action
    permissions: LinkedHashSet<(String, String, String, String, String)>,
    roles: HashMap<(String, String, String), String>
}

impl InMemoryRepository {
    pub fn new() -> Self {
        InMemoryRepository {
            namespaces: LinkedHashSet::new(),
            permissions: LinkedHashSet::new(),
            roles: HashMap::new()
        }
    }
}

#[async_trait]
impl RoleRepository for InMemoryRepository {
    async fn add_role(&mut self, role: &Role) -> Result<(), GuardError> {
        self.roles.insert(
            (role.subject.clone(), role.namespace.clone(), role.domain.clone()),
            role.name.clone()
        );
        Ok(())
    }

    async fn remove_role(&mut self, role: &Role) -> Result<(), GuardError> {
        self.roles.remove(
            &(role.subject.clone(), role.namespace.clone(), role.domain.clone())
        );
        Ok(())
    }

    async fn list_roles(&self, _subject: Option<String>) -> Result<Vec<Role>, GuardError> {
        Ok(vec![])
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
impl PermissionRepository for InMemoryRepository {
    async fn enforce(&self, permission: &Permission) -> Result<bool, Box<dyn Error>> {
        let mut parameters = to_parameters(permission);
        match self.permissions.contains(&parameters) {
            true => Ok(true),
            false => {
                parameters.4 = "*".to_string();
                match self.permissions.contains(&parameters) {
                    true => Ok(true),
                    false => {
                        parameters.2 = "*".to_string();
                        Ok(self.permissions.contains(&parameters))
                    }
                }
            }
        }
    }

    async fn grant_permission(&mut self, permission: &Permission) -> Result<(), Box<dyn Error>> {
        let parameters = to_parameters(permission);
        if self.permissions.contains(&parameters) {
            Err(Box::new(GuardError::PermissionAlreadyExists))
        } else {
            self.permissions.insert(to_parameters(permission));
            self.namespaces.insert((permission.namespace.clone(), permission.subject.clone()));
            Ok(())
        }
    }

    async fn remove_permission(&mut self, permission: &Permission) -> Result<(), Box<dyn Error>> {
        let parameters = to_parameters(permission);
        if self.permissions.contains(&parameters) {
            self.permissions.remove(&parameters);
            Ok(())
        } else {
            Err(Box::new(GuardError::CannotRemovePermission))
        }
    }

    async fn contains_permission(&mut self, permission: &Permission) -> Result<bool, GuardError> {
        let parameters = to_parameters(permission);
        Ok(self.permissions.contains(&parameters))
    }

    async fn list_permissions_from_namespace(&mut self, _namespace: &str) -> Result<Vec<Permission>, Box<dyn Error>> {
        // DEVNOTE: This is a dumb workaround, but developing a column by column filter seems a bit exhausting.
        Ok(self.permissions
            .iter()
            .map(|fields| to_permission(fields))
            .collect::<Vec<Permission>>()
        )
    }
}

type Parameters = (String, String, String, String, String);

fn to_permission(parameters: &Parameters) -> Permission {
    Permission {
        subject: parameters.0.clone(),
        namespace: parameters.1.clone(),
        domain: parameters.2.clone(),
        object: parameters.3.clone(),
        action: parameters.4.clone()
    }
}

fn to_parameters(permission: &Permission) -> Parameters {
    (
        permission.subject.clone(),
        permission.namespace.clone(),
        permission.domain.clone(),
        permission.object.clone(),
        permission.action.clone()
    )
}

