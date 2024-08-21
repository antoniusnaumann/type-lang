use std::borrow::Cow;

use crate::parser::{Type, TypeItem};

use super::Generator;

#[derive(Debug, PartialEq, Eq)]
struct TypeFile {
    name: String,
    content: String,
}

#[derive(Default)]
pub struct GleamTypeGenerator {
    types: Vec<TypeFile>,

    needs_option: bool,
    needs_dict: bool,
}

impl Generator for GleamTypeGenerator {
    fn add_type(&mut self, ty: &Type) {
        let fields = ty
            .fields
            .iter()
            .map(|f| format!("{}: {}", f.ident, self.generate_type_item(&f.ty)))
            .collect::<Vec<_>>()
            .join(", ");

        let decoder = create_decoder(ty);

        let content = format!(
            "{}\npub type {} {{\n  {}({fields})\n}}\n{decoder}",
            self.generate_imports(),
            ty.ident,
            ty.ident,
        )
        .trim()
        .to_owned();

        self.types.push(TypeFile {
            name: ty.ident.clone(),
            content,
        });

        self.reset();
    }
}

impl GleamTypeGenerator {
    fn new() -> Self {
        Self::default()
    }

    fn reset(&mut self) {
        self.needs_option = false;
        self.needs_dict = false;
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
            TypeItem::Basic(ident) => ident.clone(),
        }
    }

    fn generate_imports(&self) -> String {
        let mut imports = vec![];

        imports.push("import gleam/decode");
        if self.needs_option {
            imports.push("import gleam/option.{type Option}");
        }

        if self.needs_dict {
            imports.push("import gleam/dict.{type Dict}");
        }

        imports.join("\n")
    }
}

fn create_decoder(ty: &Type) -> String {
    let mut use_statements = Vec::new();
    let mut constructor_params = Vec::new();
    let mut field_decoders = Vec::new();

    for field in &ty.fields {
        let decode_type = type_item_decoder(&field.ty);
        use_statements.push(format!("use {} <- decode.parameter", field.ident));
        constructor_params.push(field.ident.clone());
        field_decoders.push(format!(
            "|> decode.field(\"{}\", {decode_type})",
            field.ident
        ));
    }

    let use_statements = use_statements.join("\n");
    let constructor_params = constructor_params.join(", ");
    let field_decoders = field_decoders.join("\n");

    format!(
        "pub fn decode(data: Dynamic) {{\n\tlet decoder = decode.into({{\n\t\t{use_statements}\n\t\t{}({constructor_params})\n\t}})\n\t{field_decoders}\n\tdecoder |> decode.from(data)\n}}",
        ty.ident,
    )
}

fn type_item_decoder(item: &TypeItem) -> Cow<str> {
    match item {
        TypeItem::Array(elements) => format!("decode.list({})", type_item_decoder(elements)).into(),
        TypeItem::Dict { key, value } => format!(
            "decode.dict({}, {})",
            type_item_decoder(key),
            type_item_decoder(value)
        )
        .into(),
        TypeItem::Optional(inner) => {
            format!("decode.optional({})", type_item_decoder(inner)).into()
        }
        TypeItem::Basic(plain) => match plain.as_str() {
            "String" => "decode.string".into(),
            "Int" | "UInt" | "Int8" | "UInt8" | "Int16" | "UInt16" | "Int32" | "UInt32"
            | "Int64" | "UInt64" => "decode.int".into(),
            "Bool" => "decode.bool".into(),
            "Float" | "Double" => "decode.float".into(),
            plain => todo!("Implement decoding records!"),
        },
    }
}

#[cfg(test)]
mod test {
    use crate::{
        generator::{gleam::TypeFile, Generator},
        parser::Parser,
    };

    use super::GleamTypeGenerator;

    #[test]
    fn convert_empty() {
        let empty = "type Empty {}";
        let mut exporter = GleamTypeGenerator::new();
        let mut parser = Parser::new(empty);
        let ast = parser.parse();

        for ty in &ast {
            exporter.add_type(ty);
        }

        assert_eq!(
            exporter.types,
            vec![TypeFile {
                name: "Empty".to_owned(),
                content: "import gleam/decode\npub type Empty {\n  Empty()\n}\npub fn decode(data: Dynamic) {\n\tlet decoder = decode.into({\n\t\t\n\t\tEmpty()\n\t})\n\t\n\tdecoder |> decode.from(data)\n}".to_owned()
            }]
        );
    }

    #[test]
    fn convert_type_with_primitive_field() {
        let empty = "type Container { a: Int }";
        let mut exporter = GleamTypeGenerator::new();
        let mut parser = Parser::new(empty);
        let ast = parser.parse();

        for ty in &ast {
            exporter.add_type(ty);
        }

        assert_eq!(
            exporter.types,
            vec![TypeFile {
                name: "Container".to_owned(),
                content: "import gleam/decode\npub type Container {\n  Container(a: Int)\n}\npub fn decode(data: Dynamic) {\n\tlet decoder = decode.into({\n\t\tuse a <- decode.parameter\n\t\tContainer(a)\n\t})\n\t|> decode.field(\"a\", decode.int)\n\tdecoder |> decode.from(data)\n}".to_owned()
            }]
        );
    }
}
