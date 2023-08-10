#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Identifier,
    Command,
    Comment,
    String,
    RawString,
    Number,
    LeftBracket,
    RightBracket,
    Coma,
    Period,
    Option,
}

impl Token {
    pub fn write(&self) -> String {
        match self.kind {
            TokenKind::String => format!("\"{}\"", self.lexeme),
            _ => self.lexeme.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub lexeme: String,
    pub kind: TokenKind,
}

impl Into<Token> for char {
    fn into(self) -> Token {
        Token {
            lexeme: self.to_string(),
            kind: match self {
                '(' => TokenKind::LeftBracket,
                ')' => TokenKind::RightBracket,
                ',' => TokenKind::Coma,
                '.' => TokenKind::Period,
                _ => panic!("Token from char {} not implemented!", self),
            },
        }
    }
}
