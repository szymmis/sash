#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Kind {
    Let,
    If,
    While,
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

impl Kind {
    fn from_char(char: char) -> Option<Self> {
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
            "while" => Some(Self::While),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub lexeme: String,
    pub kind: Kind,
}

impl Token {
    pub fn write(&self) -> String {
        match self.kind {
            Kind::String => format!("\"{}\"", self.lexeme),
            Kind::Identifier => format!("${}", self.lexeme),
            Kind::Less => "-lt".into(),
            Kind::LessEqual => "-lte".into(),
            Kind::Greater => "-gt".into(),
            Kind::GreaterEqual => "-gte".into(),
            Kind::EqualEqual => "-eq".into(),
            Kind::NotEqual => "-ne".into(),
            _ => self.lexeme.clone(),
        }
    }

    pub fn from_char(char: char) -> Option<Self> {
        Some(Token {
            lexeme: char.into(),
            kind: Kind::from_char(char)?,
        })
    }

    pub fn from_keyword(str: &str) -> Option<Self> {
        Some(Token {
            lexeme: str.into(),
            kind: Kind::from_keyword(str)?,
        })
    }
}
