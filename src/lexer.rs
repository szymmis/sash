use crate::token::{Token, TokenKind};

pub struct Lexer {
    index: usize,
    chars: Vec<char>,
}

impl Lexer {
    pub fn from_string(input: &str) -> Self {
        Lexer {
            index: 0,
            chars: input.chars().collect(),
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.index < self.chars.len() {
            match self.parse_char() {
                Some(token) => tokens.push(token),
                None => self.skip_char(),
            }
        }

        tokens
    }

    fn parse_char(&mut self) -> Option<Token> {
        match self.chars.get(self.index) {
            Some(char) => match char {
                '(' | ')' | ',' | '.' => Some(self.consume_any_char().unwrap().into()),
                '#' => self.match_comment(),
                '"' => self.match_string(),
                '`' => self.match_raw_string(),
                '-' => self.match_option(),
                '0'..='9' => self.match_number(),
                'a'..='z' | 'A'..='Z' => self.match_identifier(),
                _ => None,
            },
            None => None,
        }
    }

    fn get_char(&self) -> Option<&char> {
        self.chars.get(self.index)
    }

    fn skip_char(&mut self) {
        self.index += 1;
    }

    fn match_char(&self, char: char) -> Option<&char> {
        if *self.get_char()? == char {
            self.get_char()
        } else {
            None
        }
    }

    fn consume_any_char(&mut self) -> Option<char> {
        let char = self.get_char().cloned();
        self.skip_char();
        char
    }

    fn consume_char(&mut self, char: char) -> Option<char> {
        match self.match_char(char) {
            Some(_) => self.consume_any_char(),
            None => None,
        }
    }

    fn consume_lexeme_until<F>(&mut self, f: F) -> Option<String>
    where
        F: Fn(&char) -> bool,
    {
        let start = self.index.clone();

        loop {
            match self.get_char() {
                Some(char) => {
                    if f(char) {
                        self.consume_any_char();
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }

        Some(self.chars[start..self.index].into_iter().collect())
    }

    fn consume_lexeme_delimited(&mut self, char: char) -> Option<String> {
        self.consume_char(char);
        let lexeme = self.consume_lexeme_until(|c| *c != char);
        self.consume_char(char);
        lexeme
    }

    fn match_comment(&mut self) -> Option<Token> {
        self.consume_any_char();

        let lexeme = self.consume_lexeme_until(|char| *char != '\n').unwrap();

        Some(Token {
            lexeme,
            kind: TokenKind::Comment,
        })
    }

    fn match_identifier(&mut self) -> Option<Token> {
        let lexeme = self
            .consume_lexeme_until(|char| char.is_alphanumeric() || *char == '_')
            .unwrap();

        Some(Token {
            lexeme,
            kind: match self.match_char('!') {
                Some(_) => TokenKind::Command,
                None => TokenKind::Identifier,
            },
        })
    }

    fn match_string(&mut self) -> Option<Token> {
        let lexeme = self
            .consume_lexeme_delimited('"')
            .expect("Cannot parse raw string");

        Some(Token {
            lexeme,
            kind: TokenKind::String,
        })
    }

    fn match_raw_string(&mut self) -> Option<Token> {
        let lexeme = self
            .consume_lexeme_delimited('`')
            .expect("Cannot parse raw string");

        Some(Token {
            lexeme,
            kind: TokenKind::RawString,
        })
    }

    fn match_number(&mut self) -> Option<Token> {
        let lexeme = self
            .consume_lexeme_until(|char| char.is_alphanumeric())
            .unwrap();

        Some(Token {
            lexeme,
            kind: TokenKind::Number,
        })
    }

    fn match_option(&mut self) -> Option<Token> {
        let lexeme = self
            .consume_lexeme_until(|char| char.is_alphanumeric() || *char == '-')
            .expect("Cannot parse option lexeme");

        Some(Token {
            lexeme,
            kind: TokenKind::Option,
        })
    }
}