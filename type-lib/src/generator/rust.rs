use std::{borrow::Cow, slice};

use crate::parser::{Field, TypeItem};

use super::{Generator, OutputFile};

pub struct RustTypeGenerator {
    module: OutputFile,
    types: Vec<OutputFile>,
}

impl Generator for RustTypeGenerator {
    fn field_separator(&self) -> &'static str {
        ",\n"
    }

    fn file_extension(&self) -> &'static str {
        "rs"
    }

    fn generate(self) -> Vec<OutputFile> {
        self.types
    }

    fn generate_declaration(&self, ident: &str, fields: &str) -> String {
        format!(
            "#[derive(serde::Serialize, serde::Deserialize)]\npub struct {ident} {{\n{fields}\n}}"
        )
    }

    fn generate_field(&mut self, field: &Field) -> String {
        format!(
            "\tpub {}: {}",
            field.ident,
            self.generate_type_item(&field.ty)
        )
    }

    fn generate_type_item(&mut self, ty: &TypeItem) -> String {
        match ty {
            TypeItem::Array(elements) => format!("Vec<{}>", self.generate_type_item(elements)),
            TypeItem::Dict { key, value } => format!(
                "::std::collections::HashMap<{}, {}>",
                self.generate_type_item(key),
                self.generate_type_item(value)
            ),
            TypeItem::Optional(inner) => format!("Option<{}>", self.generate_type_item(inner)),
            TypeItem::Basic(plain) => match plain.as_str() {
                "String" => "String".into(),
                "Int" | "Int64" => "i64".into(),
                "UInt" | "UInt64" => "u64".into(),
                "USize" => "usize".into(),
                "ISize" => "isize".into(),
                "Int8" => "i8".into(),
                "UInt8" => "u8".into(),
                "Int16" => "i16".into(),
                "UInt16" => "u16".into(),
                "Int32" => "i32".into(),
                "UInt32" => "u32".into(),
                "Bool" => "bool".into(),
                "Float" => "f32".into(),
                "Double" => "f64".into(),
                _ => todo!("Implement records"),
            },
        }
    }

    fn output_dyn<'a>(&'a self) -> Box<dyn Iterator<Item = &OutputFile> + 'a> {
        Box::new(self.types.iter().chain(std::iter::once(&self.module)))
    }

    fn sanitize_ident<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        match ident {
            // TODO: Escape more keywords
            "type" => format!("r#{ident}").into(),
            _ => ident.into(),
        }
    }

    fn types(&mut self) -> &mut Vec<OutputFile> {
        &mut self.types
    }
}

impl Default for RustTypeGenerator {
    fn default() -> Self {
        Self {
            module: OutputFile {
                name: "mod".to_owned(),
                content: "".to_owned(),
            },
            types: Vec::new(),
        }
    }
}

impl RustTypeGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn boxed() -> Box<Self> {
        Box::default()
    }
}
