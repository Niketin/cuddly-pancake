use actix_web::{web, App, HttpServer};
use std::env;

mod common;
mod routes;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let default_path = String::from("/var/lib/dpkg/status");

    let arg_path: String = if args.get(1).is_some() {
        let path = String::from(args.get(1).unwrap());
        println!("Using path from argument: {}", &path);
        path
    } else {
        println!("Using the default path: {}", &default_path);
        default_path
    };
    let port: u16 = env::var("PORT")
        .expect("PORT variable must be set")
        .parse()
        .expect("PORT must be a number");

    println!("Starting server on port: {}", port);

    HttpServer::new(move || {
        App::new()
            .data(common::AppData::new_from_path(&arg_path[..]))
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
