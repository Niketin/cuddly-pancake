use crate::common::*;
use crate::web::Data;
use actix_web::{web, HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "package.html")]
struct PackageTemplate<'a> {
    package: &'a Package,
}

pub fn package_handler(info: web::Path<String>, data: Data<AppData>) -> impl Responder {
    let packages = data.packages_map.borrow();

    let package_name = info.as_ref();
    if let Some(package) = packages.get(package_name) {
        let template = PackageTemplate { package }.render().unwrap();
        return HttpResponse::Ok().content_type("text/html").body(template);
    } else {
        return HttpResponse::NotFound().content_type("text/html").finish();
    }
}
