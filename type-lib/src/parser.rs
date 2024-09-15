use crate::tokenizer::{Token, TokenKind, Tokenizer};

pub struct Parser<'a> {
    lexer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: Tokenizer::new(source),
        }
    }

    pub fn parse(&mut self) -> Vec<Type> {
        let mut types = vec![];
        while let Ok(ty) = self.parse_declaration() {
            types.push(ty);
        }

        types
    }

    fn parse_declaration(&mut self) -> Result<Type, Token> {
        self.lexer.expect(TokenKind::TypeKeyword)?;
        let ident = self.lexer.expect(TokenKind::TypeIdent)?;

        self.lexer.expect(TokenKind::BraceOpen)?;

        let mut fields = Vec::new();
        loop {
            match self.lexer.peek() {
                TokenKind::BraceClose => {
                    self.lexer.next_skip_newline();
                    break;
                }
                TokenKind::EOF => todo!("Parser Error: Missing closing brace!"),
                _ => fields.push(self.parse_field()?),
            }
        }

        Ok(Type {
            ident: ident.str.into(),
            fields,
        })
    }

    fn parse_field(&mut self) -> Result<Field, Token> {
        let ident = self.lexer.expect(TokenKind::Ident)?;
        self.lexer.expect(TokenKind::Colon)?;

        let ty = self.parse_type_item()?;

        while self.lexer.peek().is_delim() {
            self.lexer.next();
        }

        Ok(Field {
            ident: ident.str,
            ty,
        })
    }

    fn parse_type_item(&mut self) -> Result<TypeItem, Token> {
        let mut ty = match self.lexer.peek() {
            TokenKind::BraceOpen => {
                self.lexer.expect(TokenKind::BraceOpen).unwrap();
                let key = self.parse_type_item()?.into();
                self.lexer.expect(TokenKind::Colon)?;
                let value = self.parse_type_item()?.into();
                self.lexer.expect(TokenKind::BraceClose)?;

                TypeItem::Dict { key, value }
            }
            TokenKind::BracketOpen => {
                self.lexer.expect(TokenKind::BracketOpen).unwrap();
                let element = self.parse_type_item()?.into();
                self.lexer.expect(TokenKind::BracketClose)?;

                TypeItem::Array(element)
            }
            TokenKind::ParenOpen => todo!("Parse tuple type"),
            TokenKind::TypeIdent => {
                let ident = self.lexer.expect(TokenKind::TypeIdent).unwrap();

                TypeItem::Basic(ident.str.into())
            }
            TokenKind::EOF => todo!("Parser Error: Expected type item, found EOF!"),
            token => todo!("Parser Error: Expected type item, found {:#?}", token),
        };

        while let Some(_) = self.lexer.try_next(TokenKind::QuestionMark) {
            ty = TypeItem::Optional(Box::new(ty));
        }

        return Ok(ty);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Type {
    pub ident: Box<str>,
    pub fields: Vec<Field>,
    // span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub ident: Box<str>,
    pub ty: TypeItem,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeItem {
    Array(Box<TypeItem>),
    Dict {
        key: Box<TypeItem>,
        value: Box<TypeItem>,
    },
    Optional(Box<TypeItem>),

    Basic(String),
}

#[cfg(test)]
mod test {
    use std::ops::Deref;

    use super::*;

    #[test]
    #[should_panic]
    fn test_reject_unclosed() {
        let source = "type Test {";
        let mut parser = Parser::new(source);
        parser.parse_declaration().unwrap();
    }

    #[test]
    fn test_parse_empty_type() {
        let source = "type Empty {}";
        let mut parser = Parser::new(source);
        let ty = parser.parse_declaration().unwrap();

        assert_eq!(ty.ident.deref(), "Empty");
        assert_eq!(ty.fields, vec![]);
    }

    #[test]
    fn test_parse_empty_file() {
        let source = "          ";
        let mut parser = Parser::new(source);

        assert!(matches!(parser.parse_declaration(), Err(_)));
    }

    #[test]
    fn test_parse_newline_separated() {
        let source = "type Fields {
            a: Int
            b: String
        }";
        let mut parser = Parser::new(source);
        let ty = parser.parse_declaration().unwrap();

        assert_eq!(ty.ident.deref(), "Fields");
        assert_eq!(
            ty.fields,
            vec![
                Field {
                    ident: "a".into(),
                    ty: TypeItem::Basic("Int".into())
                },
                Field {
                    ident: "b".into(),
                    ty: TypeItem::Basic("String".into())
                }
            ]
        );
    }
}
