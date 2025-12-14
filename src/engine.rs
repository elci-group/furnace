use crate::graph::{ProjectGraph, CrateNode, ModuleNode, FileNode};
use crate::types::RustFileSnapshot;
use crate::visitor::SnapshotVisitor;
use cargo_toml::Manifest;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use syn::parse_file;
use syn::visit::Visit;

pub struct TraversalEngine {
    root: PathBuf,
}

impl TraversalEngine {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn scan(&self) -> ProjectGraph {
        // 1. Try to find Cargo.toml
        let cargo_path = self.root.join("Cargo.toml");
        if cargo_path.exists() {
            self.scan_cargo_project(&cargo_path)
        } else {
            // Fallback: Treat as a single crate rooted at the directory
            // For now, let's just support Cargo projects for the semantic graph.
            // Or we could implement a simple recursive scan here.
            eprintln!("Warning: No Cargo.toml found. Falling back to simple directory scan (not fully implemented in graph mode yet).");
            ProjectGraph {
                root_path: self.root.clone(),
                crates: vec![],
            }
        }
    }

    fn scan_cargo_project(&self, cargo_path: &Path) -> ProjectGraph {
        let manifest = Manifest::from_path(cargo_path).unwrap_or_else(|_| Manifest { package: None, workspace: None, dependencies: Default::default(), dev_dependencies: Default::default(), build_dependencies: Default::default(), target: Default::default(), features: Default::default(), patch: Default::default(), lib: None, profile: Default::default(), badges: Default::default(), bin: Default::default(), bench: Default::default(), test: Default::default(), example: Default::default(), replace: Default::default(), lints: Default::default() });
        
        let mut crates = vec![];

        // Handle workspace members
        if let Some(workspace) = &manifest.workspace {
            for member in &workspace.members {
                // Simple glob expansion would be needed here for full correctness,
                // but for now assume direct paths or simple wildcards.
                // We'll just look for subdirectories that match.
                let member_path = cargo_path.parent().unwrap().join(member);
                // This is a simplification. Real cargo workspace resolution is complex.
                // We will assume the member string is a relative path to a crate root.
                if member_path.exists() {
                     if let Some(crate_node) = self.scan_crate(&member_path) {
                        crates.push(crate_node);
                    }
                } else {
                     // Try globbing if it contains *
                     if member.contains('*') {
                         // Very basic glob support
                         let prefix = member.split('*').next().unwrap_or("");
                         let parent = cargo_path.parent().unwrap().join(prefix);
                         if parent.exists() {
                             for entry in fs::read_dir(parent).into_iter().flatten().flatten() {
                                 if entry.path().is_dir() && entry.path().join("Cargo.toml").exists() {
                                     if let Some(crate_node) = self.scan_crate(&entry.path()) {
                                         crates.push(crate_node);
                                     }
                                 }
                             }
                         }
                     }
                }
            }
        } else if let Some(_package) = &manifest.package {
            // Single package project
            if let Some(crate_node) = self.scan_crate(cargo_path.parent().unwrap()) {
                crates.push(crate_node);
            }
        }

        ProjectGraph {
            root_path: self.root.clone(),
            crates,
        }
    }

    fn scan_crate(&self, crate_root: &Path) -> Option<CrateNode> {
        let cargo_path = crate_root.join("Cargo.toml");
        let manifest = Manifest::from_path(&cargo_path).ok()?;
        let package = manifest.package?;

        let name = package.name;
        let version = package.version.get().map(|v| v.to_string()).unwrap_or_else(|_| "0.0.0".to_string());

        // Find entry point (lib.rs or main.rs)
        let src_path = crate_root.join("src");
        let lib_rs = src_path.join("lib.rs");
        let main_rs = src_path.join("main.rs");

        let root_file = if lib_rs.exists() {
            Some(lib_rs)
        } else if main_rs.exists() {
            Some(main_rs)
        } else {
            None
        };

        if let Some(root_file) = root_file {
            let root_module = self.scan_module("crate", &root_file, &src_path);
            Some(CrateNode {
                name,
                version,
                path: crate_root.to_path_buf(),
                root_module,
            })
        } else {
            None
        }
    }

    fn scan_module(&self, name: &str, file_path: &Path, search_dir: &Path) -> ModuleNode {
        let file_node = self.create_file_node(file_path);
        let mut submodules = vec![];

        // Parse the file to find `mod xyz;` declarations
        if let Some(_snapshot) = &file_node.snapshot {
             let content = fs::read_to_string(file_path).unwrap_or_default();
             if let Ok(ast) = parse_file(&content) {
                 for item in ast.items {
                     if let syn::Item::Mod(item_mod) = item {
                         let mod_name = item_mod.ident.to_string();
                         
                         if item_mod.content.is_none() {
                             // Look for the file
                             let p1 = search_dir.join(format!("{}.rs", mod_name));
                             let p2 = search_dir.join(&mod_name).join("mod.rs");
                             
                             if p1.exists() {
                                 submodules.push(self.scan_module(&mod_name, &p1, search_dir));
                             } else if p2.exists() {
                                 submodules.push(self.scan_module(&mod_name, &p2, &search_dir.join(&mod_name)));
                             }
                         }
                     }
                 }
             }
        }

        ModuleNode {
            name: name.to_string(),
            path: Some(search_dir.to_path_buf()),
            file: Some(file_node),
            submodules,
        }
    }

    fn create_file_node(&self, path: &Path) -> FileNode {
        let content = fs::read_to_string(path).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let hash = hex::encode(hasher.finalize());

        // Parse content (Content Phase)
        // In a real incremental engine, we would check the cache here.
        let snapshot = if let Ok(file) = parse_file(&content) {
            let mut visitor = SnapshotVisitor::default();
            visitor.visit_file(&file);
            
            // Associate impls (local)
            for impl_snap in &visitor.impls {
                if let Some(struct_idx) = visitor.structs.iter().position(|s| s.name == impl_snap.for_type) {
                    visitor.structs[struct_idx].methods.extend(impl_snap.methods.clone());
                } else if let Some(enum_idx) = visitor.enums.iter().position(|e| e.name == impl_snap.for_type) {
                    visitor.enums[enum_idx].methods.extend(impl_snap.methods.clone());
                }
            }

            Some(RustFileSnapshot {
                path: path.to_string_lossy().to_string(),
                functions: visitor.functions,
                structs: visitor.structs,
                traits: visitor.traits,
                enums: visitor.enums,
                impls: visitor.impls,
            })
        } else {
            None
        };

        FileNode {
            path: path.to_path_buf(),
            hash,
            snapshot,
        }
    }
}
