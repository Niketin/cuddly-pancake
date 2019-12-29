use actix_web::{web, HttpResponse, Responder};
use askama::Template;

use crate::common::*;

#[derive(Template)]
#[template(path = "package.html")]
struct PackageTemplate<'a> {
    package: &'a Package,
}

pub fn package_handler(info: web::Path<String>) -> impl Responder {
    let packages = get_packages_map();

    let package_name = info.as_ref();
    if let Some(package) = packages.get(package_name) {
        let template = PackageTemplate { package }.render().unwrap();
        return HttpResponse::Ok().content_type("text/html").body(template);
    } else {
        return HttpResponse::NotFound().content_type("text/html").finish();
    }
}
