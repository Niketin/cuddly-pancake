use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Template)]
#[template(path = "packages.html")]
struct PackagesTemplate {
    packages: Vec<Package>,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

struct Package {
    name: String,
}

#[get("/packages")]
fn packages_service() -> impl Responder {
    let packages = get_packages_vec();

    let template = PackagesTemplate { packages }.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(template)
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

fn read_package_from_file(lines: &mut Lines<BufReader<File>>) -> Result<Package, &'static str> {
    let mut name: String = String::from("");

    // Read a paragraph
    let mut package_field_read = false;
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
            return Err("No field 'Package' in this paragrpah");
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
        }
    }

    return Ok(Package { name });
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(packages_service))
        .bind("127.0.0.1:8080")?
        .run()
}
