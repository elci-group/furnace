use furnace::output::{OutputStyle, OutputRenderer, Layout, Detail, ColorMode, SymbolSet};
use furnace::types::{RustFileSnapshot, FunctionSnapshot, StructSnapshot, EnumSnapshot};

fn create_sample_snapshot() -> RustFileSnapshot {
    RustFileSnapshot {
        path: "./src/example.rs".to_string(),
        functions: vec![
            FunctionSnapshot {
                name: "calculate".to_string(),
                args: vec!["x".to_string(), "y".to_string()],
                variables: vec![
                    ("result".to_string(), Some("i32".to_string())),
                    ("temp".to_string(), None),
                ],
            },
            FunctionSnapshot {
                name: "process_data".to_string(),
                args: vec!["data".to_string()],
                variables: vec![],
            },
        ],
        structs: vec![
            StructSnapshot {
                name: "Config".to_string(),
                fields: vec!["host".to_string(), "port".to_string()],
                methods: vec!["new".to_string(), "validate".to_string()],
            },
        ],
        enums: vec![
            EnumSnapshot {
                name: "Status".to_string(),
                variants: vec!["Active".to_string(), "Inactive".to_string()],
                methods: vec!["is_active".to_string()],
            },
        ],
        traits: vec![],
        impls: vec![],
    }
}

#[test]
fn test_plain_output() {
    let snapshot = create_sample_snapshot();
    let style = OutputStyle::default(); // Plain
    let renderer = OutputRenderer::new(style);
    let output = renderer.render(&[snapshot]);
    
    assert!(output.contains("./src/example.rs"));
    assert!(output.contains("Functions"));
    assert!(output.contains("calculate"));
    assert!(output.contains("Structs"));
    assert!(output.contains("Config"));
}

#[test]
fn test_tree_output() {
    let snapshot = create_sample_snapshot();
    let style = OutputStyle::tree();
    let renderer = OutputRenderer::new(style);
    let output = renderer.render(&[snapshot]);
    
    assert!(output.contains("📄"));
    assert!(output.contains("🔧"));
    assert!(output.contains("├──") || output.contains("│"));
}

#[test]
fn test_compact_output() {
    let snapshot = create_sample_snapshot();
    let style = OutputStyle::compact();
    let renderer = OutputRenderer::new(style);
    let output = renderer.render(&[snapshot]);
    
    assert!(output.contains("f=2")); // 2 functions
    assert!(output.contains("s=1")); // 1 struct
    assert!(output.contains("e=1")); // 1 enum
}

#[test]
fn test_grid_output() {
    let snapshot = create_sample_snapshot();
    let style = OutputStyle::grid();
    let renderer = OutputRenderer::new(style);
    let output = renderer.render(&[snapshot]);
    
    assert!(output.contains("+-"));
    assert!(output.contains("|"));
    assert!(output.contains("Functions"));
    assert!(output.contains("Structs"));
}

#[test]
fn test_minimal_detail() {
    let snapshot = create_sample_snapshot();
    let mut style = OutputStyle::default();
    style.detail = Detail::Minimal;
    let renderer = OutputRenderer::new(style);
    let output = renderer.render(&[snapshot]);
    
    // Minimal should only show names, not args/fields
    assert!(output.contains("calculate"));
    assert!(!output.contains("args:"));
}

#[test]
fn test_verbose_detail() {
    let snapshot = create_sample_snapshot();
    let mut style = OutputStyle::default();
    style.detail = Detail::Verbose;
    let renderer = OutputRenderer::new(style);
    let output = renderer.render(&[snapshot]);
    
    // Verbose should show args and variables counts
    assert!(output.contains("calculate"));
    assert!(output.contains("args:") || output.contains("vars:"));
}

#[test]
fn test_composability() {
    let snapshot = create_sample_snapshot();
    
    // Tree layout with minimal detail
    let mut style = OutputStyle::tree();
    style.detail = Detail::Minimal;
    let renderer = OutputRenderer::new(style);
    let output = renderer.render(&[snapshot]);
    
    assert!(output.contains("📄") || output.contains("├──"));
    assert!(output.contains("calculate"));
}

#[test]
fn test_empty_snapshot() {
    let empty = RustFileSnapshot {
        path: "./src/empty.rs".to_string(),
        functions: vec![],
        structs: vec![],
        enums: vec![],
        traits: vec![],
        impls: vec![],
    };
    
    let style = OutputStyle::default();
    let renderer = OutputRenderer::new(style);
    let output = renderer.render(&[empty]);
    
    assert!(output.contains("./src/empty.rs"));
}

#[test]
fn test_multiple_files() {
    let snapshot1 = create_sample_snapshot();
    let mut snapshot2 = create_sample_snapshot();
    snapshot2.path = "./src/another.rs".to_string();
    
    let style = OutputStyle::tree();
    let renderer = OutputRenderer::new(style);
    let output = renderer.render(&[snapshot1, snapshot2]);
    
    assert!(output.contains("./src/example.rs"));
    assert!(output.contains("./src/another.rs"));
}

#[test]
fn test_badges_color_mode() {
    let snapshot = create_sample_snapshot();
    let style = OutputStyle::badges();
    let renderer = OutputRenderer::new(style);
    let output = renderer.render(&[snapshot]);
    
    assert!(output.contains("📁") || output.contains("🔧") || output.contains("🏗️"));
}
