use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use guard::jwt::Principal;

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct User {
    pub sub: String,
    pub namespace: String,
    pub exp: u64
}

impl Into<User> for Principal {
    fn into(self) -> User {
        User {
            sub: self.sub,
            namespace: self.namespace,
            exp: self.exp
        }
    }
}
