use std::iter::Peekable;

use crate::tokenizer::{Token, Tokenizer};

struct Parser<'a> {
    lexer: Peekable<Tokenizer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: Tokenizer::new(source).peekable(),
        }
    }

    fn parse_declaration(&mut self) -> Type<'a> {
        let Some(Token::TypeIdent(ident)) = self.lexer.next() else {
            todo!("Parser Error: Expected type ident")
        };

        let Some(Token::BraceOpen) = self.lexer.next() else {
            todo!("Parser Error: Expected '{{', found ...")
        };

        let mut fields = Vec::new();
        loop {
            match self.lexer.peek() {
                Some(Token::BraceClose) => {
                    self.lexer.next();
                    break;
                }
                Some(_) => fields.push(self.parse_field()),
                None => todo!("Parser Error: Missing closing brace!"),
            }
        }

        Type { ident, fields }
    }

    fn parse_field(&mut self) -> Field {}
}

struct Type<'a> {
    ident: &'a str,
    fields: Vec<Field>,
    // span: Span,
}

struct Field {}
