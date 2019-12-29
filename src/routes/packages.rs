use crate::common::*;
use actix_web::{HttpRequest, HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[derive(Template)]
#[template(path = "packages.html")]
struct PackagesTemplate {
    packages: Vec<Package>,
}

pub fn packages_handler(req: HttpRequest) -> impl Responder {
    let mut packages = get_packages_vec();
    let packages = packages
        .drain(..)
        .map(|mut package| {
            package.url = if let Ok(url) = req.url_for("package", &[&package.name]) {
                Some(url.into_string())
            } else {
                None
            };
            package
        })
        .collect();

    let template = PackagesTemplate { packages }.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(template)
}
