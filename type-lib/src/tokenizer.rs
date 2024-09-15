use std::ops::{Range, RangeInclusive};

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

    /// Returns the next token kind and range in the source without mutating the position directly
    fn next_kind(&self) -> (TokenKind, Range<usize>) {
        if self.position >= self.source.len() {
            return (TokenKind::EOF, self.position..self.position);
        }

        let slice = &self.source[self.position..];
        let mut start = self.position;

        for ch in slice.chars() {
            if ch.is_whitespace() && ch != '\n' {
                start += 1;
            } else {
                break;
            }
        }

        let mut new_position = start + 1;

        let slice = &self.source[start..];
        let next = match slice.chars().nth(0) {
            Some('[') => TokenKind::BracketOpen,
            Some(']') => TokenKind::BracketClose,
            Some('{') => TokenKind::BraceOpen,
            Some('}') => TokenKind::BraceClose,
            Some('(') => TokenKind::ParenOpen,
            Some(')') => TokenKind::ParenClose,
            Some('?') => TokenKind::QuestionMark,
            Some(',') => TokenKind::Comma,
            Some(':') => TokenKind::Colon,
            Some('\n') => TokenKind::Newline,
            Some(c) if c.is_alphabetic() => {
                let end = slice
                    .find(|c: char| !c.is_alphanumeric() && c != '_')
                    .unwrap_or(slice.len());
                let ident = &slice[..end];
                new_position = start + end;

                if ident.starts_with(|c: char| c.is_uppercase()) {
                    TokenKind::TypeIdent
                } else {
                    TokenKind::Ident
                }
            }
            Some(_) => TokenKind::Invalid,
            None => {
                new_position = start;
                TokenKind::EOF
            }
        };

        (next, start..new_position)
    }

    pub fn next(&mut self) -> Token {
        let (token, range) = self.next_kind();
        self.position = range.end;
        token.at(range.clone(), &self.source[range])
    }

    /// Advances the lexer only if the token matches the expected token kind, including newlines
    pub fn try_next(&mut self, expected: TokenKind) -> Option<Token> {
        let (token, range) = self.next_kind();
        if token == expected {
            self.position = range.end;
            Some(token.at(range.clone(), &self.source[range]))
        } else {
            None
        }
    }

    pub fn peek(&self) -> TokenKind {
        let (result, _) = self.next_kind();
        result
    }

    pub fn next_skip_newline(&mut self) -> Token {
        while self.peek() == TokenKind::Newline {
            self.next();
        }

        self.next()
    }

    /// Returns the token as Ok variant if it was the expected token, otherwise returns an error with the token that was encountered instead
    ///
    /// This method skips newlines
    pub fn expect(&mut self, token: TokenKind) -> Result<Token, Token> {
        let t = self.next_skip_newline();
        if t.kind == token {
            Ok(t)
        } else {
            let t = t.into_keyword();
            if t.kind == token {
                Ok(t)
            } else {
                Err(t)
            }
        }
    }

    #[cfg(test)]
    fn collect(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let (token, range) = self.next_kind();
            self.position = range.end;
            if token == TokenKind::EOF {
                break;
            }
            tokens.push((token, range))
        }

        tokens
            .into_iter()
            .map(|(kind, range)| Token {
                span: range.clone().into(),
                kind,
                str: self.source[range].into(),
            })
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span(RangeInclusive<usize>);

impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Span(value.start..=(value.end - 1))
    }
}

impl From<RangeInclusive<usize>> for Span {
    fn from(value: RangeInclusive<usize>) -> Self {
        Span(value)
    }
}

impl From<usize> for Span {
    fn from(value: usize) -> Self {
        Span(value..=value)
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
    pub str: Box<str>,
}

impl Token {
    /// Try to convert this token into the equivalent keyword
    ///
    /// Returns the keyword token, if the receiver would be a valid keyword, otherwise returns the receiver unchanged
    /// This is useful to lift contextual keywords into their keyword form
    pub fn into_keyword(self) -> Token {
        match self.kind {
            TokenKind::Ident if self.str.trim() == "type" => Token {
                span: self.span,
                kind: TokenKind::TypeKeyword,
                str: self.str,
            },
            _ => self,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    ParenOpen,
    ParenClose,

    Colon,
    QuestionMark,

    Comma,
    Newline,

    TypeIdent,
    Ident,

    TypeKeyword,

    Invalid,
    EOF,
}

impl TokenKind {
    pub fn at(self, span: impl Into<Span>, str: &str) -> Token {
        Token {
            span: span.into(),
            kind: self,
            str: str.into(),
        }
    }

    pub fn is_delim(self) -> bool {
        self == TokenKind::Comma || self == TokenKind::Newline
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl PartialEq for Token {
        fn eq(&self, other: &Self) -> bool {
            self.kind == other.kind && self.span == other.span
        }
    }
    impl Eq for Token {}

    #[test]
    fn test_tokenize_empty_type() {
        let source = "Example {}";

        let mut lexer = Tokenizer::new(source);
        let tokens: Vec<_> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                TokenKind::TypeIdent.at(0..=6, "Example"),
                TokenKind::BraceOpen.at(8, "{"),
                TokenKind::BraceClose.at(9, "}")
            ]
        );
    }

    #[test]
    fn test_tokenize_list() {
        let source = "[Object]";

        let mut lexer = Tokenizer::new(source);
        let tokens: Vec<_> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                TokenKind::BracketOpen.at(0, "["),
                TokenKind::TypeIdent.at(1..=6, "Object"),
                TokenKind::BracketClose.at(7, "]")
            ]
        )
    }

    #[test]
    fn test_multiple_idents() {
        let source = "Object ident otherName identWithNumber123 Type123";

        let mut lexer = Tokenizer::new(source);
        let tokens: Vec<_> = lexer.collect();

        assert_eq!(
            tokens,
            vec![
                TokenKind::TypeIdent.at(0..6, "Object"),
                TokenKind::Ident.at(7..12, "ident"),
                TokenKind::Ident.at(13..22, "otherName"),
                TokenKind::Ident.at(23..41, "identWithNumber123"),
                TokenKind::TypeIdent.at(42..49, "Type123")
            ]
        );
    }

    #[test]
    fn test_empty_string() {
        let source = "";

        let mut lexer = Tokenizer::new(source);
        let tokens: Vec<_> = lexer.collect();

        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn test_space_string() {
        let source = "          ";

        let mut lexer = Tokenizer::new(source);
        let tokens: Vec<_> = lexer.collect();

        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn test_unknown_token() {
        let source = "ident $Type";

        let mut lexer = Tokenizer::new(source);
        let tokens: Vec<_> = lexer.collect();

        assert_eq!(tokens[0].kind, TokenKind::Ident);
        assert_eq!(tokens[1].kind, TokenKind::Invalid);
        assert_eq!(tokens[2].kind, TokenKind::TypeIdent);
    }

    #[test]
    fn test_convert_type_keyword() {
        let token = TokenKind::Ident.at(0, "type");
        assert_eq!(token.clone().into_keyword().kind, TokenKind::TypeKeyword);
        assert_ne!(token.into_keyword().kind, TokenKind::Ident);
    }

    #[test]
    fn test_convert_type_keyword_no_match() {
        let token = TokenKind::Ident.at(0, "noKeyword");
        assert_eq!(token.clone().into_keyword().kind, TokenKind::Ident);
        assert_ne!(token.into_keyword().kind, TokenKind::TypeKeyword);
    }
}
