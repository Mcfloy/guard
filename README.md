# Guard

Centralized authorization for multiple projects. Inspired of casbin.

| Name           | Description                                      |
|----------------|--------------------------------------------------|
| guard          | core library that exposes the trait              |
| guard-grpc     | exposition of the enforce method through gRPC    |
| guard-postgres | implementation of Guard with postgres            |
| guard-server   | server that runs grpc and rest APIs to use Guard |
| guard-console  | executable to create permission for Guard        |

## TODO

* [ ] Roles
  * [ ] Add a role
  * [ ] Remove a role
  * [ ] Get roles of a user
* [x] Get Namespaces
* [ ] Get Roles from a namespace
* [x] Enforce a rule
