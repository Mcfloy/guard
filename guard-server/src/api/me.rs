use crate::api::jwt::AuthenticatedUser;
use poem::{Result};
use poem_openapi::{OpenApi, ApiResponse};
use poem_openapi::payload::Json;

pub struct MeApi;

#[derive(ApiResponse)]
enum MeResponse {
    #[oai(status = 200)]
    Ok(Json<String>)
}

impl Into<MeResponse> for String {
    fn into(self) -> MeResponse {
        MeResponse::Ok(Json(self))
    }
}

#[OpenApi]
impl MeApi {
    #[oai(path = "/me_mod", method = "get")]
    async fn get_me(&self,
                    user: AuthenticatedUser,
    ) -> Result<MeResponse> {
        Ok(user.0.sub.into())
    }
}