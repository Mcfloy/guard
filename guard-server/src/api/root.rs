use poem::{Request, Result};
use poem::i18n::Locale;
use poem_openapi::{ApiResponse, OpenApi};
use crate::api::jwt::extract_jwt;

use crate::links::{Link, Links};

pub struct RootApi;

#[derive(ApiResponse)]
enum RootResponse {
    #[oai(status = 200)]
    Ok(
        #[oai(header = "Link")] String
    )
}

#[OpenApi]
impl RootApi {
    #[oai(path = "/", method = "head")]
    async fn get_root_info(&self,
                           req: &Request,
                           locale: Locale,
    ) -> Result<RootResponse> {
        let user = extract_jwt(req);
        match user {
            None => Ok(RootResponse::Ok(not_connected_links(locale))),
            Some(user) => Ok(RootResponse::Ok(connected_links(&user.sub, locale)))
        }
    }
}

fn not_connected_links(locale: Locale) -> String {
    let mut links = Links::new();

    let title = locale.text("login").unwrap_or("".to_owned());
    links.push("login", Link::new("/","GET", &title));

    links.to_header()
}

fn connected_links(user_subject: &str, locale: Locale) -> String {
    let mut links = Links::new();

    let title = locale.text("permissions").unwrap_or("".to_owned());
    links.push("permissions", Link::new("/permissions", "HEAD", &title));

    links.to_header()
}
