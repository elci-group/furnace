use furnace::types::RustFileSnapshot;
use furnace::linting::lint_snapshots;
use furnace::config::load_config;
use furnace::engine::TraversalEngine;
use furnace::graph::ModuleNode;
use furnace::output::{OutputStyle, OutputRenderer, Layout, Detail, ColorMode, SymbolSet};

use clap::{Parser, ValueEnum};
use colored::*;
use std::path::PathBuf;
use std::fs;

#[derive(Parser)]
#[command(author = "Rory Spring", version="0.1.0", about="Furnace: Rust snapshot scanner with linting")]
struct Args {
    /// Rust project path
    #[arg(value_name = "RUST_PROJECT_PATH")]
    path: String,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,

    // ===== OUTPUT AESTHETICS (10 Presets) =====
    /// Style 1: Plain (default) - Simple text, no colors
    #[arg(long)]
    plain: bool,

    /// Style 2: Tree - Hierarchical tree view with colors
    #[arg(long)]
    tree: bool,

    /// Style 3: Compact - Dense, minimal whitespace
    #[arg(long)]
    compact: bool,

    /// Style 4: Verbose - Extra details and descriptions
    #[arg(long)]
    verbose: bool,

    /// Style 5: Minimal - Only essential information
    #[arg(long)]
    minimal: bool,

    /// Style 6: Grid - Table-like layout
    #[arg(long)]
    grid: bool,

    /// Style 7: Markdown - Markdown-formatted output
    #[arg(long)]
    markdown: bool,

    /// Style 8: HTML - HTML-formatted output
    #[arg(long)]
    html: bool,

    /// Style 9: Badges - With emoji/badge indicators
    #[arg(long)]
    badges: bool,

    /// Style 10: Monochrome - No colors, symbols only
    #[arg(long)]
    monochrome: bool,

    // ===== COMPOSABLE MODIFIERS =====
    /// Force specific layout: plain, tree, grid, compact
    #[arg(long, value_enum)]
    layout: Option<LayoutArg>,

    /// Force specific detail level: minimal, standard, verbose
    #[arg(long, value_enum)]
    detail: Option<DetailArg>,

    /// Force specific color mode: none, standard, badges
    #[arg(long, value_enum)]
    color: Option<ColorArg>,

    /// Force specific symbol set: none, ascii, unicode
    #[arg(long, value_enum)]
    symbols: Option<SymbolArg>,

    // ===== AI-POWERED ANALYSIS =====
    /// Enable AI-powered code analysis (requires --features ai and API key)
    #[arg(long)]
    ai_lint: bool,

    /// AI provider to use: openai or google
    #[arg(long, default_value = "openai")]
    ai_provider: String,

    /// AI model to use (e.g., gpt-4, gpt-3.5-turbo, gemini-pro)
    #[arg(long)]
    ai_model: Option<String>,

    /// Explain code in simple/layman terms using AI (requires --features ai)
    /// Specify provider: --layman=openai or --layman=google
    #[arg(long, value_name = "PROVIDER")]
    layman: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum LayoutArg {
    Plain,
    Tree,
    Grid,
    Compact,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum DetailArg {
    Minimal,
    Standard,
    Verbose,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ColorArg {
    None,
    Standard,
    Badges,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum SymbolArg {
    None,
    Ascii,
    Unicode,
}

fn collect_snapshots(module: &ModuleNode, snapshots: &mut Vec<RustFileSnapshot>, ignore: &[String]) {
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

fn main() {
    let args = Args::parse();
    let project_path = PathBuf::from(&args.path);
    let config = load_config(&project_path);

    // Use the new Semantic Traversal Engine
    let engine = TraversalEngine::new(project_path.clone());
    let graph = engine.scan();

    // Flatten graph to snapshots for existing linting/output logic
    let mut snapshots: Vec<RustFileSnapshot> = vec![];
    for crate_node in &graph.crates {
        collect_snapshots(&crate_node.root_module, &mut snapshots, &config.ignore);
    }
    
    // Run linting
    let warnings = lint_snapshots(&snapshots[..], &config.lints);

    // Run AI analysis if requested
    if args.ai_lint {
        #[cfg(feature = "ai")]
        {
            use furnace::ai_linting::{AILinter, AIProvider};
            
            println!("{}", "\n🤖 Running AI-powered analysis...".cyan().bold());
            
            let provider = match args.ai_provider.as_str() {
                "openai" => AIProvider::OpenAI {
                    model: args.ai_model.as_ref().map(|s| s.clone()).unwrap_or_else(|| "gpt-4".to_string()),
                },
                "google" => AIProvider::Google {
                    model: args.ai_model.as_ref().map(|s| s.clone()).unwrap_or_else(|| "gemini-pro".to_string()),
                },
                _ => {
                    eprintln!("Unknown AI provider: {}. Use 'openai' or 'google'.", args.ai_provider);
                    std::process::exit(1);
                }
            };
            
            let linter = AILinter::new(provider);
            
            // Run async analysis
            let runtime = tokio::runtime::Runtime::new().unwrap();
            match runtime.block_on(linter.analyze_project(&snapshots)) {
                Ok(analysis) => {
                    println!("\n{}", "AI Analysis Results:".green().bold());
                    
                    if let Some(score) = analysis.quality_score {
                        println!("Quality Score: {}/100", score);
                    }
                    
                    if !analysis.insights.is_empty() {
                        println!("\n{}:", "Insights".yellow());
                        for (i, insight) in analysis.insights.iter().enumerate() {
                            println!("{}. {}", i + 1, insight);
                        }
                    }
                    
                    if !analysis.suggestions.is_empty() {
                        println!("\n{}:", "Suggestions".cyan());
                        for (i, suggestion) in analysis.suggestions.iter().enumerate() {
                            println!("{}. {}", i + 1, suggestion);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "AI analysis failed".red(), e);
                }
            }
        }
        
        #[cfg(not(feature = "ai"))]
        {
            eprintln!("{}", "AI features are not enabled. Rebuild with:".yellow());
            eprintln!("  cargo build --features ai");
        }
    }

    // Run layman explanation if requested
    if let Some(provider) = &args.layman {
        #[cfg(feature = "ai")]
        {
            use furnace::ai_linting::{AILinter, AIProvider};
            
            println!("{}", "\n📚 Generating beginner-friendly explanation...".cyan().bold());
            
            let ai_provider = match provider.to_lowercase().as_str() {
                "openai" => AIProvider::OpenAI {
                    model: args.ai_model.clone().unwrap_or_else(|| "gpt-4".to_string()),
                },
                "google" | "gemini" => AIProvider::Google {
                    model: args.ai_model.clone().unwrap_or_else(|| "gemini-pro".to_string()),
                },
                _ => {
                    eprintln!("Unknown provider: {}. Use 'openai' or 'google'.", provider);
                    std::process::exit(1);
                }
            };
            
            let linter = AILinter::new(ai_provider);
            
            let runtime = tokio::runtime::Runtime::new().unwrap();
            match runtime.block_on(linter.explain_for_layman(&snapshots)) {
                Ok(explanation) => {
                    println!("\n{}", "═".repeat(80).bright_cyan());
                    println!("{}", "🎓 LAYMAN'S EXPLANATION".bright_green().bold());
                    println!("{}", "═".repeat(80).bright_cyan());
                    println!();
                    println!("{}", explanation);
                    println!();
                    println!("{}", "═".repeat(80).bright_cyan());
                }
                Err(e) => {
                    eprintln!("{}: {}", "Explanation failed".red(), e);
                }
            }
        }
        
        #[cfg(not(feature = "ai"))]
        {
            eprintln!("{}", "AI features are not enabled. Rebuild with:".yellow());
            eprintln!("  cargo build --features ai");
        }
        
        // Exit after layman explanation (don't show technical output)
        return;
    }

    // Determine output style
    let style = resolve_output_style(&args);

    // Render output
    match args.format {
        OutputFormat::Text => {
            let renderer = OutputRenderer::new(style);
            let output = renderer.render(&snapshots);
            println!("{}", output);
            
            // Print warnings
            if !warnings.is_empty() {
                println!("{}", "Linting Warnings:".yellow().bold());
                for warning in &warnings {
                    println!("{}", warning);
                }
            }
            
            println!("\nOutput saved to furnace_output.toon");
            fs::write("furnace_output.toon", &output).unwrap_or_default();
        }
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&snapshots).unwrap();
            println!("{}", json);
        }
    }
}

fn resolve_output_style(args: &Args) -> OutputStyle {
    // Start with a base style from presets
    let mut style = if args.tree {
        OutputStyle::tree()
    } else if args.compact {
        OutputStyle::compact()
    } else if args.verbose {
        OutputStyle::verbose()
    } else if args.minimal {
        OutputStyle::minimal()
    } else if args.grid {
        OutputStyle::grid()
    } else if args.markdown {
        OutputStyle::markdown()
    } else if args.html {
        OutputStyle::html()
    } else if args.badges {
        OutputStyle::badges()
    } else if args.monochrome {
        OutputStyle::monochrome()
    } else if args.plain {
        OutputStyle::default()
    } else {
        // No preset specified, use default (plain)
        OutputStyle::default()
    };

    // Apply composable modifiers
    if let Some(layout) = args.layout {
        style.layout = match layout {
            LayoutArg::Plain => Layout::Plain,
            LayoutArg::Tree => Layout::Tree,
            LayoutArg::Grid => Layout::Grid,
            LayoutArg::Compact => Layout::Compact,
        };
    }

    if let Some(detail) = args.detail {
        style.detail = match detail {
            DetailArg::Minimal => Detail::Minimal,
            DetailArg::Standard => Detail::Standard,
            DetailArg::Verbose => Detail::Verbose,
        };
    }

    if let Some(color) = args.color {
        style.color = match color {
            ColorArg::None => ColorMode::None,
            ColorArg::Standard => ColorMode::Standard,
            ColorArg::Badges => ColorMode::Badges,
        };
    }

    if let Some(symbols) = args.symbols {
        style.symbols = match symbols {
            SymbolArg::None => SymbolSet::None,
            SymbolArg::Ascii => SymbolSet::Ascii,
            SymbolArg::Unicode => SymbolSet::Unicode,
        };
    }

    style
}
