use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::io::Write;
use std::path::{Path,PathBuf};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub crates: Option<BTreeMap<String, CrateInfo>>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub struct CrateInfo {
    pub path: PathBuf,
}

impl Config {
    pub fn new() -> Config {
        Config {
            crates: Some(BTreeMap::new()),
        }
    }
    pub fn add_crate<P: AsRef<Path>>(&mut self, path: P) {
        let crate_name = cargo_edit::get_crate_name_from_path(path.as_ref().canonicalize().unwrap().to_str().unwrap()).unwrap();
        if self.crates.is_none() {
            self.crates = Some(BTreeMap::new());
        }
        let mut pathbuf = PathBuf::new();
        pathbuf.push(path);
        let crate_info = CrateInfo {
            path: pathbuf,
        };
        self.crates.as_mut().unwrap().insert(crate_name, crate_info);
    }
    pub fn remove_crate(&mut self, crate_name: &str) {
        if self.crates.is_none() {
            return;
        }
        self.crates.as_mut().unwrap().remove(crate_name);
    }
    pub fn crate_path(&self, name: &str) -> Option<PathBuf> {
        if name == own_name() {
            let mut res = cargo_edit::find(&None).unwrap();
            res.pop();
            Some(res)
        }
        else {
            let r = self.crates.as_ref().unwrap();
            Some(r.get(name).unwrap().path.clone())
        }
    }
}

fn own_name() -> String {
    let mut path = cargo_edit::find(&None).unwrap();
    path.pop();
    cargo_edit::get_crate_name_from_path(path.to_str().unwrap()).unwrap()
}

pub fn read_config() -> Config {
    let mut path = config_dir().unwrap();
    path.push("kunai");
    std::fs::create_dir_all(&path).unwrap();
    path.push("config.toml");
    match read_to_string(&path) {
        Ok(s) => {
            parse_config(&s)
        }
        Err(err) => {
            Config::new()
        }
    }
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
    std::fs::create_dir_all(&path).unwrap();
    path.push("config.toml");
    let s = toml::to_string(config).unwrap();
    let file = std::fs::File::create(&path).unwrap();
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(s.as_bytes()).unwrap();
}
