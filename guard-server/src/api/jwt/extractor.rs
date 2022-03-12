use poem::Request;
use poem_openapi::auth::Bearer;
use poem_openapi::SecurityScheme;

use guard::jwt::{decode, Principal};

const AUTHORIZATION: &'static str = "Authorization";
const BEARER_PREFIX: &'static str = "Bearer ";

#[derive(SecurityScheme)]
#[oai(type = "bearer", checker = "token_checker")]
pub struct AuthenticatedUser(pub Principal);

pub async fn token_checker(_: &Request, bearer: Bearer) -> Option<Principal> {
    decode(&bearer.token).ok().map(|p| p.into())
}

pub fn extract_jwt(req: &Request) -> Option<Principal> {
    if let Some(auth) = req.headers().get(AUTHORIZATION) {
        if let Ok(auth) = auth.to_str() {
            if auth.starts_with(BEARER_PREFIX) {
                return decode(&auth.replace(BEARER_PREFIX, "").trim()).ok()
            }
        }
    }
    None
}
