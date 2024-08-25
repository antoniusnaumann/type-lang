use std::borrow::Cow;

use crate::parser::{Field, Type, TypeItem};

pub mod gleam;
pub mod rust;

#[derive(Debug, PartialEq, Eq)]
pub struct OutputFile {
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

        let file = OutputFile {
            name: self.to_file_name(&ty.ident),
            content,
        };
        self.add_type_boilerplate(ty, &file);

        self.types().push(file);

        self.reset();
    }

    /// An optional hook to add boilerplate after the type has been created
    fn add_type_boilerplate(&mut self, _ty: &Type, _file: &OutputFile) {}

    /// Create encoder code. This is not needed for languages with decorator-based serialization.
    fn create_decoder(&mut self, _ty: &Type) -> String {
        "".to_owned()
    }

    /// The separator between struct members in the target language    
    fn field_separator(&self) -> &'static str;

    fn file_extension(&self) -> &'static str;

    /// Finalize builder and return the created type files
    fn generate(self) -> Vec<OutputFile>;

    fn generate_declaration(&self, ident: &str, fields: &str) -> String;

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

    fn output_dyn<'a>(&'a self) -> Box<dyn Iterator<Item = &OutputFile> + 'a>;

    /// Resets the builder between types, e.g. resets flags
    fn reset(&mut self) {}

    /// Takes an identifier string and turns it into a valid identifier in the target language, escaping it if neccessary
    fn sanitize_ident<'a>(&self, ident: &'a str) -> Cow<'a, str>;

    /// Converts a name into the file naming convention for this language
    fn to_file_name(&self, name: &str) -> String;

    /// Mutable list of types in this builder. Should not include boilerplate files like module definitions
    fn types(&mut self) -> &mut Vec<OutputFile>;
}
