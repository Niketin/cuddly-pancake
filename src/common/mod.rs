use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Default)]
pub struct Package {
    pub name: String,
    pub description: Vec<String>,
    pub url: Option<String>,
    pub dependencies: Vec<Vec<String>>,
}

pub fn get_packages_vec() -> Vec<Package> {
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

pub fn get_packages_map() -> HashMap<String, Package> {
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

fn get_lines_from_file(path: &str) -> Lines<BufReader<File>> {
    let f = File::open(path).unwrap();
    let file = BufReader::new(f);
    let lines = file.lines();
    return lines;
}

fn read_package_from_file(lines: &mut Lines<BufReader<File>>) -> Result<Package, &'static str> {
    let mut name: String = String::from("");
    let mut description: Vec<String> = vec![];
    let mut dependencies: Vec<Vec<String>> = vec![];

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
        if l.starts_with("Depends: ") {
            dependencies = parse_dependencies(l.trim_start_matches("Depends: "));
        }
    }
    return Ok(Package {
        name,
        description,
        url: None,
        dependencies,
    });
}

fn parse_dependencies(input: &str) -> Vec<Vec<String>> {
    let dependencies: Vec<&str> = input.split(",").collect();
    let mut results: Vec<Vec<String>> = vec![];
    for dep in dependencies {
        let alternatives: Vec<&str> = dep.split("|").collect();
        let alternatives_trimmed: Vec<String> = alternatives
            .into_iter()
            .map(|x| {
                if let Some(i) = x.find("(") {
                    &x[..i]
                } else {
                    &x[..]
                }
            })
            .map(|x| String::from(x.trim()))
            .collect();
        if alternatives_trimmed.len() > 0 {
            results.push(alternatives_trimmed);
        }
    }
    results
}
