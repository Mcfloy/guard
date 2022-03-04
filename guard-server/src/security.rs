use poem::Request;
use poem_openapi::SecurityScheme;
use poem_openapi::auth::Bearer;
use guard::jwt::decode;
use crate::user::User;

#[derive(SecurityScheme)]
#[oai(type = "bearer", checker = "token_checker")]
pub struct AuthenticatedUser(pub User);

pub async fn token_checker(_: &Request, bearer: Bearer) -> Option<User> {
    decode(&bearer.token).ok().map(|p| p.into())
}
