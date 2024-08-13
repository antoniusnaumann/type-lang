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

    fn parse_declaration(&mut self) -> Option<Type<'a>> {
        if let Some(token @ Token::Ident(ident)) = self.lexer.next() {
            if token.into_keyword() != Token::TypeKeyword {
                todo!("Parser Error: Expected type keyword, found ident '{ident}'")
            }
        } else {
            return None;
        }

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

        Some(Type { ident, fields })
    }

    fn parse_field(&mut self) -> Field {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Type<'a> {
    ident: &'a str,
    fields: Vec<Field>,
    // span: Span,
}

#[derive(Debug, PartialEq, Eq)]
struct Field {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn test_reject_unclosed() {
        let source = "type Test {";
        let mut parser = Parser::new(source);
        parser.parse_declaration();
    }

    #[test]
    fn test_parse_empty_type() {
        let source = "type Empty {}";
        let mut parser = Parser::new(source);
        let ty = parser.parse_declaration().unwrap();

        assert_eq!(ty.ident, "Empty");
        assert_eq!(ty.fields, vec![]);
    }

    #[test]
    fn test_parse_empty_file() {
        let source = "          ";
        let mut parser = Parser::new(source);

        assert_eq!(parser.parse_declaration(), None);
    }
}
