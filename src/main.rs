use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use askama::Template;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Template)]
#[template(path = "packages.html")]
struct PackagesTemplate {
    packages: Vec<PackageWithUrl>,
}

#[derive(Template)]
#[template(path = "package.html")]
struct PackageTemplate<'a> {
    package: &'a Package,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

struct Package {
    name: String,
    description: Vec<String>,
}

struct PackageWithUrl {
    package: Package,
    url: String,
}

fn packages_service(req: HttpRequest) -> impl Responder {
    let mut packages = get_packages_vec();
    let packages = packages
        .drain(..)
        .map(|package| {
            let name = package.name.clone();
            let res = if let Ok(url) = req.url_for("package", &[name]) {
                url.into_string()
            } else {
                String::from("")
            };

            PackageWithUrl {
                url: res,
                package: package,
            }
        })
        .collect();

    let template = PackagesTemplate { packages }.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(template)
}

fn package_service(info: web::Path<String>) -> impl Responder {
    let packages = get_packages_map();

    let package_name = info.as_ref();
    if let Some(package) = packages.get(package_name) {
        let template = PackageTemplate { package }.render().unwrap();
        return HttpResponse::Ok().content_type("text/html").body(template);
    } else {
        return HttpResponse::NotFound().content_type("text/html").finish();
    }
}

fn get_lines_from_file(path: &str) -> Lines<BufReader<File>> {
    let f = File::open(path).unwrap();
    let file = BufReader::new(f);
    let lines = file.lines();
    return lines;
}

fn get_packages_vec() -> Vec<Package> {
    let mut packages = vec![];
    let mut lines = get_lines_from_file("status.real");
    loop {
        if let Ok(package) = read_package_from_file(&mut lines) {
            packages.push(package);
        } else {
            break;
        }
    }
    packages.sort_by(|a, b| a.name.cmp(&b.name));
    return packages;
}

fn get_packages_map() -> HashMap<String, Package> {
    let mut packages = HashMap::new();
    let mut lines = get_lines_from_file("status.real");
    loop {
        if let Ok(package) = read_package_from_file(&mut lines) {
            packages.insert(String::from(package.name.clone()), package);
        } else {
            break;
        }
    }
    return packages;
}

fn read_package_from_file(lines: &mut Lines<BufReader<File>>) -> Result<Package, &'static str> {
    let mut name: String = String::from("");
    let mut description: Vec<String> = vec![];

    // Read a paragraph
    let mut package_field_read = false;
    let mut currently_reading_description = false;
    let mut llines = lines.peekable();
    if let Some(_) = llines.peek() {
    } else {
        return Err("There is nothing to read anymore");
    }
    for (i, line) in llines.enumerate() {
        let l: String = line.unwrap();
        if l == "" && package_field_read {
            // End of paragraph
            break;
        }

        if l == "" && !package_field_read {
            return Err("No field 'Package' in this paragraph");
        }

        if i == 0 && !l.starts_with("Package: ") {
            return Err("First line did not have field 'Package'");
        }

        let split_iter = l.split_ascii_whitespace();
        if i == 0 {
            split_iter
                .enumerate()
                .filter(|&(i, _)| i == 1)
                .for_each(|(_, v)| name = String::from(v));
            package_field_read = true;
            continue;
        } else if l.starts_with("Description: ") {
            split_iter
                .enumerate()
                .filter(|&(i, _)| i == 1)
                .for_each(|(_, v)| description.push(String::from(v)));
            currently_reading_description = true;
            continue;
        }

        if currently_reading_description && l.starts_with(" ") {
            description.push(String::from(l.trim_start()));
        } else if currently_reading_description && !l.starts_with(" ") {
            currently_reading_description = false;
        }
    }
    return Ok(Package { name, description });
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/packages")
                .service(
                    web::resource("/{package_name}")
                        .name("package")
                        .route(web::get().to(package_service)),
                )
                .service(
                    web::resource("")
                        .name("packages")
                        .route(web::get().to(packages_service)),
                ),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
}
