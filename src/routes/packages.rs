use crate::common::*;
use crate::web::Data;
use actix_web::{HttpRequest, HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[derive(Template)]
#[template(path = "packages.html")]
struct PackagesTemplate<'a> {
    packages: Vec<&'a Package>,
}

pub fn packages_handler(req: HttpRequest, data: Data<AppData>) -> impl Responder {
    let mut packages = data.packages_vec.borrow_mut();

    for ref mut package in packages.iter_mut() {
        package.url = if let Ok(url) = req.url_for("package", &[&package.name]) {
            Some(url.into_string())
        } else {
            None
        };
    }
    let packages: Vec<&Package> = packages.iter().map(|x| x).collect();

    let template = PackagesTemplate { packages }.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(template)
}
