use crate::parser::Type;

pub mod gleam;

#[derive(Debug, PartialEq, Eq)]
pub struct TypeFile {
    pub name: String,
    pub content: String,
}

pub trait Generator {
    /// Generates a type declaration and adds it to the internal state
    fn add_type(&mut self, ty: &Type);

    fn generate(self) -> Vec<TypeFile>;
}
