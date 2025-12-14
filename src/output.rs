use crate::types::{RustFileSnapshot, FunctionSnapshot, StructSnapshot, EnumSnapshot};
use colored::*;

#[derive(Debug, Clone)]
pub struct OutputStyle {
    pub layout: Layout,
    pub detail: Detail,
    pub color: ColorMode,
    pub symbols: SymbolSet,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Layout {
    Plain,      // Simple list
    Tree,       // Hierarchical tree (default aesthetic)
    Grid,       // Table-like
    Compact,    // Dense, minimal whitespace
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Detail {
    Minimal,    // Only names
    Standard,   // Names + counts
    Verbose,    // Full details
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorMode {
    None,       // Monochrome
    Standard,   // Default colors
    Badges,     // Emoji/unicode badges
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SymbolSet {
    None,       // No symbols
    Ascii,      // ASCII-only symbols
    Unicode,    // Full unicode
}

impl Default for OutputStyle {
    fn default() -> Self {
        Self {
            layout: Layout::Plain,
            detail: Detail::Standard,
            color: ColorMode::None,
            symbols: SymbolSet::None,
        }
    }
}

impl OutputStyle {
    pub fn tree() -> Self {
        Self {
            layout: Layout::Tree,
            detail: Detail::Standard,
            color: ColorMode::Standard,
            symbols: SymbolSet::Unicode,
        }
    }

    pub fn compact() -> Self {
        Self {
            layout: Layout::Compact,
            detail: Detail::Minimal,
            color: ColorMode::None,
            symbols: SymbolSet::None,
        }
    }

    pub fn verbose() -> Self {
        Self {
            layout: Layout::Tree,
            detail: Detail::Verbose,
            color: ColorMode::Standard,
            symbols: SymbolSet::Unicode,
        }
    }

    pub fn minimal() -> Self {
        Self {
            layout: Layout::Plain,
            detail: Detail::Minimal,
            color: ColorMode::None,
            symbols: SymbolSet::None,
        }
    }

    pub fn grid() -> Self {
        Self {
            layout: Layout::Grid,
            detail: Detail::Standard,
            color: ColorMode::None,
            symbols: SymbolSet::Ascii,
        }
    }

    pub fn markdown() -> Self {
        Self {
            layout: Layout::Plain,
            detail: Detail::Standard,
            color: ColorMode::None,
            symbols: SymbolSet::Ascii,
        }
    }

    pub fn html() -> Self {
        Self {
            layout: Layout::Tree,
            detail: Detail::Standard,
            color: ColorMode::None,
            symbols: SymbolSet::None,
        }
    }

    pub fn badges() -> Self {
        Self {
            layout: Layout::Plain,
            detail: Detail::Standard,
            color: ColorMode::Badges,
            symbols: SymbolSet::Unicode,
        }
    }

    pub fn monochrome() -> Self {
        Self {
            layout: Layout::Tree,
            detail: Detail::Standard,
            color: ColorMode::None,
            symbols: SymbolSet::Unicode,
        }
    }
}

pub struct OutputRenderer {
    style: OutputStyle,
}

impl OutputRenderer {
    pub fn new(style: OutputStyle) -> Self {
        Self { style }
    }

    pub fn render(&self, snapshots: &[RustFileSnapshot]) -> String {
        match self.style.layout {
            Layout::Plain => self.render_plain(snapshots),
            Layout::Tree => self.render_tree(snapshots),
            Layout::Grid => self.render_grid(snapshots),
            Layout::Compact => self.render_compact(snapshots),
        }
    }

    fn render_plain(&self, snapshots: &[RustFileSnapshot]) -> String {
        let mut output = String::new();
        
        for snapshot in snapshots {
            output.push_str(&self.format_path(&snapshot.path));
            output.push('\n');
            
            if !snapshot.functions.is_empty() {
                output.push_str(&self.format_section_header("Functions"));
                for func in &snapshot.functions {
                    output.push_str(&self.format_function(func));
                }
            }
            
            if !snapshot.structs.is_empty() {
                output.push_str(&self.format_section_header("Structs"));
                for strct in &snapshot.structs {
                    output.push_str(&self.format_struct(strct));
                }
            }
            
            if !snapshot.enums.is_empty() {
                output.push_str(&self.format_section_header("Enums"));
                for enm in &snapshot.enums {
                    output.push_str(&self.format_enum(enm));
                }
            }
            
            output.push('\n');
        }
        
        output
    }

    fn render_tree(&self, snapshots: &[RustFileSnapshot]) -> String {
        let mut output = String::new();
        let tree_sym = match self.style.symbols {
            SymbolSet::Unicode => ("├──", "│  ", "└──"),
            SymbolSet::Ascii => ("|--", "|  ", "`--"),
            SymbolSet::None => ("", "  ", ""),
        };
        
        for snapshot in snapshots {
            output.push_str(&format!("{} 📄 {}\n", tree_sym.0, self.format_path(&snapshot.path)));
            
            if !snapshot.functions.is_empty() {
                output.push_str(&format!("{}  🔧 Functions:\n", tree_sym.1));
                for func in &snapshot.functions {
                    output.push_str(&format!("{}  - {}\n", tree_sym.1, self.format_function_inline(func)));
                }
            }
            
            if !snapshot.structs.is_empty() {
                output.push_str(&format!("{}  🏗️ Structs:\n", tree_sym.1));
                for strct in &snapshot.structs {
                    output.push_str(&format!("{}  - {}\n", tree_sym.1, self.format_struct_inline(strct)));
                }
            }
            
            if !snapshot.enums.is_empty() {
                output.push_str(&format!("{}  🧩 Enums:\n", tree_sym.1));
                for enm in &snapshot.enums {
                    output.push_str(&format!("{}  - {}\n", tree_sym.1, self.format_enum_inline(enm)));
                }
            }
        }
        
        output
    }

    fn render_grid(&self, snapshots: &[RustFileSnapshot]) -> String {
        let mut output = String::new();
        
        output.push_str("+----------------------+----------+----------+----------+\n");
        output.push_str("| File                 | Functions| Structs  | Enums    |\n");
        output.push_str("+----------------------+----------+----------+----------+\n");
        
        for snapshot in snapshots {
            let path = snapshot.path.split('/').last().unwrap_or(&snapshot.path);
            output.push_str(&format!(
                "| {:<20} | {:<8} | {:<8} | {:<8} |\n",
                self.truncate(path, 20),
                snapshot.functions.len(),
                snapshot.structs.len(),
                snapshot.enums.len()
            ));
        }
        
        output.push_str("+----------------------+----------+----------+----------+\n");
        output
    }

    fn render_compact(&self, snapshots: &[RustFileSnapshot]) -> String {
        let mut output = String::new();
        
        for snapshot in snapshots {
            let path = snapshot.path.split('/').last().unwrap_or(&snapshot.path);
            output.push_str(&format!(
                "{}: f={} s={} e={}\n",
                path,
                snapshot.functions.len(),
                snapshot.structs.len(),
                snapshot.enums.len()
            ));
        }
        
        output
    }

    fn format_path(&self, path: &str) -> String {
        match self.style.color {
            ColorMode::Standard => path.bright_blue().to_string(),
            ColorMode::Badges => format!("📁 {}", path),
            ColorMode::None => path.to_string(),
        }
    }

    fn format_section_header(&self, name: &str) -> String {
        match self.style.color {
            ColorMode::Standard => format!("  {}:\n", name.yellow().bold()),
            ColorMode::Badges => {
                let icon = match name {
                    "Functions" => "🔧",
                    "Structs" => "🏗️",
                    "Enums" => "🧩",
                    _ => "📦",
                };
                format!("  {} {}:\n", icon, name)
            }
            ColorMode::None => format!("  {}:\n", name),
        }
    }

    fn format_function(&self, func: &FunctionSnapshot) -> String {
        match self.style.detail {
            Detail::Minimal => format!("    {}\n", func.name),
            Detail::Standard => format!("    {} (args: {})\n", func.name, func.args.len()),
            Detail::Verbose => format!(
                "    {} (args: {}, vars: {})\n",
                func.name,
                func.args.len(),
                func.variables.len()
            ),
        }
    }

    fn format_function_inline(&self, func: &FunctionSnapshot) -> String {
        match self.style.detail {
            Detail::Minimal => func.name.clone(),
            Detail::Standard => format!("{}: args [{}]", func.name, func.args.join(", ")),
            Detail::Verbose => format!(
                "{}: args [{}], variables [{}]",
                func.name,
                func.args.join(", "),
                func.variables.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>().join(", ")
            ),
        }
    }

    fn format_struct(&self, strct: &StructSnapshot) -> String {
        match self.style.detail {
            Detail::Minimal => format!("    {}\n", strct.name),
            Detail::Standard => format!("    {} (fields: {})\n", strct.name, strct.fields.len()),
            Detail::Verbose => format!(
                "    {} (fields: {}, methods: {})\n",
                strct.name,
                strct.fields.len(),
                strct.methods.len()
            ),
        }
    }

    fn format_struct_inline(&self, strct: &StructSnapshot) -> String {
        match self.style.detail {
            Detail::Minimal => strct.name.clone(),
            Detail::Standard => format!("{}: fields [{}]", strct.name, strct.fields.join(", ")),
            Detail::Verbose => format!(
                "{}: fields [{}], methods [{}]",
                strct.name,
                strct.fields.join(", "),
                strct.methods.join(", ")
            ),
        }
    }

    fn format_enum(&self, enm: &EnumSnapshot) -> String {
        match self.style.detail {
            Detail::Minimal => format!("    {}\n", enm.name),
            Detail::Standard => format!("    {} (variants: {})\n", enm.name, enm.variants.len()),
            Detail::Verbose => format!(
                "    {} (variants: {}, methods: {})\n",
                enm.name,
                enm.variants.len(),
                enm.methods.len()
            ),
        }
    }

    fn format_enum_inline(&self, enm: &EnumSnapshot) -> String {
        match self.style.detail {
            Detail::Minimal => enm.name.clone(),
            Detail::Standard => format!("{}: variants [{}]", enm.name, enm.variants.join(", ")),
            Detail::Verbose => format!(
                "{}: variants [{}], methods [{}]",
                enm.name,
                enm.variants.join(", "),
                enm.methods.join(", ")
            ),
        }
    }

    fn truncate(&self, s: &str, max_len: usize) -> String {
        if s.len() <= max_len {
            s.to_string()
        } else {
            format!("{}...", &s[..max_len - 3])
        }
    }
}
