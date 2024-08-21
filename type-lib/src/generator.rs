use crate::parser::Type;

mod gleam;

pub trait Generator {
    /// Generates a type declaration and adds it to the internal state
    fn add_type(&mut self, ty: &Type);
}
