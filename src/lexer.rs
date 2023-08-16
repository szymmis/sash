use crate::token::{Kind, Token};

pub struct Lexer {
    index: usize,
    chars: Vec<char>,
    line_counter: usize,
    column_counter: usize,
}

#[allow(clippy::unnecessary_wraps)]
impl Lexer {
    pub fn from_string(input: &str) -> Self {
        Lexer {
            index: 0,
            line_counter: 1,
            column_counter: 1,
            chars: input.chars().collect(),
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.index < self.chars.len() {
            match self.seek_token() {
                Some(token) => tokens.push(token),
                None => self.skip_char(),
            }
        }

        tokens
    }

    fn seek_token(&mut self) -> Option<Token> {
        match self.get_char().unwrap() {
            '#' => self.match_comment(),
            '"' => self.match_string(),
            '`' => self.match_raw_string(),
            '-' => self.match_option(),
            '0'..='9' => self.match_number(),
            'a'..='z' | 'A'..='Z' => self.match_identifier(),
            ' ' | '\t' => self.match_whitespace(),
            '\n' => self.match_new_line(),
            _ => self.match_char_token(),
        }
    }

    fn get_char(&self) -> Option<&char> {
        self.chars.get(self.index)
    }

    fn skip_char(&mut self) {
        self.index += 1;
        self.column_counter += 1;
    }

    fn match_char(&self, char: char) -> Option<&char> {
        if *self.get_char()? == char {
            self.get_char()
        } else {
            None
        }
    }

    fn consume_any_char(&mut self) -> Option<char> {
        let char = self.get_char().copied();
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
        let start = self.index;

        while let Some(char) = self.get_char() {
            if f(char) {
                self.consume_any_char();
            } else {
                break;
            }
        }

        Some(self.chars[start..self.index].iter().collect())
    }

    fn consume_lexeme_delimited(&mut self, char: char) -> Option<String> {
        self.consume_char(char);
        let lexeme = self.consume_lexeme_until(|c| *c != char);
        self.consume_char(char);
        lexeme
    }

    fn match_char_token(&mut self) -> Option<Token> {
        match self.consume_any_char()? {
            '=' => {
                if self.consume_char('=').is_some() {
                    Some(Token {
                        lexeme: "==".into(),
                        kind: Kind::EqualEqual,
                    })
                } else {
                    Some(Token {
                        lexeme: '='.into(),
                        kind: Kind::Equal,
                    })
                }
            }
            '<' => {
                if self.consume_char('=').is_some() {
                    Some(Token {
                        lexeme: "<=".into(),
                        kind: Kind::LessEqual,
                    })
                } else {
                    Some(Token {
                        lexeme: '<'.into(),
                        kind: Kind::Less,
                    })
                }
            }
            '>' => {
                if self.consume_char('=').is_some() {
                    Some(Token {
                        lexeme: ">=".into(),
                        kind: Kind::GreaterEqual,
                    })
                } else {
                    Some(Token {
                        lexeme: '>'.into(),
                        kind: Kind::Greater,
                    })
                }
            }
            '!' => {
                if self.consume_char('=').is_some() {
                    Some(Token {
                        lexeme: "!=".into(),
                        kind: Kind::NotEqual,
                    })
                } else {
                    Some(Token {
                        lexeme: '!'.into(),
                        kind: Kind::Not,
                    })
                }
            }
            char => match Token::from_char(char) {
                Some(token) => Some(token),
                None => panic!(
                    "Syntax error: Unknown character '{}' at column {}, line {}",
                    char, self.column_counter, self.line_counter
                ),
            },
        }
    }

    fn match_new_line(&mut self) -> Option<Token> {
        self.line_counter += 1;
        self.column_counter = 0;

        None
    }

    #[allow(clippy::unused_self)]
    fn match_whitespace(&self) -> Option<Token> {
        None
    }

    fn match_comment(&mut self) -> Option<Token> {
        self.consume_any_char();

        let lexeme = self.consume_lexeme_until(|char| *char != '\n').unwrap();

        Some(Token {
            lexeme,
            kind: Kind::Comment,
        })
    }

    fn match_identifier(&mut self) -> Option<Token> {
        let lexeme = self
            .consume_lexeme_until(|char| char.is_alphanumeric() || *char == '_')
            .unwrap();

        Token::from_keyword(&lexeme).or(Some(Token {
            lexeme,
            kind: match self.consume_char('!') {
                Some(_) => Kind::Command,
                None => Kind::Identifier,
            },
        }))
    }

    fn match_string(&mut self) -> Option<Token> {
        let lexeme = self
            .consume_lexeme_delimited('"')
            .expect("Cannot parse raw string");

        Some(Token {
            lexeme,
            kind: Kind::String,
        })
    }

    fn match_raw_string(&mut self) -> Option<Token> {
        let lexeme = self
            .consume_lexeme_delimited('`')
            .expect("Cannot parse raw string");

        Some(Token {
            lexeme,
            kind: Kind::RawString,
        })
    }

    fn match_number(&mut self) -> Option<Token> {
        let lexeme = self
            .consume_lexeme_until(|char| char.is_alphanumeric())
            .unwrap();

        Some(Token {
            lexeme,
            kind: Kind::Number,
        })
    }

    fn match_option(&mut self) -> Option<Token> {
        let lexeme = self
            .consume_lexeme_until(|char| char.is_alphanumeric() || *char == '-')
            .expect("Cannot parse option lexeme");

        if lexeme == "-" {
            Some(Token {
                lexeme: '-'.into(),
                kind: Kind::Minus,
            })
        } else {
            Some(Token {
                lexeme,
                kind: Kind::Option,
            })
        }
    }
}
