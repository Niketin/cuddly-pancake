use crate::common::*;
use crate::web::Data;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use askama::Template;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "package.html")]
struct PackageTemplate<'a> {
    package: &'a Package,
    urls: HashMap<String, Option<String>>,
}

pub fn package_handler(
    info: web::Path<String>,
    req: HttpRequest,
    data: Data<AppData>,
) -> impl Responder {
    let packages = data.packages_map.borrow();
    let package_name = info.as_ref();
    if let Some(package) = packages.get(package_name) {
        let mut urls: HashMap<String, Option<String>> = HashMap::new();
        for dep in &package.dependencies {
            for alt in dep {
                let url: Option<String> = if let Some(_) = packages.get(&alt[..]) {
                    if let Ok(url) = req.url_for("package", &[alt]) {
                        Some(url.into_string())
                    } else {
                        None
                    }
                } else {
                    None
                };
                urls.insert(alt.clone(), url);
            }
        }

        let template = PackageTemplate { package, urls }.render().unwrap();
        return HttpResponse::Ok().content_type("text/html").body(template);
    } else {
        return HttpResponse::NotFound().content_type("text/html").finish();
    }
}
