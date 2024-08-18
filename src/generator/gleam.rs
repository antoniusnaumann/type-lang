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
    needs_map: bool,
}

impl Generator for GleamTypeGenerator {
    fn add_type(&mut self, ty: &Type) {
        let fields = ty
            .fields
            .iter()
            .map(|f| format!("{}: {}", f.ident, self.generate_type_item(&f.ty)))
            .collect::<Vec<_>>()
            .join(", ");

        let content = format!(
            "{}\npub type {} {{\n  {}({fields})\n}}",
            self.generate_imports(),
            ty.ident,
            ty.ident,
        )
        .trim()
        .to_owned();

        // TODO: Generate decoder code

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
        self.needs_map = false;
    }

    fn generate_type_item(&mut self, ty: &TypeItem) -> String {
        match ty {
            TypeItem::Array(items) => {
                format!("List({})", self.generate_type_item(items))
            }
            TypeItem::Dict { key, value } => {
                self.needs_map = true;
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

        if self.needs_option {
            imports.push("import gleam/option.{type Option}");
        }

        if self.needs_map {
            imports.push("import gleam/dict.{type Dict}");
        }

        imports.join("\n")
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
                content: "pub type Empty {\n  Empty()\n}".to_owned()
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
                content: "pub type Container {\n  Container(a: Int)\n}".to_owned()
            }]
        );
    }
}
