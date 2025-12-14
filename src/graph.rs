use serde::Serialize;
use std::path::PathBuf;
use crate::types::RustFileSnapshot;

#[derive(Debug, Clone, Serialize)]
pub struct ProjectGraph {
    pub root_path: PathBuf,
    pub crates: Vec<CrateNode>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CrateNode {
    pub name: String,
    pub version: String,
    pub path: PathBuf,
    pub root_module: ModuleNode,
}

#[derive(Debug, Clone, Serialize)]
pub struct ModuleNode {
    pub name: String,
    pub path: Option<PathBuf>, // Directory path if it's a dir module
    pub file: Option<FileNode>, // The file defining this module (mod.rs or name.rs)
    pub submodules: Vec<ModuleNode>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileNode {
    pub path: PathBuf,
    pub hash: String,
    pub snapshot: Option<RustFileSnapshot>, // Content, loaded lazily or cached
}
