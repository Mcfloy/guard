use poem::Request;
use guard::jwt::{decode, Principal};

const AUTHORIZATION: &'static str = "Authorization";
const BEARER_PREFIX: &'static str = "Bearer ";

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
