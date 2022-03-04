use poem_openapi::Object;

#[derive(Object)]
struct Link {

}

#[derive(Object)]
struct Links {
    links: Vec<Link>
}
