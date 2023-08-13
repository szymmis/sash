#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Let,
    If,
    Else,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Not,
    NotEqual,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Identifier,
    Command,
    Comment,
    String,
    RawString,
    Number,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Coma,
    Period,
    Option,
}

impl TokenKind {
    fn from_char(char: &char) -> Option<Self> {
        match char {
            '(' => Some(Self::LeftParen),
            ')' => Some(Self::RightParen),
            '{' => Some(Self::LeftBracket),
            '}' => Some(Self::RightBracket),
            ',' => Some(Self::Coma),
            '.' => Some(Self::Period),
            '+' => Some(Self::Plus),
            // '-' => Some(Self::Minus), See Lexer::match_option
            '*' => Some(Self::Asterisk),
            '/' => Some(Self::Slash),
            _ => None,
        }
    }

    fn from_keyword(str: &str) -> Option<Self> {
        match str {
            "let" => Some(Self::Let),
            "if" => Some(Self::If),
            "else" => Some(Self::Else),
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
            TokenKind::Identifier => format!("${}", self.lexeme),
            TokenKind::Less => "-lt".into(),
            TokenKind::LessEqual => "-lte".into(),
            TokenKind::Greater => "-gt".into(),
            TokenKind::GreaterEqual => "-gte".into(),
            TokenKind::EqualEqual => "-eq".into(),
            TokenKind::NotEqual => "-ne".into(),
            _ => self.lexeme.to_owned(),
        }
    }

    pub fn from_char(char: char) -> Option<Self> {
        Some(Token {
            lexeme: char.into(),
            kind: TokenKind::from_char(&char)?,
        })
    }

    pub fn from_keyword(str: &str) -> Option<Self> {
        Some(Token {
            lexeme: str.into(),
            kind: TokenKind::from_keyword(&str)?,
        })
    }
}
