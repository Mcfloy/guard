use std::collections::HashMap;

use poem_openapi::Object;

// TODO: Convert into a utility method
/// Object that is composing a {Links}
#[derive(Object)]
pub struct Link {
    pub href: String,
    pub method: String,
    pub title: String
}

impl Link {
    pub fn new(href: &str, method: &str, title: &str) -> Self {
        Self {
            href: href.to_owned(),
            method: method.to_owned(),
            title: title.to_owned()
        }
    }
}

#[derive(Object)]
pub struct Links {
    pub links: HashMap<String, Link>
}

impl Links {
    pub fn new() -> Self {
        Self {
            links: HashMap::new()
        }
    }

    pub fn push(&mut self, rel: &str, link: Link) {
        self.links.insert(rel.to_owned(), link);
    }
}

impl Links {
    pub fn to_header(self) -> String {
        let mut value: Vec<String> = vec![];
        for (rel, link) in self.links {
            value.push(
                format!(
                    "<{}>; rel=\"{}\"; method=\"{}\"; title=\"{}\"",
                    link.href, rel, link.method, link.title
                )
            );
        }
        value.join(",").to_owned()
    }
}
