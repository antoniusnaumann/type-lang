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
        let slice = &slice[start..];
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
            Some(_) => todo!("Parse Ident, Type Ident and type keyword"),
            None => None,
        }
    }
}

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
    TypeKeyword,

    TypeIdent(&'a str),
    Ident(&'a str),
}
