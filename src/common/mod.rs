use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::rc::Rc;

pub struct AppData {
    pub packages_vec: Vec<Rc<RefCell<Package>>>,
    pub packages_hashmap: HashMap<String, Rc<RefCell<Package>>>,
}

impl Default for AppData {
    fn default() -> Self {
        let packages_hashmap = get_packages_hashmap();
        let packages_vec = get_packages_vec(&packages_hashmap);
        Self {
            packages_vec,
            packages_hashmap,
        }
    }
}

#[derive(Default)]
pub struct Package {
    pub name: String,
    pub description: Vec<String>,
    pub dependencies: Vec<Vec<Rc<RefCell<Package>>>>,
    pub installed: bool,
}

pub fn get_packages_vec(
    packages: &HashMap<String, Rc<RefCell<Package>>>,
) -> Vec<Rc<RefCell<Package>>> {
    let mut packages: Vec<Rc<RefCell<Package>>> = packages
        .iter()
        .map(|(_, package)| package.clone())
        .collect();
    packages.sort_by(|a, b| a.borrow().name.cmp(&b.borrow().name));
    return packages;
}

pub fn get_packages_hashmap() -> HashMap<String, Rc<RefCell<Package>>> {
    let mut map: HashMap<String, Rc<RefCell<Package>>> = HashMap::new();
    let mut lines = get_lines_from_file("status.real");
    loop {
        if let Ok((package, dependencies_strings)) = read_package_from_file(&mut lines) {
            let mut dependencies_packages: Vec<Vec<Rc<RefCell<Package>>>> = vec![];
            for alternatives_strings in dependencies_strings {
                let mut alternative_packages: Vec<Rc<RefCell<Package>>> = vec![];
                for alternative_string in alternatives_strings {
                    if map.contains_key(&alternative_string) {
                        if let Some(alternative_package) = map.get(&alternative_string) {
                            alternative_packages.push(alternative_package.clone());
                        }
                    } else {
                        let alternative_package = Rc::new(RefCell::new(Package {
                            name: String::from(&alternative_string),
                            description: vec![],
                            dependencies: vec![],
                            installed: false,
                        }));
                        map.insert(
                            String::from(&alternative_string),
                            alternative_package.clone(),
                        );
                        alternative_packages.push(alternative_package);
                    }
                }
                dependencies_packages.push(alternative_packages);
            }
            if let Some(p) = map.get(&package.name) {
                p.borrow_mut().installed = true;
                p.borrow_mut().dependencies = dependencies_packages;
                p.borrow_mut().description = package.description.clone();
            } else {
                let new_package = Rc::new(RefCell::new(package));
                new_package.borrow_mut().dependencies = dependencies_packages;
                let key = String::from(&new_package.borrow().name);
                map.insert(key, new_package);
            }
        } else {
            break;
        }
    }
    return map;
}

fn get_lines_from_file(path: &str) -> Lines<BufReader<File>> {
    let f = File::open(path).unwrap();
    let file = BufReader::new(f);
    let lines = file.lines();
    return lines;
}

fn read_package_from_file(
    lines: &mut Lines<BufReader<File>>,
) -> Result<(Package, Vec<Vec<String>>), &'static str> {
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

    return Ok((
        Package {
            name,
            description,
            dependencies: vec![],
            installed: true,
        },
        dependencies,
    ));
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
