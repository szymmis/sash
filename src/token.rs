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

impl TokenKind {
    fn from_char(char: &char) -> Option<Self> {
        match char {
            '(' => Some(Self::LeftBracket),
            ')' => Some(Self::RightBracket),
            ',' => Some(Self::Coma),
            '.' => Some(Self::Period),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub lexeme: String,
    pub kind: TokenKind,
}

impl Token {
    pub fn write(&self) -> String {
        match self.kind {
            TokenKind::String => format!("\"{}\"", self.lexeme),
            _ => self.lexeme.to_owned(),
        }
    }

    pub fn from_char(char: char) -> Option<Self> {
        match TokenKind::from_char(&char) {
            Some(kind) => Some(Token {
                lexeme: char.to_string(),
                kind,
            }),
            None => None,
        }
    }
}
