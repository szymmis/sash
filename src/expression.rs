use std::fmt::Write;

use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    Token(TokenExpr),
    Arithmetic(ArithmeticExpr),
    Parenthesis(ParenthesisExpr),
    FnCall(FnCall),
    CmdCall(CmdCall),
    FnChain(FnChain),
    VarAssignment(VarAssignmentExpr),
    VarDeclaration(VarDeclarationExpr),
}

impl Expression {
    pub fn write(&self) -> String {
        match self {
            Self::Token(expr) => format!("{}", expr.value.write()),
            Self::Arithmetic(expr) => expr.write(),
            Self::Parenthesis(expr) => expr.write(),
            Self::FnCall(fn_call) => fn_call.write(),
            Self::CmdCall(cmd_call) => cmd_call.write(),
            Self::FnChain(fn_chain) => fn_chain.write(),
            Self::VarAssignment(expr) => expr.write(),
            Self::VarDeclaration(expr) => expr.write(),
        }
    }
}

trait Expr {
    fn write(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct FnCall {
    pub name: Token,
    pub args: Vec<Box<Expression>>,
}

impl Expr for FnCall {
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

#[derive(Debug, Clone)]
pub struct CmdCall {
    pub name: Token,
    pub args: Vec<Box<Expression>>,
}

impl Expr for CmdCall {
    fn write(&self) -> String {
        format!("{} {}", self.name.lexeme, get_args_as_string(&self.args))
    }
}

#[derive(Debug, Clone)]
pub struct FnChain {
    pub invocations: Vec<Expression>,
}

impl Expr for FnChain {
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

fn get_args_as_string(args: &Vec<Box<Expression>>) -> String {
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

#[derive(Debug, Clone)]
pub struct TokenExpr {
    pub value: Token,
}

#[derive(Debug, Clone)]
pub struct ArithmeticExpr {
    pub lhs: Box<Expression>,
    pub operator: Token,
    pub rhs: Box<Expression>,
}

impl ArithmeticExpr {
    fn eval(&self) -> String {
        format!(
            "{} {} {}",
            self.lhs.write(),
            self.operator.write(),
            match *self.rhs.clone() {
                Expression::Arithmetic(rhs) => rhs.eval(),
                _ => self.rhs.write(),
            }
        )
    }
}

impl Expr for ArithmeticExpr {
    fn write(&self) -> String {
        format!("$(({}))", self.eval())
    }
}

#[derive(Debug, Clone)]
pub struct VarAssignmentExpr {
    pub name: Token,
    pub value: Box<Expression>,
}

impl Expr for VarAssignmentExpr {
    fn write(&self) -> String {
        format!("{}={}", self.name.lexeme, self.value.write())
    }
}

#[derive(Debug, Clone)]
pub struct VarDeclarationExpr {
    pub name: Token,
    pub value: Box<Expression>,
}

impl Expr for VarDeclarationExpr {
    fn write(&self) -> String {
        format!("{}={}", self.name.lexeme, self.value.write())
    }
}

#[derive(Debug, Clone)]
pub struct ParenthesisExpr {
    pub value: Box<Expression>,
}

impl Expr for ParenthesisExpr {
    fn write(&self) -> String {
        format!(
            "({})",
            match *self.value.clone() {
                Expression::Arithmetic(rhs) => rhs.eval(),
                _ => self.value.write(),
            }
        )
    }
}
