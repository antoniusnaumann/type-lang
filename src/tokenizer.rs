pub struct Tokenizer<'a> {
    source: &'a str,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            position: 0,
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let slice = &self.source[self.position..];
        let mut start = self.position;

        for ch in slice.chars() {
            if ch.is_whitespace() {
                start += 1;
            } else {
                break;
            }
        }

        self.position = start + 1;
        let slice = &self.source[start..];
        match slice.chars().nth(0) {
            Some('[') => Some(Token::BracketOpen),
            Some(']') => Some(Token::BracketClose),
            Some('{') => Some(Token::BraceOpen),
            Some('}') => Some(Token::BraceClose),
            Some('(') => Some(Token::ParenOpen),
            Some(')') => Some(Token::ParenClose),
            Some('?') => Some(Token::QuestionMark),
            Some(',') => Some(Token::Comma),
            Some(':') => Some(Token::Colon),
            Some(c) if c.is_alphabetic() => {
                let end = slice
                    .find(|c: char| !c.is_alphanumeric())
                    .unwrap_or(slice.len());
                let ident = &slice[..end];
                self.position = start + end;

                if ident.starts_with(|c: char| c.is_uppercase()) {
                    Some(Token::TypeIdent(ident))
                } else {
                    Some(Token::Ident(ident))
                }
            }
            Some(c) => todo!("Tokenizer error: {c} is not a valid token!"),
            None => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    ParenOpen,
    ParenClose,

    QuestionMark,
    Comma,
    Colon,

    TypeIdent(&'a str),
    Ident(&'a str),

    TypeKeyword,
}

impl<'a> Token<'a> {
    /// Try to convert this token into the equivalent keyword
    ///
    /// Returns the keyword token, if the receiver would be a valid keyword, otherwise returns the receiver unchanged
    /// This is useful to lift contextual keywords into their keyword form
    pub fn into_keyword(self) -> Token<'a> {
        match self {
            Token::Ident(ident) if ident.trim() == "type" => Token::TypeKeyword,
            _ => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_empty_type() {
        let source = "
          Example {}  
        ";

        let lexer = Tokenizer::new(source);
        let tokens: Vec<_> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Token::TypeIdent("Example"),
                Token::BraceOpen,
                Token::BraceClose
            ]
        );
    }

    #[test]
    fn test_tokenize_list() {
        let source = "[Object]";

        let lexer = Tokenizer::new(source);
        let tokens: Vec<_> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Token::BracketOpen,
                Token::TypeIdent("Object"),
                Token::BracketClose
            ]
        )
    }

    #[test]
    fn test_multiple_idents() {
        let source = "Object ident otherName identWithNumber123 Type123";

        let lexer = Tokenizer::new(source);
        let tokens: Vec<_> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                Token::TypeIdent("Object"),
                Token::Ident("ident"),
                Token::Ident("otherName"),
                Token::Ident("identWithNumber123"),
                Token::TypeIdent("Type123")
            ]
        );
    }

    #[test]
    fn test_empty_string() {
        let source = "";

        let lexer = Tokenizer::new(source);
        let tokens: Vec<_> = lexer.collect();

        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn test_space_string() {
        let source = "          ";

        let lexer = Tokenizer::new(source);
        let tokens: Vec<_> = lexer.collect();

        assert_eq!(tokens, vec![]);
    }

    #[test]
    #[should_panic]
    fn test_unknown_token() {
        let source = "ident $Type";

        let lexer = Tokenizer::new(source);
        let _tokens: Vec<_> = lexer.collect();
    }

    #[test]
    fn test_convert_type_keyword() {
        let token = Token::Ident("type");
        assert_eq!(token.into_keyword(), Token::TypeKeyword);
        assert_ne!(token.into_keyword(), Token::Ident("type"));
        assert_eq!(token, Token::Ident("type"));
    }

    #[test]
    fn test_convert_type_keyword_no_match() {
        let token = Token::Ident("noKeyword");
        assert_eq!(token.into_keyword(), Token::Ident("noKeyword"));
        assert_ne!(token.into_keyword(), Token::TypeKeyword);
    }
}
