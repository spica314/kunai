use crate::config::*;
use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::path::PathBuf;
use crate::cargo_wrapper;

pub fn unify(bin_name: &Option<&str>, rust_2018: bool) -> String {
    if bin_name.is_none() {
        unimplemented!();
    }
    let config = read_config();
    let mut pathbuf = cargo_wrapper::manifest_path().unwrap();
    pathbuf.pop();
    pathbuf.push("src");
    pathbuf.push("bin");
    pathbuf.push(&format!("{}.rs", bin_name.unwrap()));
    eprintln!("pathbuf = {:?}", pathbuf);
    let code = read_to_string(&pathbuf).unwrap();
    unify_code(&config, rust_2018, &code, &pathbuf)
}

pub fn unify_code(config: &Config, rust_2018: bool, s: &str, path: &PathBuf) -> String {
    let mut expanded = BTreeSet::new();
    let mut macro_use = BTreeSet::new();
    let mut crate_texts = vec![];
    let mut buf = String::new();
    dfs(
        config,
        rust_2018,
        s,
        &mut expanded,
        &mut macro_use,
        &mut crate_texts,
        path,
        "",
        &mut buf,
    );
    let mut res = String::new();
    let mut expanded_flag = false;
    for line in buf.lines() {
        if !expanded_flag && line.starts_with("use") {
            for (crate_name, text) in &crate_texts {
                if crate_name == "" {
                    continue;
                }
                if macro_use.contains(crate_name) {
                    res.push_str("#[macro_use]");
                    res.push('\n');
                }
                res.push_str(&format!("mod {} {{", crate_name));
                res.push('\n');
                res.push_str(text);
                res.push_str("}");
                res.push('\n');
                res.push('\n');
            }
            res.push_str(line);
            res.push('\n');
            expanded_flag = true;
        } else {
            res.push_str(line);
            res.push('\n');
        }
    }
    let mut buf = vec![];
    let mut config = rustfmt_nightly::Config::default();
    config.set().emit_mode(rustfmt_nightly::EmitMode::Stdout);
    config.set().verbose(rustfmt_nightly::Verbosity::Quiet);
    let mut session = rustfmt_nightly::Session::new(config, Some(&mut buf));
    session.format(rustfmt_nightly::Input::Text(res)).ok();
    drop(session);
    std::str::from_utf8(&buf).unwrap().to_string()
}

fn dfs(
    config: &Config,
    rust_2018: bool,
    s: &str,
    expanded: &mut BTreeSet<String>,
    macro_use: &mut BTreeSet<String>,
    crate_texts: &mut Vec<(String, String)>,
    my_path: &PathBuf,
    my_crate_name: &str,
    res: &mut String,
) {
    let mut flag_macro_use = false;
    // let mut res = String::new();
    for line in s.lines() {
        if line.starts_with("use ") {
            let crate_name: String = line
                .chars()
                .skip(4)
                .take_while(|&c| c != ':' && c != ';')
                .collect();
            if crate_name == "std" || crate_name == "core" {
                res.push_str(line);
                res.push('\n');
                continue;
            }
            if crate_name == "crate" {
                if my_crate_name == "" {
                    res.push_str(line);
                    res.push('\n');
                }
                else {
                    if rust_2018 {
                        let line2 = line.replace("crate", &format!("crate::{}", my_crate_name));
                        res.push_str(&line2);
                        res.push('\n');
                    } else {
                        let line2 = res.replace("crate", &format!("::{}", my_crate_name));
                        res.push_str(&line2);
                        res.push('\n');
                    }
                }
                continue;
            }
            let manifest = cargo_wrapper::manifest_from_path(my_path.as_path()).unwrap();
            let pathbuf = if my_crate_name == "" && crate_name == cargo_wrapper::crate_name(&manifest) {
                let mut pathbuf = cargo_wrapper::manifest_path().unwrap();
                pathbuf.pop();
                pathbuf.push("src");
                pathbuf.push("lib.rs");
                pathbuf
            }
            else {
                let deps = cargo_wrapper::dependency_paths(&manifest);
                let mut pathbuf = deps.get(&crate_name).unwrap().clone();
                pathbuf.push("src");
                pathbuf.push("lib.rs");
                pathbuf
            };
            let code = read_to_string(&pathbuf).unwrap();
            let mut buf = String::new();
            if !expanded.contains(&crate_name) {
                dfs(
                    config,
                    rust_2018,
                    &code,
                    expanded,
                    macro_use,
                    crate_texts,
                    &pathbuf,
                    &crate_name,
                    &mut buf,
                );
                expanded.insert(crate_name.to_string());
                crate_texts.push((crate_name.to_string(), buf));
            }
            if rust_2018 {
                let line2 = line.replace("use ", "use crate::");
                res.push_str(&line2);
                res.push('\n');
            } else {
                let line2 = line.replace("use ", "use ::");
                res.push_str(&line2);
                res.push('\n');
            }
            flag_macro_use = false;
        } else if line.starts_with("pub mod ") {
            let mod_name: String = line.chars().skip(8).take_while(|&c| c != ';').collect();
            let mut pathbuf = my_path.clone();
            pathbuf.pop();
            pathbuf.push(&format!("{}.rs", mod_name));
            let code = read_to_string(&pathbuf).unwrap();
            res.push_str(&format!("pub mod {} {{", mod_name));
            res.push('\n');
            dfs(
                config,
                rust_2018,
                &code,
                expanded,
                macro_use,
                crate_texts,
                &pathbuf,
                my_crate_name,
                res,
            );
            res.push_str("}");
            res.push('\n');
        } else if line.starts_with("#[macro_use]") {
            flag_macro_use = true;
        } else if line.starts_with("extern crate ") {
            let crate_name: String = line.chars().skip(13).take_while(|&c| c != ';').collect();
            if crate_name == "std" || crate_name == "core" || crate_name == "crate" {
                panic!("unable to expand '{}'", line);
            }
            let manifest = cargo_wrapper::manifest_from_path(my_path.as_path()).unwrap();
            let deps = cargo_wrapper::dependency_paths(&manifest);
            let mut pathbuf = deps.get(&crate_name).unwrap().clone();
            pathbuf.push("src");
            pathbuf.push("lib.rs");
            let code = read_to_string(&pathbuf).unwrap();
            let mut buf = String::new();
            if !expanded.contains(&crate_name) {
                dfs(
                    config,
                    rust_2018,
                    &code,
                    expanded,
                    macro_use,
                    crate_texts,
                    &pathbuf,
                    &crate_name,
                    &mut buf,
                );
                expanded.insert(crate_name.to_string());
                crate_texts.push((crate_name.to_string(), buf));
            }
            if flag_macro_use {
                macro_use.insert(crate_name.clone());
            }
            flag_macro_use = false;
        } else {
            res.push_str(line);
            res.push('\n');
        }
    }
    // expanded.insert(my_name.to_string());
    // crate_texts.push((my_name.to_string(), res));
}
