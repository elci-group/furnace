use crate::types::RustFileSnapshot;
use crate::config::LintConfig;

pub fn lint_snapshots(snapshots: &[RustFileSnapshot], config: &LintConfig) -> Vec<String> {
    let mut warnings = Vec::new();

    // Check if linting is globally enabled
    if config.enabled == Some(false) {
        return warnings;
    }

    for snapshot in snapshots {
        // Complexity: Function argument count
        if let Some(max_args) = config.complexity.max_args {
            for func in &snapshot.functions {
                if func.args.len() > max_args {
                    warnings.push(format!(
                        "Warning: Function '{}' in '{}' has {} arguments (max {} recommended)",
                        func.name, snapshot.path, func.args.len(), max_args
                    ));
                }
            }
        }

        // Complexity: Struct field count
        if let Some(max_fields) = config.complexity.max_fields {
            for strct in &snapshot.structs {
                if strct.fields.len() > max_fields {
                    warnings.push(format!(
                        "Warning: Struct '{}' in '{}' has {} fields (max {} recommended)",
                        strct.name, snapshot.path, strct.fields.len(), max_fields
                    ));
                }
            }
        }

        // Naming: Function snake_case
        if config.naming.enforce_snake_case_functions == Some(true) {
            for func in &snapshot.functions {
                if !is_snake_case(&func.name) {
                    warnings.push(format!(
                        "Warning: Function '{}' in '{}' should use snake_case",
                        func.name, snapshot.path
                    ));
                }
            }
        }

        // Naming: Variable snake_case
        if config.naming.enforce_snake_case_variables == Some(true) {
            for func in &snapshot.functions {
                for (var_name, _var_type) in &func.variables {
                    if !is_snake_case(var_name) {
                        warnings.push(format!(
                            "Warning: Variable '{}' in function '{}' ('{}') should use snake_case",
                            var_name, func.name, snapshot.path
                        ));
                    }
                }
            }
        }

        // Naming: Type PascalCase
        if config.naming.enforce_pascal_case_types == Some(true) {
            for strct in &snapshot.structs {
                if !is_pascal_case(&strct.name) {
                    warnings.push(format!(
                        "Warning: Struct '{}' in '{}' should use PascalCase",
                        strct.name, snapshot.path
                    ));
                }
            }
            for enm in &snapshot.enums {
                if !is_pascal_case(&enm.name) {
                    warnings.push(format!(
                        "Warning: Enum '{}' in '{}' should use PascalCase",
                        enm.name, snapshot.path
                    ));
                }
            }
        }

        // Naming: Discouraged names
        if let Some(discouraged) = &config.naming.discouraged_names {
            for func in &snapshot.functions {
                for (var_name, _var_type) in &func.variables {
                    if discouraged.contains(var_name) {
                        warnings.push(format!(
                            "Warning: Discouraged variable name '{}' in function '{}' ('{}')",
                            var_name, func.name, snapshot.path
                        ));
                    }
                }
            }
        }
    }

    warnings
}

fn is_snake_case(s: &str) -> bool {
    // Allow leading underscore
    let s = s.trim_start_matches('_');
    //  Should be all lowercase with underscores
    s.chars().all(|c| c.is_lowercase() || c.is_numeric() || c == '_')
}

fn is_pascal_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    // First character should be uppercase
    let mut chars = s.chars();
    if let Some(first) = chars.next() {
        first.is_uppercase()
    } else {
        false
    }
}