use furnace::engine::TraversalEngine;
use furnace::config::load_config;
use furnace::linting::lint_snapshots;
use std::path::PathBuf;

#[test]
fn test_engine_scans_furnace_project() {
    // Test scanning the furnace project itself
    let project_path = PathBuf::from(".");
    let engine = TraversalEngine::new(project_path.clone());
    let graph = engine.scan();
    
    // Should find at least one crate
    assert!(!graph.crates.is_empty(), "Should find at least one crate");
    
    // The crate should be named "furnace"
    assert_eq!(graph.crates[0].name, "furnace");
    
    // Should have a root module
    assert_eq!(graph.crates[0].root_module.name, "crate");
}

#[test]
fn test_engine_discovers_modules() {
    let project_path = PathBuf::from(".");
    let engine = TraversalEngine::new(project_path);
    let graph = engine.scan();
    
    let root_module = &graph.crates[0].root_module;
    
    // Should discover submodules
    assert!(!root_module.submodules.is_empty(), "Should find submodules");
    
    // Check that known modules exist
    let module_names: Vec<String> = root_module.submodules.iter()
        .map(|m| m.name.clone())
        .collect();
    
    assert!(module_names.contains(&"types".to_string()), "Should find types module");
    assert!(module_names.contains(&"config".to_string()), "Should find config module");
    assert!(module_names.contains(&"engine".to_string()), "Should find engine module");
}

#[test]
fn test_engine_ignores_target() {
    let project_path = PathBuf::from(".");
    let engine = TraversalEngine::new(project_path);
    let graph = engine.scan();
    
    // Collect all file paths from the graph
    fn collect_paths(module: &furnace::graph::ModuleNode, paths: &mut Vec<PathBuf>) {
        if let Some(file) = &module.file {
            paths.push(file.path.clone());
        }
        for submodule in &module.submodules {
            collect_paths(submodule, paths);
        }
    }
    
    let mut all_paths = Vec::new();
    for crate_node in &graph.crates {
        collect_paths(&crate_node.root_module, &mut all_paths);
    }
    
    // No path should contain "target/"
    for path in &all_paths {
        let path_str = path.to_string_lossy();
        assert!(!path_str.contains("/target/"), 
            "Should not scan target directory, but found: {}", path_str);
    }
}

#[test]
fn test_config_loading() {
    let project_path = PathBuf::from(".");
    let config = load_config(&project_path);
    
    // Config should load successfully (either from file or defaults)
    assert!(config.lints.enabled.is_some());
}

#[test]
fn test_linting_integration() {
    let project_path = PathBuf::from(".");
    let config = load_config(&project_path);
    let engine = TraversalEngine::new(project_path);
    let graph = engine.scan();
    
    // Flatten graph to snapshots
    fn collect_snapshots(
        module: &furnace::graph::ModuleNode, 
        snapshots: &mut Vec<furnace::types::RustFileSnapshot>,
        ignore: &[String]
    ) {
        if let Some(file_node) = &module.file {
            let path_str = file_node.path.to_string_lossy();
            if !ignore.iter().any(|pattern| path_str.contains(pattern)) {
                if let Some(snap) = &file_node.snapshot {
                    snapshots.push(snap.clone());
                }
            }
        }
        for submodule in &module.submodules {
            collect_snapshots(submodule, snapshots, ignore);
        }
    }
    
    let mut snapshots = Vec::new();
    for crate_node in &graph.crates {
        collect_snapshots(&crate_node.root_module, &mut snapshots, &config.ignore);
    }
    
    // Run linting
    let _warnings = lint_snapshots(&snapshots, &config.lints);
    
    // Warnings should be a valid vector
    // We don't assert on count as it depends on the current code state
}

#[test]
fn test_file_hashing() {
    let project_path = PathBuf::from(".");
    let engine = TraversalEngine::new(project_path);
    let graph = engine.scan();
    
    // Check that files have hashes
    if let Some(file) = &graph.crates[0].root_module.file {
        assert!(!file.hash.is_empty(), "File should have a hash");
        assert_eq!(file.hash.len(), 64, "SHA256 hash should be 64 hex characters");
    }
}

#[test]
fn test_snapshots_contain_data() {
    let project_path = PathBuf::from(".");
    let engine = TraversalEngine::new(project_path);
    let graph = engine.scan();
    
    // At least one file should have parsed content
    let mut found_snapshot = false;
    
    fn check_snapshots(module: &furnace::graph::ModuleNode) -> bool {
        if let Some(file) = &module.file {
            if file.snapshot.is_some() {
                return true;
            }
        }
        module.submodules.iter().any(check_snapshots)
    }
    
    for crate_node in &graph.crates {
        if check_snapshots(&crate_node.root_module) {
            found_snapshot = true;
            break;
        }
    }
    
    assert!(found_snapshot, "Should find at least one snapshot with parsed content");
}
