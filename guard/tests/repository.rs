use std::collections::HashMap;
use async_trait::async_trait;
use guard::enforce::{EnforceRequest, EnforceRepository};

use guard::permission::{Permission, PermissionRepository};
use guard::error::GuardError;
use guard::role::{Role, RoleRepository};
use guard::namespace::NamespaceRepository;

pub struct InMemoryRepository {
    /// Map with namespace as key
    roles: HashMap<String, Vec<Role>>,
    /// Map with namespace as key
    permissions: HashMap<String, Vec<Permission>>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        InMemoryRepository {
            permissions: HashMap::new(),
            roles: HashMap::new(),
        }
    }
}

impl InMemoryRepository {
    fn get_permissions(&mut self, namespace: &str) -> Result<&mut Vec<Permission>, GuardError> {
        self.permissions.get_mut(namespace)
            .ok_or(GuardError::PermissionError("Namespace not found.".to_owned()))
    }
}

#[async_trait]
impl RoleRepository for InMemoryRepository {
    async fn assign_role(&mut self, namespace: &str, role: &Role) -> Result<(), GuardError> {
        let roles = self.roles
            .entry(namespace.to_owned())
            .or_insert(Vec::new());

        if roles.contains(&role) {
            return Err(GuardError::RoleError("Role already added.".to_owned()));
        }

        roles.push(role.clone());
        Ok(())
    }

    async fn remove_role(&mut self, namespace: &str, role: &Role) -> Result<(), GuardError> {
        let roles = self.roles
            .entry(namespace.to_owned())
            .or_insert(Vec::new());

        if !roles.contains(&role) {
            return Err(GuardError::RoleError("Role wasn't found.".to_owned()));
        }
        roles.retain(|r| r != role);
        Ok(())
    }

    async fn list_roles(&self, namespace: &str, domain: &str, subject: &str) -> Result<Vec<Role>, GuardError> {
        let roles = self.roles.get(namespace)
            .ok_or(GuardError::RoleError("No namespace found.".to_owned()))?;

        Ok(
            roles.iter()
                .filter(|role| role.subject == subject && role.domain == domain)
                .cloned()
                .collect::<Vec<Role>>()
        )
    }
}

#[async_trait]
impl NamespaceRepository for InMemoryRepository {
    async fn create_namespace(&mut self, namespace: &str) -> Result<(), GuardError> {
        match self.roles.insert(namespace.to_owned(), Vec::new()) {
            None => Ok(()),
            Some(_) => Err(GuardError::NamespaceError("Namespace already exists.".to_owned()))
        }
    }

    async fn get_namespaces(&self) -> Result<Vec<String>, GuardError> {
        Ok(
            self.permissions
                .keys()
                .cloned()
                .collect::<Vec<String>>()
        )
    }
}

#[async_trait]
impl EnforceRepository for InMemoryRepository {
    async fn grant_permission(&mut self, namespace: &str, permission: &Permission) -> Result<(), GuardError> {
        let permissions = self.permissions.get_mut(namespace)
                .ok_or_else(|| GuardError::PermissionError("Namespace not found.".to_owned()))?;

        if permissions.contains(permission) {
            return Err(GuardError::PermissionError("Permission already exists.".to_owned()));
        }
        permissions.push(permission.clone());
        Ok(())
    }

    async fn remove_permission(&mut self, namespace: &str, permission: &Permission) -> Result<(), GuardError> {
        let permissions = self.get_permissions(namespace)?;

        if !permissions.contains(permission) {
            return Err(GuardError::PermissionError("No permission found.".to_owned()))
        }
        permissions.retain(|p| p != permission);
        Ok(())
    }

    async fn contains_permission(&mut self, namespace: &str, permission: &Permission) -> Result<bool, GuardError> {
        Ok(self.get_permissions(namespace)?.contains(permission))
    }

    async fn list_permissions(&mut self, namespace: &str) -> Result<Vec<Permission>, GuardError> {
        Ok(self.get_permissions(namespace)?
            .iter()
            .cloned()
            .collect::<Vec<Permission>>()
        )
    }
}

#[async_trait]
impl EnforceRepository for InMemoryRepository {
    async fn enforce(&self, request: &EnforceRequest) -> Result<bool, GuardError> {
        let roles = self.roles
            .get(&request.namespace)
            .ok_or_else(|| GuardError::EnforceError("Namespace not found.".to_owned()))?
            .iter()
            .filter(|role| {
                let role = *role;
                role.subject.as_str() == request.subject.as_str() && (role.domain.as_str() == "*" || role.domain.as_str() == request.domain.as_str())
            })
            .map(|role| role.role.clone())
            .collect::<Vec<String>>();

        let permissions = self.permissions
            .get(&request.namespace)
            .ok_or_else(|| GuardError::EnforceError("Namespace not found.".to_owned()))?;

        Ok(permissions
            .iter()
            .find(|permission| {
                let permission = *permission;
                roles.contains(&permission.role)
                    && (permission.domain.as_str() == "*" || permission.domain.as_str() == &request.domain)
                    && permission.object.as_str() == &request.object
                    && permission.action.as_str() == &request.action
            })
            .is_some()
        )
    }
}
