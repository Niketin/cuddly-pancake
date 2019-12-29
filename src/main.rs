use actix_web::{web, App, HttpServer};

mod common;
mod routes;

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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
    .bind("127.0.0.1:8080")?
    .run()
}
