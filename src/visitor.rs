use crate::types::{FunctionSnapshot, StructSnapshot, TraitSnapshot, EnumSnapshot, ImplSnapshot};
use syn::{visit::Visit, ItemFn, ItemStruct, ItemTrait, Pat, ItemEnum, ItemImpl, ImplItem, Type};
use quote::ToTokens;

#[derive(Default)]
pub struct SnapshotVisitor {
    pub functions: Vec<FunctionSnapshot>,
    pub structs: Vec<StructSnapshot>,
    pub traits: Vec<TraitSnapshot>,
    pub enums: Vec<EnumSnapshot>,
    pub impls: Vec<ImplSnapshot>,
}

impl Visit<'_> for SnapshotVisitor {
    fn visit_item_fn(&mut self, node: &'_ ItemFn) {
        let name = node.sig.ident.to_string();
        
        let mut args = Vec::new();
        for input in &node.sig.inputs {
             if let syn::FnArg::Typed(pat_type) = input {
                 if let Pat::Ident(pat_ident) = &*pat_type.pat {
                     args.push(pat_ident.ident.to_string());
                 }
             }
        }

        let mut variables = Vec::new();
        for stmt in &node.block.stmts {
            if let syn::Stmt::Local(local) = stmt {
                let (pat, ty_str) = match &local.pat {
                    Pat::Type(pat_type) => (&*pat_type.pat, Some((&*pat_type.ty).to_token_stream().to_string())),
                    p => (p, None),
                };

                if let Pat::Ident(pat_ident) = pat {
                    let var_name = pat_ident.ident.to_string();
                    variables.push((var_name, ty_str));
                }
            }
        }
        self.functions.push(FunctionSnapshot { name, args, variables });
    }

    fn visit_item_struct(&mut self, node: &'_ ItemStruct) {
        let name = node.ident.to_string();
        let mut fields = Vec::new();
        if let syn::Fields::Named(fields_named) = &node.fields {
            for field in &fields_named.named {
                if let Some(ident) = &field.ident {
                    fields.push(ident.to_string());
                }
            }
        }
        self.structs.push(StructSnapshot { name, fields, methods: Vec::new() });
    }

    fn visit_item_trait(&mut self, node: &'_ ItemTrait) {
        let name = node.ident.to_string();
        let mut methods = Vec::new();
        for item in &node.items {
            if let syn::TraitItem::Fn(method) = item {
                methods.push(method.sig.ident.to_string());
            }
        }
        self.traits.push(TraitSnapshot { name, methods });
    }

    fn visit_item_enum(&mut self, node: &'_ ItemEnum) {
        let name = node.ident.to_string();
        let mut variants = Vec::new();
        for variant in &node.variants {
            variants.push(variant.ident.to_string());
        }
        self.enums.push(EnumSnapshot { name, variants, methods: Vec::new() });
    }

    fn visit_item_impl(&mut self, node: &'_ ItemImpl) {
        let for_type = if let Type::Path(path) = *node.self_ty.clone() {
            path.path.get_ident().map(|ident| ident.to_string())
        } else {
            None
        };

        let trait_name = node.trait_.as_ref().map(|( _, path, _ )| path.segments.last().map(|seg| seg.ident.to_string())).flatten();

        let mut methods = Vec::new();
        for item in &node.items {
            if let ImplItem::Fn(method) = item {
                methods.push(method.sig.ident.to_string());
            }
        }

        if let Some(for_type) = for_type {
            self.impls.push(ImplSnapshot { for_type, trait_name, methods });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_file;

    #[test]
    fn test_function_extraction() {
        let code = r#"
            fn my_func() {
                let x = 10;
            }
        "#;
        let file = parse_file(code).unwrap();
        let mut visitor = SnapshotVisitor::default();
        visitor.visit_file(&file);

        assert_eq!(visitor.functions.len(), 1);
        assert_eq!(visitor.functions[0].name, "my_func");
        assert_eq!(visitor.functions[0].variables.len(), 1);
        assert_eq!(visitor.functions[0].variables[0].0, "x");
    }

    #[test]
    fn test_struct_extraction() {
        let code = r#"
            struct MyStruct {
                field1: i32,
                field2: String,
            }
        "#;
        let file = parse_file(code).unwrap();
        let mut visitor = SnapshotVisitor::default();
        visitor.visit_file(&file);

        assert_eq!(visitor.structs.len(), 1);
        assert_eq!(visitor.structs[0].name, "MyStruct");
        assert_eq!(visitor.structs[0].fields.len(), 2);
        assert_eq!(visitor.structs[0].fields[0], "field1");
        assert_eq!(visitor.structs[0].fields[1], "field2");
    }
}
