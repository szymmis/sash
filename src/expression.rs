use std::fmt::Write;

use crate::{
    formatter,
    token::{Kind, Token},
};

#[derive(Debug, Clone)]
pub enum Expression {
    Value(ValueExpr),
    Arithmetic(ArithmeticExpr),
    Parenthesis(ParenthesisExpr),
    Condition(ConditionExpr),
    FnCall(FnCall),
    FnChain(FnChain),
    VarAssignment(VarAssignmentExpr),
    VarDeclaration(VarDeclarationExpr),
    IfStatement(IfStatementExpr),
    ElifStatement(ElifStatementExpr),
    ElseStatement(ElseStatementExpr),
    WhileStatement(WhileStatementExpr),
}

impl Expression {
    pub fn write(&self) -> String {
        match self {
            Self::Value(expr) => expr.value.write(),
            Self::Arithmetic(expr) => expr.write(),
            Self::Parenthesis(expr) => expr.write(),
            Self::Condition(expr) => expr.write(),
            Self::FnCall(fn_call) => fn_call.write(),
            Self::FnChain(fn_chain) => fn_chain.write(),
            Self::VarAssignment(expr) => expr.write(),
            Self::VarDeclaration(expr) => expr.write(),
            Self::IfStatement(expr) => expr.write(),
            Self::ElifStatement(expr) => expr.write(),
            Self::ElseStatement(expr) => expr.write(),
            Self::WhileStatement(expr) => expr.write(),
        }
    }
}

trait Expr {
    fn write(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct FnCall {
    pub name: Token,
    pub args: Vec<Expression>,
    pub command: bool,
}

impl Expr for FnCall {
    fn write(&self) -> String {
        let args_string = formatter::get_args_as_string(&self.args);

        if self.command {
            format!("{} {}", self.name.write(), args_string)
        } else {
            match self.name.lexeme.as_str() {
                "print" => format!("echo -e {args_string}"),
                "compress" => format!("tar -caf {args_string}"),
                "decompress" => format!("tar -xf {args_string}"),
                "ls_archive" => format!("tar -tvf {args_string}"),
                "red" | "green" | "yellow" | "blue" | "magenta" | "cyan" => {
                    match self.args.get(0).unwrap() {
                        Expression::Value(ValueExpr { value }) => match value.kind {
                            Kind::String => {
                                formatter::colorize_string(&self.name.lexeme, &value.lexeme)
                            }
                            _ => formatter::colorize_string(&self.name.lexeme, &value.write()),
                        },
                        _ => {
                            panic!("Color functions can only take value expression as an argument")
                        }
                    }
                }
                _ => panic!("Build-in function \"{}\" not supported!", self.name.lexeme),
            }
        }
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

#[derive(Debug, Clone)]
pub struct ValueExpr {
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
        format!(
            "{}={}",
            self.name.lexeme,
            match *self.value.clone() {
                Expression::FnCall(fn_call) => format!("\"$({})\"", fn_call.write()),
                _ => self.value.write(),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct VarDeclarationExpr {
    pub name: Token,
    pub value: Box<Expression>,
}

impl Expr for VarDeclarationExpr {
    fn write(&self) -> String {
        format!(
            "{}={}",
            self.name.lexeme,
            match *self.value.clone() {
                Expression::FnCall(fn_call) => format!("\"$({})\"", fn_call.write()),
                _ => self.value.write(),
            }
        )
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

#[derive(Debug, Clone)]
pub struct ConditionExpr {
    pub lhs: Box<Expression>,
    pub operator: Token,
    pub rhs: Box<Expression>,
}

impl Expr for ConditionExpr {
    fn write(&self) -> String {
        format!(
            "[ {} {} {} ]",
            self.lhs.write(),
            self.operator.write(),
            self.rhs.write()
        )
    }
}

#[derive(Debug, Clone)]
pub struct IfStatementExpr {
    pub condition: Box<Expression>,
    pub body: Vec<Expression>,
    pub branching: Option<Box<Expression>>,
}

impl Expr for IfStatementExpr {
    fn write(&self) -> String {
        if self.body.is_empty() {
            String::new()
        } else {
            format!(
                "if {}; then\n{}{}",
                self.condition.write(),
                formatter::write_formatted_expressions(&self.body),
                match &self.branching {
                    Some(branching) => branching.write(),
                    None => "fi".into(),
                }
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct ElifStatementExpr {
    pub condition: Box<Expression>,
    pub body: Vec<Expression>,
    pub branching: Option<Box<Expression>>,
}

impl Expr for ElifStatementExpr {
    fn write(&self) -> String {
        if self.body.is_empty() {
            String::new()
        } else {
            format!(
                "elif {}; then\n{}{}",
                self.condition.write(),
                formatter::write_formatted_expressions(&self.body),
                match &self.branching {
                    Some(branching) => branching.write(),
                    None => "fi".into(),
                }
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct ElseStatementExpr {
    pub body: Vec<Expression>,
}

impl Expr for ElseStatementExpr {
    fn write(&self) -> String {
        if self.body.is_empty() {
            String::new()
        } else {
            format!(
                "else\n{}fi",
                formatter::write_formatted_expressions(&self.body)
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct WhileStatementExpr {
    pub condition: Box<Expression>,
    pub body: Vec<Expression>,
}

impl Expr for WhileStatementExpr {
    fn write(&self) -> String {
        if self.body.is_empty() {
            String::new()
        } else {
            format!(
                "while {}\ndo\n{}done",
                self.condition.write(),
                formatter::write_formatted_expressions(&self.body),
            )
        }
    }
}
