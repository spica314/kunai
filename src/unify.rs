use std::collections::BTreeMap;
use crate::config::*;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::fs::read_to_string;

pub fn unify_code(s: &str) -> String {
    let mut btree = BTreeMap::new();
    let mut queue = VecDeque::new();
    let config = read_config();
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
    for (_, unified) in &btree {
        res.push_str(unified);
    }
    res.push_str(s);
    res
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
    }
    (res, deps)
}
