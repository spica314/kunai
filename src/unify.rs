use std::collections::BTreeMap;
use crate::config::*;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::fs::read_to_string;

pub fn unify_code(config: &Config, s: &str) -> String {
    let mut btree = BTreeMap::new();
    let mut queue = VecDeque::new();
    for line in s.lines() {
        if line.starts_with("use ") {
            let crate_name: String = line.chars().skip(4).take_while(|&c| c != ':' && c != ';').collect();
            if crate_name == "std" || crate_name == "core" {
                continue;
            }
            queue.push_back(crate_name);
        }
    }
    while let Some(crate_name) = queue.pop_front() {
        if btree.contains_key(&crate_name) {
            continue;
        }
        let (unified, deps) = unify_crate(&config, &crate_name);
        btree.insert(crate_name, unified);
        for d in deps {
            queue.push_back(d);
        }
    }
    let mut res = String::new();
    for (crate_name, unified) in &btree {
        res.push_str(&format!("mod {} {{\n", crate_name));
        res.push_str(unified);
        res.push_str("}\n")
    }
    res.push_str(s);
    normalize(&res)
}

fn unify_crate(config: &Config, crate_name: &str) -> (String, Vec<String>) {
    let mut pathbuf = config.crates.as_ref().unwrap().get(crate_name).unwrap().path.clone();
    pathbuf.push("src");
    pathbuf.push("lib.rs");
    let s = read_to_string(&pathbuf).unwrap();
    let mut res = String::new();
    let mut deps = vec![];
    for line in s.lines() {
        if line.starts_with("use ") {
            let crate_name: String = line.chars().skip(4).take_while(|&c| c != ':' && c != ';').collect();
            if crate_name == "std" || crate_name == "core" {
                continue;
            }
            deps.push(crate_name);
        }
        res.push_str(line);
        res.push('\n');
    }
    (res, deps)
}

pub fn normalize(s: &str) -> String {
    let mut res = String::new();
    let mut prev_is_empty_line = true;
    for line in s.lines() {
        if line.len() == 0 {
            if ! prev_is_empty_line {
                res.push_str(line);
                res.push('\n');
                prev_is_empty_line = true;
            }
        }
        else {
            res.push_str(line);
            res.push('\n');
        }
    }
    res
}

#[test]
fn test_unify() {
    use std::collections::BTreeMap;
    use std::str::FromStr;
    let s = r#"
use test_crate::test_function;
fn main() {
    println!("test");
    assert_eq!(test_function(1), 2);
}"#;
    let mut btree = BTreeMap::new();
    btree.insert("test_crate".to_string(), CrateInfo { path: PathBuf::from_str("test_data/test-crate").unwrap() });
    let config = Config {
        crates: Some(btree),
    };
    let res = unify_code(&config, &s);
    let right = r#"
mod test_crate {
pub fn test_function(x: i64) -> i64 {
    x + 1
}
}
use test_crate::test_function;
fn main() {
    println!("test");
    assert_eq!(test_function(1), 2);
}"#;
    assert_eq!(res, normalize(right));
}
