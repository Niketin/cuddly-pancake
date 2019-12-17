use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "packages.html")]
struct PackagesTemplate<'a> {
    packages: Vec<Package<'a>>,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

struct Package<'a> {
    name: &'a str,
}

#[get("/packages")]
fn packages_service(info: web::Path<(u32, String)>) -> impl Responder {
    let p1 = Package {
        name: &info.1,
    };

    let p2 = Package {
        name: &info.1,
    };

    let template = PackagesTemplate {
        packages: vec![p1, p2],
    }
    .render()
    .unwrap();
    HttpResponse::Ok().content_type("text/html").body(template)
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(packages_service)
    })
    .bind("127.0.0.1:8080")?
    .run()
}
