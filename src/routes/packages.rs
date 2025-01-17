use crate::common::*;
use crate::web::Data;
use actix_web::{HttpRequest, HttpResponse, Responder};
use askama::Template;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[derive(Template)]
#[template(path = "packages.html")]
struct PackagesTemplate<'a> {
    packages: &'a Vec<Rc<RefCell<Package>>>,
    urls: Vec<Option<String>>,
}

pub fn packages_handler(req: HttpRequest, data: Data<AppData>) -> impl Responder {
    let packages = &data.packages_vec;
    let mut urls = vec![];
    for package in packages.iter() {
        if let Ok(url) = req.url_for("package", &[&package.borrow().name]) {
            urls.push(Some(url.into_string()));
        } else {
            urls.push(None)
        };
    }

    let template = PackagesTemplate { packages, urls }.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(template)
}
