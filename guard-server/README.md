# Guard Server

## Universal rules

- no namespace
- no domain specified

| Actions               | Owner | Manager | Member |
|-----------------------|:-----:|:-------:|:------:|
| List namespaces       |  ✔️   |   ✔️    |   ✔️   |
| List owned namespaces |  ✔️   |   ✔️    |   ✔️   |

## Guard scoped rules
- namespace: `guard`
- no domain specified

| Actions               | Owner | Manager | Member |
|-----------------------|:-----:|:-------:|:------:|
| Add a new namespace   |  ✔️️  |         |        |
| Manage a namespace    |  ✔️   |   ✔️    |        |
| Remove a namespace    |  ✔️   |   ✔️    |        |

## Guard domain scoped rules
- namespace: `guard`
- domain specified

| Actions          | Owner | Manager | Member |
|------------------|:-----:|:-------:|:------:|
| List roles       |  ✔️   |   ✔️    |   ️    |
| Assign role      |  ✔️   |   ✔️    |   ️    |
| Remove role      |  ✔️   |   ✔️    |   ️    |

## Namespace scoped rules

- namespace: not `guard`
- domain optionally specified

| Actions          | Owner | Manager | Member |
|------------------|:-----:|:-------:|:------:|
| List owned roles |  ✔️   |   ✔️    |   ✔️   |

```mermaid
flowchart TB
    Root["HEAD /"]
    PermissionsInfo["HEAD /permissions"]
    PermissionsGrant["POST /permissions"]
    PermissionsRemove["DELETE /permissions"]
    NamespacesInfo["HEAD /namespaces"]
    NamespacesList["GET /namespaces"]
    NamespacesCreate["POST /namespaces"]
    NamespacesDelete["DELETE /namespaces/{id}"]
    NamespacesId["GET /namespaces/{id}"]
    NamespacesRoles["GET /namespaces/{id}/roles"]
    NamespacesDeleteRoles["DELETE /namespaces/{id}/roles/{name}"]
    MeInfo["GET /me"]
    MeRoles["GET /me/roles"]
    MeNamespaces["GET /me/namespaces"]
    MeInfo --> MeRoles
    MeInfo --> MeNamespaces
    Root --> MeInfo
    NamespacesId --> NamespacesDelete
    NamespacesId --> NamespacesDeleteRoles
    NamespacesId --> NamespacesRoles
    NamespacesList --> NamespacesId
    NamespacesInfo --> NamespacesList
    NamespacesInfo --> NamespacesCreate
    Root --> NamespacesInfo
    PermissionsInfo --> PermissionsRemove
    PermissionsInfo --> PermissionsGrant
    Root --> PermissionsInfo
```
