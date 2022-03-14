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
    NamespacesInfo["HEAD /namespaces"]
    NamespacesList["GET /namespaces"]
    NamespacesCreate["POST /namespaces"]
    NamespacesDelete["DELETE /namespaces/{id}"]
    NamespacesId["HEAD /namespaces/{id}"]
    NamespacesRoles["GET /namespaces/{id}/roles"]
    NamespacesCreateRoles["CREATE /namespaces/{id}/roles"]
    NamespacesDeleteRoles["DELETE /namespaces/{id}/roles"]
    NamespacesCreatePermissions["CREATE /namespaces/{id}/permissions"]
    NamespacesDeletePermissions["DELETE /namespaces/{id}/permissions"]
    MeInfo["GET /me"]
    MeRoles["GET /me/roles"]
    MeNamespaces["GET /me/namespaces"]
    MeInfo --> MeRoles
    MeInfo --> MeNamespaces
    Root --> MeInfo
    NamespacesId --> NamespacesDelete
    NamespacesId --> NamespacesDeleteRoles
    NamespacesId --> NamespacesCreateRoles
    NamespacesId --> NamespacesDeletePermissions
    NamespacesId --> NamespacesCreatePermissions
    NamespacesId --> NamespacesRoles
    NamespacesList --> NamespacesId
    NamespacesInfo --> NamespacesList
    NamespacesInfo --> NamespacesCreate
    Root --> NamespacesInfo
```
