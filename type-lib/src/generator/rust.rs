use crate::parser::{Type, TypeItem};

use super::{Generator, TypeFile};

#[derive(Default)]
pub struct RustTypeGenerator {
    types: Vec<TypeFile>,
}

impl Generator for RustTypeGenerator {
    fn add_type(&mut self, ty: &Type) {
        let fields = self.generate_fields(ty);
        let content = format!("pub struct {} {{\n{fields}\n}}", ty.ident);
    }

    fn generate(self) -> Vec<TypeFile> {
        self.types
    }

    fn generate_field(&mut self, field: &crate::parser::Field) -> String {
        todo!()
    }

    fn generate_type_item(&mut self, ty: &TypeItem) -> String {
        todo!()
    }

    fn field_separator() -> &'static str {
        todo!()
    }

    fn sanitize_ident(ident: &str) -> String {
        todo!()
    }

    fn types(&mut self) -> &mut Vec<TypeFile> {
        todo!()
    }
}
