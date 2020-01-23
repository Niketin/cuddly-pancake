use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::rc::Rc;

pub struct AppData {
    pub packages_vec: Vec<Rc<RefCell<Package>>>,
    pub packages_hashmap: HashMap<String, Rc<RefCell<Package>>>,
}

impl AppData {
    pub fn new_from_path(path: &str) -> Self {
        let packages_hashmap = get_packages_hashmap(path);
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
    pub reverse_dependencies: Vec<Rc<RefCell<Package>>>,
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

pub fn get_packages_hashmap(path: &str) -> HashMap<String, Rc<RefCell<Package>>> {
    let mut map: HashMap<String, Rc<RefCell<Package>>> = HashMap::new();
    let mut lines = get_lines_from_file(path);

    loop {
        if let Ok((package_from_file, dependencies_strings)) = read_package_from_file(&mut lines) {
            let package: Rc<RefCell<Package>> = if let Some(p) = map.get(&package_from_file.name) {
                p.borrow_mut().installed = true;
                p.borrow_mut().description = package_from_file.description.clone();
                p.clone()
            } else {
                let new_package = Rc::new(RefCell::new(package_from_file));
                let key = String::from(&new_package.borrow().name);
                map.insert(key, new_package.clone());
                new_package
            };

            let mut dependencies_packages: Vec<Vec<Rc<RefCell<Package>>>> = vec![];
            for alternatives_strings in dependencies_strings {
                let mut alternative_packages: Vec<Rc<RefCell<Package>>> = vec![];
                for alternative_string in alternatives_strings {
                    if map.contains_key(&alternative_string) {
                        if let Some(alternative_package) = map.get(&alternative_string) {
                            let alternative_package_clone = alternative_package.clone();
                            alternative_package_clone
                                .borrow_mut()
                                .reverse_dependencies
                                .push(package.clone());
                            alternative_packages.push(alternative_package_clone);
                        }
                    } else {
                        let alternative_package = Rc::new(RefCell::new(Package {
                            name: String::from(&alternative_string),
                            description: vec![],
                            dependencies: vec![],
                            reverse_dependencies: vec![package.clone()],
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

            package.borrow_mut().dependencies = dependencies_packages;
        } else {
            break;
        }
    }
    return map;
}

fn get_lines_from_file(path: &str) -> Lines<BufReader<File>> {
    if let Ok(f) = File::open(path) {
        let file = BufReader::new(f);
        let lines = file.lines();
        return lines;
    } else {
        panic!("Could not read file from path: {}", path)
    }
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
    let mut lines_peekable = lines.peekable();
    if let Some(_) = lines_peekable.peek() {
    } else {
        return Err("There is nothing to read anymore");
    }
    for line in lines_peekable {
        let l: String = line.unwrap();
        if l == "" && package_field_read {
            // End of the paragraph
            break;
        }

        if l == "" && !package_field_read {
            return Err("No field 'Package' in this paragraph");
        }

        if l.starts_with("Package: ") {
            l.split_ascii_whitespace()
                .enumerate()
                .filter(|&(i, _)| i == 1)
                .for_each(|(_, v)| name = String::from(v));
            package_field_read = true;
            continue;
        } else if l.starts_with("Description: ") {
            l.split_ascii_whitespace()
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
            reverse_dependencies: vec![],
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
