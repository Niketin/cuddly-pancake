use crate::common::*;
use crate::web::Data;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use askama::Template;
use std::cell::Ref;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "package.html")]
struct PackageTemplate<'a> {
    package: Ref<'a, Package>,
    urls: HashMap<String, Option<String>>,
}

pub fn package_handler(
    info: web::Path<String>,
    req: HttpRequest,
    data: Data<AppData>,
) -> impl Responder {
    let packages = &data.packages_hashmap;
    let package_name = info.as_ref();
    if let Some(package) = packages.get(package_name) {
        let mut urls: HashMap<String, Option<String>> = HashMap::new();
        for dep in &package.borrow().dependencies {
            for alt in dep {
                let url: Option<String> = if alt.borrow().installed {
                    if let Ok(url) = req.url_for("package", &[&alt.borrow().name]) {
                        Some(url.into_string())
                    } else {
                        None
                    }
                } else {
                    None
                };
                urls.insert(alt.borrow().name.clone(), url);
            }
        }

        for rev_dep in &package.borrow().reverse_dependencies {
            let url: Option<String> = if rev_dep.borrow().installed {
                if let Ok(url) = req.url_for("package", &[&rev_dep.borrow().name]) {
                    Some(url.into_string())
                } else {
                    None
                }
            } else {
                None
            };
            urls.insert(rev_dep.borrow().name.clone(), url);
        }

        let template = PackageTemplate {
            package: package.clone().borrow(),
            urls,
        }
        .render()
        .unwrap();
        return HttpResponse::Ok().content_type("text/html").body(template);
    } else {
        return HttpResponse::NotFound().content_type("text/html").finish();
    }
}
