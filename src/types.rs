use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct RustFileSnapshot {
    pub path: String,
    pub functions: Vec<FunctionSnapshot>,
    pub structs: Vec<StructSnapshot>,
    pub traits: Vec<TraitSnapshot>,
    pub enums: Vec<EnumSnapshot>,
    pub impls: Vec<ImplSnapshot>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FunctionSnapshot {
    pub name: String,
    pub args: Vec<String>,
    pub variables: Vec<(String, Option<String>)>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StructSnapshot {
    pub name: String,
    pub fields: Vec<String>,
    pub methods: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TraitSnapshot {
    pub name: String,
    pub methods: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EnumSnapshot {
    pub name: String,
    pub variants: Vec<String>,
    pub methods: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ImplSnapshot {
    pub for_type: String,
    pub trait_name: Option<String>,
    pub methods: Vec<String>,
}
