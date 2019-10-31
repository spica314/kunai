use cargo::util::important_paths::find_root_manifest_for_wd;
use cargo::util::errors::CargoResult;
use std::path::PathBuf;
use cargo::core::manifest::Manifest;
use cargo::util::config::Config;
use cargo::core::Workspace;
use std::path::Path;
use std::collections::BTreeMap;

pub fn manifest_path() -> CargoResult<PathBuf> {
    let pwd = std::env::current_dir()?;
    find_root_manifest_for_wd(pwd.as_path())
}

pub fn manifest_from_path(path: &Path) -> CargoResult<Manifest> {
    let manifest_path = find_root_manifest_for_wd(path)?;
    let config = Config::default()?;
    let workspace = Workspace::new(manifest_path.as_path(), &config)?;
    let package = workspace.current()?;
    Ok(package.manifest().clone())
}

pub fn crate_name(manifest: &Manifest) -> String {
    manifest.name().as_str().to_string()
}

pub fn dependency_paths(manifest: &Manifest) -> BTreeMap<String,PathBuf> {
    let dependencies = manifest.dependencies();
    let mut res = BTreeMap::new();
    for dep in dependencies {
        let source_id = dep.source_id();
        let url = source_id.url();
        let file_path = url.to_file_path().unwrap();
        let name = dep.name_in_toml().as_str();
        res.insert(name.to_string(), file_path);
    }
    res
}
