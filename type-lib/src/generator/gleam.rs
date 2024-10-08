use std::{borrow::Cow, collections::HashSet};

use crate::parser::{Field, Type, TypeItem};

use super::{Generator, OutputFile};

pub struct GleamTypeGenerator {
    types: Vec<OutputFile>,

    needs_option: bool,
    needs_dict: bool,
    /// Generated types that are used by fields of the current type
    used_types: HashSet<String>,

    /// The module folder name that the generated types should be located in
    module_name: String,
}

impl Generator for GleamTypeGenerator {
    fn create_decoder(&mut self, ty: &Type) -> String {
        let mut use_statements = Vec::new();
        let mut constructor_params = Vec::new();
        let mut field_decoders = Vec::new();

        for field in &ty.fields {
            let decode_type = self.type_item_decoder(&field.ty);
            use_statements.push(format!("use {} <- decode.parameter", field.ident));
            constructor_params.push(field.ident.clone());
            field_decoders.push(format!(
                "|> decode.field(\"{}\", {decode_type})",
                field.ident
            ));
        }

        let use_statements = use_statements.join("\n\t\t");
        let constructor_params = constructor_params.join(", ");
        let field_decoders = field_decoders.join("\n\t");

        format!(
        "pub fn decode(data: Dynamic) {{\n\tlet decoder = decode.into({{\n\t\t{use_statements}\n\n\t\t{}({constructor_params})\n\t}})\n\t{field_decoders}\n\n\tdecoder |> decode.from(data)\n}}",
        ty.ident,
    )
    }

    fn field_separator(&self) -> &'static str {
        ", "
    }

    fn file_extension(&self) -> &'static str {
        "gleam"
    }

    fn generate(self) -> Vec<OutputFile> {
        self.types
    }

    fn generate_declaration(&self, ident: &str, fields: &str) -> String {
        format!("pub type {ident} {{\n\t{ident}({fields})\n}}")
    }

    fn generate_field(&mut self, field: &Field) -> String {
        format!("{}: {}", field.ident, self.generate_type_item(&field.ty))
    }

    fn generate_imports(&self) -> String {
        let mut imports = Vec::<Cow<str>>::new();

        imports.push("import gleam/decode".into());
        if self.needs_option {
            imports.push("import gleam/option.{type Option}".into());
        }

        if self.needs_dict {
            imports.push("import gleam/dict.{type Dict}".into());
        }

        for ty in &self.used_types {
            imports.push(
                format!(
                    "import {}/{}.{{type {}}}",
                    self.module_name,
                    self.to_file_name(ty),
                    ty
                )
                .into(),
            )
        }

        imports.join("\n")
    }

    fn generate_type_item(&mut self, ty: &TypeItem) -> String {
        match ty {
            TypeItem::Array(items) => {
                format!("List({})", self.generate_type_item(items))
            }
            TypeItem::Dict { key, value } => {
                self.needs_dict = true;
                format!(
                    "Map({}, {})",
                    self.generate_type_item(key),
                    self.generate_type_item(value)
                )
            }
            TypeItem::Optional(inner) => {
                self.needs_option = true;
                format!("Option({})", self.generate_type_item(inner))
            }
            TypeItem::Basic(plain) => match plain.as_str() {
                "String" => "String".into(),
                "Int" | "UInt" | "Int8" | "UInt8" | "Int16" | "UInt16" | "Int32" | "UInt32"
                | "Int64" | "UInt64" | "ISize" | "USize" => "Int".into(),
                "Bool" => "Bool".into(),
                "Float" | "Double" => "Float".into(),
                ty => {
                    self.used_types.insert(ty.to_owned());
                    ty.to_owned()
                }
            },
        }
    }

    fn output_dyn<'a>(&'a self) -> Box<dyn Iterator<Item = &OutputFile> + 'a> {
        Box::new(self.types.iter())
    }

    fn reset(&mut self) {
        self.needs_option = false;
        self.needs_dict = false;
    }

    fn sanitize_ident<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        // TODO: Sanitize Ident
        ident.into()
    }

    fn to_file_name(&self, name: &str) -> String {
        // TODO: Convert to snake case
        name.to_lowercase()
    }

    fn push_type(&mut self, ty: OutputFile) {
        self.types.push(ty)
    }
}

impl Default for GleamTypeGenerator {
    fn default() -> Self {
        Self {
            types: Vec::new(),
            needs_option: false,
            needs_dict: false,
            used_types: HashSet::new(),
            module_name: "types".to_owned(),
        }
    }
}

impl GleamTypeGenerator {
    pub fn new(module_name: String) -> Self {
        Self {
            types: Vec::new(),
            needs_option: false,
            needs_dict: false,
            used_types: HashSet::new(),
            module_name,
        }
    }

    pub fn boxed() -> Box<Self> {
        Box::default()
    }

    fn type_item_decoder(&self, item: &TypeItem) -> Cow<str> {
        match item {
            TypeItem::Array(elements) => {
                format!("decode.list({})", self.type_item_decoder(elements)).into()
            }
            TypeItem::Dict { key, value } => format!(
                "decode.dict({}, {})",
                self.type_item_decoder(key),
                self.type_item_decoder(value)
            )
            .into(),
            TypeItem::Optional(inner) => {
                format!("decode.optional({})", self.type_item_decoder(inner)).into()
            }
            TypeItem::Basic(plain) => match plain.as_str() {
                "String" => "decode.string".into(),
                "Int" | "UInt" | "Int8" | "UInt8" | "Int16" | "UInt16" | "Int32" | "UInt32"
                | "Int64" | "UInt64" => "decode.int".into(),
                "Bool" => "decode.bool".into(),
                "Float" | "Double" => "decode.float".into(),
                // This decoder relies on the fact that the other types module will be imported due to the type being used in the struct declaration
                ty => format!("{}.decode", self.to_file_name(ty)).into(),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        generator::{gleam::OutputFile, Generator},
        parser::Parser,
    };

    use super::GleamTypeGenerator;

    #[test]
    fn convert_empty() {
        let empty = "type Empty {}";
        let mut exporter = GleamTypeGenerator::default();
        let mut parser = Parser::new(empty);
        let ast = parser.parse();

        for ty in &ast {
            exporter.add_type(ty);
        }

        assert_eq!(
            exporter.types,
            vec![OutputFile {
                name: "empty".to_owned(),
                content: "import gleam/decode\n\npub type Empty {\n\tEmpty()\n}\n\npub fn decode(data: Dynamic) {\n\tlet decoder = decode.into({\n\t\t\n\n\t\tEmpty()\n\t})\n\t\n\n\tdecoder |> decode.from(data)\n}".to_owned()
            }]
        );
    }

    #[test]
    fn convert_type_with_primitive_field() {
        let empty = "type Container { a: Int }";
        let mut exporter = GleamTypeGenerator::default();
        let mut parser = Parser::new(empty);
        let ast = parser.parse();

        for ty in &ast {
            exporter.add_type(ty);
        }

        assert_eq!(
            exporter.types,
            vec![OutputFile {
                name: "container".to_owned(),
                content: "import gleam/decode\n\npub type Container {\n\tContainer(a: Int)\n}\n\npub fn decode(data: Dynamic) {\n\tlet decoder = decode.into({\n\t\tuse a <- decode.parameter\n\n\t\tContainer(a)\n\t})\n\t|> decode.field(\"a\", decode.int)\n\n\tdecoder |> decode.from(data)\n}".to_owned()
            }]
        );
    }
}
