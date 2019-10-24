use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Config {
    crates: Option<BTreeMap<String, CrateInfo>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CrateInfo {
    path: PathBuf,
}

pub fn read_config() -> Config {
    let mut path = config_dir().unwrap();
    path.push("kunai");
    path.push("config.toml");
    let s = read_to_string(&path).unwrap();
    parse_config(&s)
}

fn parse_config(s: &str) -> Config {
    toml::from_str(s).unwrap()
}

#[test]
fn test_parse_config() {
    use std::str::FromStr;
    let toml_str = r#"
        [crates]
        crate1 = { path = "/path1" }
        crate2 = { path = "/path2" }
    "#;
    let config = parse_config(&toml_str);
    let mut btree = BTreeMap::new();
    btree.insert(
        "crate1".to_string(),
        CrateInfo {
            path: PathBuf::from_str("/path1").unwrap(),
        },
    );
    btree.insert(
        "crate2".to_string(),
        CrateInfo {
            path: PathBuf::from_str("/path2").unwrap(),
        },
    );
    let right = Config {
        crates: Some(btree),
    };
    assert_eq!(config, right);
}

pub fn write_config(config: &Config) {
    let mut path = config_dir().unwrap();
    path.push("kunai");
    path.push("config.toml");
    let s = toml::to_string(config).unwrap();
    let file = std::fs::File::create(&path).unwrap();
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(s.as_bytes()).unwrap();
}
