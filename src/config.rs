use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::cargo_wrapper;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub struct Config {
}

impl Config {
    pub fn new() -> Config {
        Config {
        }
    }
}

pub fn read_config() -> Config {
    let mut path = config_dir().unwrap();
    path.push("kunai");
    std::fs::create_dir_all(&path).unwrap();
    path.push("config.toml");
    match read_to_string(&path) {
        Ok(s) => parse_config(&s),
        Err(err) => Config::new(),
    }
}

fn parse_config(s: &str) -> Config {
    toml::from_str(s).unwrap()
}

pub fn write_config(config: &Config) {
    let mut path = config_dir().unwrap();
    path.push("kunai");
    std::fs::create_dir_all(&path).unwrap();
    path.push("config.toml");
    let s = toml::to_string(config).unwrap();
    let file = std::fs::File::create(&path).unwrap();
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(s.as_bytes()).unwrap();
}
