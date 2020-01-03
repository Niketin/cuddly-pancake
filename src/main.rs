use actix_web::{web, App, HttpServer};
use std::env;

mod common;
mod routes;

fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
        .expect("PORT variable must be set")
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .data(common::AppData::default())
            .service(
                web::resource("/")
                    .name("packages")
                    .route(web::get().to(routes::packages_handler)),
            )
            .service(
                web::resource("/package/{package_name}")
                    .name("package")
                    .route(web::get().to(routes::package_handler)),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
}
