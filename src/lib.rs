#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]
pub mod expression;
pub mod formatter;
pub mod lexer;
pub mod parser;
pub mod token;

use std::{
    fmt::Write,
    fs::{read_to_string, File},
    os::unix::process::CommandExt,
    process::Command,
};

use crate::{expression::Expression, lexer::Lexer, parser::Parser, token::Token};

pub struct Script {
    tokens: Vec<Token>,
    expressions: Vec<Expression>,
}

impl Script {
    pub fn from_file(path: &str) -> Self {
        let input = read_to_string(path).expect("Cannot open source file");
        Script::from_string(&input)
    }

    pub fn from_string(input: &str) -> Self {
        let mut lexer = Lexer::from_string(input);
        let tokens = lexer.parse();

        dbg!(&tokens);

        let mut parser = Parser::from_tokens(tokens.clone());
        let expressions = parser.parse();

        dbg!(&expressions);

        Script {
            tokens,
            expressions,
        }
    }

    pub fn debug(&self) {
        println!("{:#?} {:#?}", self.tokens, self.expressions);
    }

    pub fn get_code(&self) -> String {
        let mut output = String::new();

        for expression in &self.expressions {
            output.write_str(expression.write().as_str()).unwrap();
            output.write_char('\n').unwrap();
        }

        output
    }

    pub fn write_file(&self, path: &str) {
        let mut file = File::options().create(true).write(true).open(path).unwrap();
        std::io::Write::write_fmt(&mut file, format_args!("{}", self.get_code())).unwrap();
    }

    pub fn interpret(&self) {
        Command::new("bash").args(["-c", &self.get_code()]).exec();
    }
}
