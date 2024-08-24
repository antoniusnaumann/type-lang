use std::borrow::Cow;

use crate::parser::{Field, Type, TypeItem};

pub mod gleam;
pub mod rust;

#[derive(Debug, PartialEq, Eq)]
pub struct TypeFile {
    pub name: String,
    pub content: String,
}

pub trait Generator {
    /// Generates a type declaration and adds it to the internal state
    fn add_type(&mut self, ty: &Type) {
        let fields = self.generate_fields(ty);

        let decoder = self.create_decoder(ty);

        let declaration = self.generate_declaration(&ty.ident, &fields);
        let content = format!(
            "{}\n\n{declaration}\n\n{decoder}\n",
            self.generate_imports(),
        )
        .trim()
        .to_owned();

        self.types().push(TypeFile {
            name: ty.ident.clone(),
            content,
        });

        self.reset();
    }

    fn generate_declaration(&self, ident: &str, fields: &str) -> String;

    /// Create encoder code. This is not needed for languages with decorator-based serialization.
    fn create_decoder(&mut self, _ty: &Type) -> String {
        "".to_owned()
    }

    /// The separator between struct members in the target language    
    fn field_separator(&self) -> &'static str;

    fn file_extension(&self) -> &'static str;

    /// Finalize builder and return the created type files
    fn generate(self) -> Vec<TypeFile>;

    fn generate_field(&mut self, field: &Field) -> String;

    /// Generate struct fields
    fn generate_fields(&mut self, ty: &Type) -> String {
        ty.fields
            .iter()
            .map(|f| self.generate_field(f))
            .collect::<Vec<_>>()
            .join(self.field_separator())
    }

    /// Generates imports that are needed for struct declaration or encoder code
    fn generate_imports(&self) -> String {
        "".to_owned()
    }

    /// Generate a type annotation
    fn generate_type_item(&mut self, ty: &TypeItem) -> String;

    /// Resets the builder between types, e.g. resets flags
    fn reset(&mut self) {}

    /// Takes an identifier string and turns it into a valid identifier in the target language, escaping it if neccessary
    fn sanitize_ident<'a>(&self, ident: &'a str) -> Cow<'a, str>;

    /// Mutable list of types in this builder
    fn types(&mut self) -> &mut Vec<TypeFile>;
}
