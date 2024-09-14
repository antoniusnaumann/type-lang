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
        while let Some(ty) = self.parse_declaration() {
            types.push(ty);
        }

        types
    }

    fn parse_declaration(&mut self) -> Option<Type> {
        let token = self.lexer.next_skip_newline()?;
        let (span, token) = match token.token {
            TokenKind::Ident(ident) => {
                if token.into_keyword() != TokenKind::TypeKeyword {
                    todo!("Parser Error: Expected type keyword, found ident '{ident}'")
                }
                (span, token);
            }
            other => todo!("pARS"),
        };

        let Some(TokenKind::TypeIdent(ident)) = self.lexer.next_skip_newline() else {
            todo!("Parser Error: Expected type ident")
        };
        let ident = ident.into();

        let Some(TokenKind::BraceOpen) = self.lexer.next_skip_newline() else {
            todo!("Parser Error: Expected '{{', found ...")
        };

        let mut fields = Vec::new();
        loop {
            match self.lexer.peek() {
                Some(TokenKind::BraceClose) => {
                    self.lexer.next_skip_newline();
                    break;
                }
                Some(_) => fields.push(self.parse_field()),
                None => todo!("Parser Error: Missing closing brace!"),
            }
        }

        Some(Type { ident, fields })
    }

    fn parse_field(&mut self) -> Field {
        let Some(TokenKind::Ident(ident)) = self.lexer.next_skip_newline() else {
            todo!("Parser Error: Expected ident!")
        };
        let ident = ident.into();

        let Some(TokenKind::Colon) = self.lexer.next_skip_newline() else {
            todo!("Parser Error: Expected colon after field name")
        };

        let ty = self.parse_type_item();

        while self.lexer.peek().is_some_and(|t| t.is_delim()) {
            self.lexer.next();
        }

        Field { ident, ty }
    }

    fn parse_type_item(&mut self) -> TypeItem {
        let mut ty = match self.lexer.peek() {
            Some(token) => {
                let Token { span, kind: token } = token;
                match token {
                    TokenKind::BraceOpen => {
                        self.lexer.next();
                        let key = self.parse_type_item().into();
                        let Some(TokenKind::Colon) = self.lexer.next() else {
                            todo!("Parser Error: Expected colon after dict key type!")
                        };
                        let value = self.parse_type_item().into();
                        let Some(TokenKind::BraceClose) = self.lexer.next() else {
                            todo!("Parser Error: Missing closing brace!")
                        };

                        TypeItem::Dict { key, value }
                    }
                    TokenKind::BracketOpen => {
                        self.lexer.next();
                        let element = self.parse_type_item();
                        let Some(TokenKind::BracketClose) = self.lexer.next() else {
                            todo!("Parser Error: Missing closing bracket!")
                        };

                        TypeItem::Array(element.into())
                    }
                    TokenKind::ParenOpen => todo!("Parse tuple type"),
                    TokenKind::TypeIdent(_) => {
                        let Some(TokenKind::TypeIdent(ident)) = self.lexer.next_skip_newline()
                        else {
                            unreachable!()
                        };
                        TypeItem::Basic(ident.into())
                    }
                    token => todo!("Parser Error: Expected type item, found {:#?}", token),
                }
            }
            None => todo!("Parser Error: Expected type item, found EOF!"),
        };

        while let Some(_) = self.lexer.next_if(|&t| *t == TokenKind::QuestionMark) {
            ty = TypeItem::Optional(Box::new(ty));
        }

        return ty;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Type {
    pub ident: String,
    pub fields: Vec<Field>,
    // span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub ident: String,
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

        assert_eq!(&ty.ident, "Empty");
        assert_eq!(ty.fields, vec![]);
    }

    #[test]
    fn test_parse_empty_file() {
        let source = "          ";
        let mut parser = Parser::new(source);

        assert_eq!(parser.parse_declaration(), None);
    }

    #[test]
    fn test_parse_newline_separated() {
        let source = "type Fields {
            a: Int
            b: String
        }";
        let mut parser = Parser::new(source);
        let ty = parser.parse_declaration().unwrap();

        assert_eq!(ty.ident, "Fields");
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
