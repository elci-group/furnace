use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct FurnaceConfig {
    #[serde(default)]
    pub lints: LintConfig,
    #[serde(default)]
    pub ignore: Vec<String>,
}

impl Default for FurnaceConfig {
    fn default() -> Self {
        Self {
            lints: LintConfig::default(),
            ignore: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct LintConfig {
    // Global controls
    #[serde(default)]
    pub enabled: Option<bool>, // Master switch, defaults to true
    
    // Complexity lints
    #[serde(default)]
    pub complexity: ComplexityLints,
    
    // Naming convention lints
    #[serde(default)]
    pub naming: NamingLints,
    
    // Style lints
    #[serde(default)]
    pub style: StyleLints,
    
    // AI-powered lints
    #[serde(default)]
    pub ai: AILintConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AILintConfig {
    pub enabled: Option<bool>,
    pub provider: Option<String>, // "openai" or "google"
    pub model: Option<String>,
    pub max_tokens: Option<usize>,
    pub temperature: Option<f32>,
}

impl Default for AILintConfig {
    fn default() -> Self {
        Self {
            enabled: None,
            provider: Some("openai".to_string()),
            model: Some("gpt-4".to_string()),
            max_tokens: Some(4000),
            temperature: Some(0.3),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ComplexityLints {
    pub max_args: Option<usize>,
    pub max_fields: Option<usize>,
    pub max_function_lines: Option<usize>,
    pub max_struct_size: Option<usize>, // In number of fields
}

#[derive(Debug, Deserialize, Clone)]
pub struct NamingLints {
    pub enforce_snake_case_functions: Option<bool>,
    pub enforce_snake_case_variables: Option<bool>,
    pub enforce_pascal_case_types: Option<bool>,
    pub enforce_screaming_snake_case_constants: Option<bool>,
    pub discouraged_names: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StyleLints {
    pub require_doc_comments: Option<bool>,
    pub warn_todo_comments: Option<bool>,
}

impl Default for ComplexityLints {
    fn default() -> Self {
        Self {
            max_args: None,         // Disabled by default
            max_fields: None,       // Disabled by default
            max_function_lines: None,
            max_struct_size: None,
        }
    }
}

impl Default for NamingLints {
    fn default() -> Self {
        Self {
            enforce_snake_case_functions: None,  // Disabled by default
            enforce_snake_case_variables: None,  // Disabled by default
            enforce_pascal_case_types: None,     // Disabled by default
            enforce_screaming_snake_case_constants: None,
            discouraged_names: None,             // Disabled by default
        }
    }
}

impl Default for StyleLints {
    fn default() -> Self {
        Self {
            require_doc_comments: None,  // Disabled by default
            warn_todo_comments: None,    // Disabled by default
        }
    }
}

impl Default for LintConfig {
    fn default() -> Self {
        Self {
            enabled: Some(true),
            complexity: ComplexityLints::default(),
            naming: NamingLints::default(),
            style: StyleLints::default(),
            ai: AILintConfig::default(),
        }
    }
}

pub fn load_config(path: &Path) -> FurnaceConfig {
    let config_path = path.join(".furnacerc.toml");
    if config_path.exists() {
        let content = fs::read_to_string(config_path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_else(|e| {
            eprintln!("Warning: Failed to parse .furnacerc.toml: {}", e);
            FurnaceConfig::default()
        })
    } else {
        FurnaceConfig::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = FurnaceConfig::default();
        assert_eq!(config.lints.enabled, Some(true));
        assert!(config.ignore.is_empty());
        // All lints should be disabled by default
        assert_eq!(config.lints.complexity.max_args, None);
        assert_eq!(config.lints.naming.enforce_snake_case_functions, None);
    }

    #[test]
    fn test_parse_config() {
        let toml = r#"
            ignore = ["target", "dist"]
            [lints.complexity]
            max_args = 10
            max_fields = 15
            [lints.naming]
            discouraged_names = ["temp"]
            enforce_snake_case_functions = true
        "#;
        let config: FurnaceConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.lints.complexity.max_args, Some(10));
        assert_eq!(config.lints.complexity.max_fields, Some(15));
        assert_eq!(config.ignore.len(), 2);
        assert_eq!(config.lints.naming.discouraged_names.as_ref().unwrap()[0], "temp");
        assert_eq!(config.lints.naming.enforce_snake_case_functions, Some(true));
    }
}
