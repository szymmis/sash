use std::fmt::Write;

use crate::token::Token;

#[derive(Debug)]
pub enum Expression {
    FnCall(FnCall),
    CmdCall(CmdCall),
    FnChain(FnChain),
}

impl Expression {
    pub fn write(&self) -> String {
        match self {
            Expression::FnCall(fn_call) => fn_call.write(),
            Expression::CmdCall(cmd_call) => cmd_call.write(),
            Expression::FnChain(fn_chain) => fn_chain.write(),
        }
    }
}

#[derive(Debug)]
pub struct FnCall {
    pub name: Token,
    pub args: Vec<Token>,
}

impl FnCall {
    fn write(&self) -> String {
        let args = get_args_as_string(&self.args);

        match self.name.lexeme.as_str() {
            "print" => format!("echo {}", args),
            "compress" => format!("tar -caf {}", args),
            "decompress" => format!("tar -xf {}", args),
            "ls_archive" => format!("tar -tvf {}", args),
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct CmdCall {
    pub name: Token,
    pub args: Vec<Token>,
}

impl CmdCall {
    fn write(&self) -> String {
        format!("{} {}", self.name.lexeme, get_args_as_string(&self.args))
    }
}

#[derive(Debug)]
pub struct FnChain {
    pub invocations: Vec<Expression>,
}

impl FnChain {
    fn write(&self) -> String {
        let mut output = String::new();

        let mut iter = self.invocations.iter().peekable();
        while let Some(invocation) = iter.next() {
            output.write_str(&invocation.write()).unwrap();
            if iter.peek().is_some() {
                output.write_str(" | ").unwrap();
            }
        }

        output
    }
}

fn get_args_as_string(args: &Vec<Token>) -> String {
    let mut arguments_string = String::new();

    let mut iter = args.iter().peekable();

    while let Some(arg) = iter.next() {
        arguments_string
            .write_str(format!("{}", arg.write()).as_str())
            .unwrap();

        if iter.peek().is_some() {
            arguments_string.write_str(" ").unwrap();
        }
    }

    arguments_string
}
