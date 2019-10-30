use std::collections::BTreeSet;
use crate::config::*;
use std::path::PathBuf;
use std::fs::read_to_string;

pub fn unify(bin_name: &Option<&str>, rust_2018: bool) -> String {
    if bin_name.is_none() {
        unimplemented!();
    }
    let config = read_config();
    let mut pathbuf = cargo_edit::find(&None).unwrap();
    pathbuf.pop();
    pathbuf.push("src");
    pathbuf.push("bin");
    pathbuf.push(&format!("{}.rs", bin_name.unwrap()));
    eprintln!("pathbuf = {:?}", pathbuf);
    let code = read_to_string(&pathbuf).unwrap();
    unify_code2(&config, rust_2018, &code, &pathbuf)
}

fn unify_code2(config: &Config, rust_2018: bool, s: &str, path: &PathBuf) -> String {
    let mut expanded = BTreeSet::new();
    let mut macro_use = BTreeSet::new();
    let mut crate_texts = vec![];
    dfs(config, rust_2018, s, &mut expanded, &mut macro_use, &mut crate_texts, "", path);
    let mut res = String::new();
    let mut expanded_flag = false;
    for line in crate_texts[crate_texts.len()-1].1.lines() {
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
        }
        else {
            res.push_str(line);
            res.push('\n');
        }
    }
    eprintln!("prev");
    eprintln!("{}", res);
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
    crate_texts: &mut Vec<(String,String)>, 
    my_name: &str,
    my_path: &PathBuf,
) {
    if expanded.contains(my_name) {
        return;
    }
    let mut flag_macro_use = false;
    let mut res = String::new();
    for line in s.lines() {
        if line.starts_with("use ") {
            let crate_name: String = line.chars().skip(4).take_while(|&c| c != ':' && c != ';').collect();
            if crate_name == "std" || crate_name == "core" || crate_name == "crate" {
                res.push_str(line);
                res.push('\n');
                continue;
            }
            let mut pathbuf = config.crate_path(&crate_name).unwrap();
            pathbuf.push("src");
            pathbuf.push("lib.rs");
            let code = read_to_string(&pathbuf).unwrap();
            dfs(config, rust_2018, &code, expanded, macro_use, crate_texts, &crate_name, &pathbuf);
            if rust_2018 {
                let line2 = line.replace("use ", "use crate::");
                res.push_str(&line2);
                res.push('\n');
            }
            else {
                let line2 = line.replace("use ", "use ::");
                res.push_str(&line2);
                res.push('\n');
            }
            flag_macro_use = false;
        }
        else if line.starts_with("#[macro_use]") {
            flag_macro_use = true;
        }
        else if line.starts_with("extern crate ") {
            let crate_name: String = line.chars().skip(13).take_while(|&c| c != ';').collect();
            if crate_name == "std" || crate_name == "core" || crate_name == "crate" {
                panic!("unable to expand '{}'", line);
            }
            let mut pathbuf = config.crate_path(&crate_name).unwrap();
            pathbuf.push("src");
            pathbuf.push("lib.rs");
            let code = read_to_string(&pathbuf).unwrap();
            dfs(config, rust_2018, &code, expanded, macro_use, crate_texts, &crate_name, &pathbuf);
            if flag_macro_use {
                macro_use.insert(crate_name.clone());
            }
            flag_macro_use = false;
        }
        else {
            res.push_str(line);
            res.push('\n');
        }
    }
    expanded.insert(my_name.to_string());
    crate_texts.push((my_name.to_string(), res));
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
    let res = unify_code2(&config, false, &s, &PathBuf::new());
    let right = r#"mod test_crate {
    pub fn test_function(x: i64) -> i64 {
        x + 1
    }
}

use test_crate::test_function;
fn main() {
    println!("test");
    assert_eq!(test_function(1), 2);
}
"#;
    assert_eq!(res, right);
}
