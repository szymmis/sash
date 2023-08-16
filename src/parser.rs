#[allow(clippy::wildcard_imports)]
use crate::{
    expression::*,
    token::{Kind, Token},
};

pub struct Parser {
    index: usize,
    tokens: Vec<Token>,
}

#[allow(clippy::unnecessary_wraps)]
impl Parser {
    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        Parser { index: 0, tokens }
    }

    pub fn parse(&mut self) -> Vec<Expression> {
        let mut expressions = Vec::new();

        while self.index < self.tokens.len() {
            match self.seek_expression() {
                Some(expression) => expressions.push(expression),
                None => self.skip_token(),
            }
        }

        expressions
    }

    fn seek_expression(&mut self) -> Option<Expression> {
        match self.tokens.get(self.index).unwrap().kind {
            Kind::Identifier => self.match_identifier(),
            Kind::Command => self.match_fn_chain(),
            Kind::Let => self.match_var_declaration(),
            Kind::If => self.match_if_statement(),
            Kind::Else => self.match_else_if_statement(),
            Kind::While => self.match_while_statement(),
            Kind::Comment => None,
            _ => {
                println!(
                    "Unknown expression: {:#?}",
                    self.tokens.get(self.index).unwrap()
                );
                None
            }
        }
    }

    fn get_token(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn skip_token(&mut self) {
        self.index += 1;
    }

    fn match_token(&self, kind: Kind) -> Option<&Token> {
        if self.get_token()?.kind == kind {
            self.get_token()
        } else {
            None
        }
    }

    fn peek_token(&self, kind: Kind) -> Option<&Token> {
        let token = self.tokens.get(self.index + 1)?;
        if token.kind == kind {
            Some(token)
        } else {
            None
        }
    }

    fn consume_token(&mut self, kind: Kind) -> Option<Token> {
        match self.match_token(kind) {
            Some(token) => {
                let token = token.clone();
                self.index += 1;
                Some(token)
            }
            None => None,
        }
    }

    fn consume_token_of_multiple_kinds(&mut self, valid_kinds: &[Kind]) -> Option<Token> {
        for kind in valid_kinds {
            if let Some(token) = self.consume_token(*kind) {
                return Some(token);
            }
        }

        None
    }

    fn match_value_expr(&mut self) -> Option<Expression> {
        let value = self.consume_token_of_multiple_kinds(&[Kind::Number, Kind::Identifier])?;
        Some(Expression::Token(TokenExpr { value }))
    }

    fn match_arithmetic_expr(&mut self) -> Option<Expression> {
        let lhs = self.match_value_expr().or_else(|| {
            self.consume_token(Kind::LeftParen);

            match self.match_arithmetic_expr().unwrap() {
                Expression::Token(value) => Some(Expression::Token(value)),
                value => {
                    self.consume_token(Kind::RightParen).unwrap();
                    Some(Expression::Parenthesis(ParenthesisExpr {
                        value: Box::new(value),
                    }))
                }
            }
        })?;

        let operator = self.consume_token_of_multiple_kinds(&[
            Kind::Plus,
            Kind::Minus,
            Kind::Asterisk,
            Kind::Slash,
        ]);

        if operator.is_none() {
            return Some(lhs);
        }

        let operator = operator.unwrap();

        match self.match_arithmetic_expr() {
            Some(rhs) => Some(Expression::Arithmetic(ArithmeticExpr {
                lhs: Box::new(lhs),
                operator,
                rhs: Box::new(rhs),
            })),
            None => panic!(
                "Syntax error: Expected value after operator {}",
                operator.write()
            ),
        }
    }

    fn match_conditional_expr(&mut self) -> Option<Expression> {
        let lhs = self.match_arithmetic_expr().unwrap();

        let operator = self
            .consume_token_of_multiple_kinds(&[
                Kind::Less,
                Kind::LessEqual,
                Kind::Greater,
                Kind::GreaterEqual,
                Kind::EqualEqual,
            ])
            .expect("Expected operator");

        let rhs = self.match_arithmetic_expr().unwrap();

        Some(Expression::Condition(ConditionExpr {
            lhs: Box::new(lhs),
            operator,
            rhs: Box::new(rhs),
        }))
    }

    fn match_if_statement(&mut self) -> Option<Expression> {
        self.consume_token(Kind::If)?;

        self.consume_token(Kind::LeftParen)
            .expect("Expected ( after if keyword");

        let condition = self.match_conditional_expr().unwrap();

        self.consume_token(Kind::RightParen)
            .expect("Expected ( after if keyword");

        self.consume_token(Kind::LeftBracket)
            .expect("Expected { after if (...) keyword");

        let mut body = Vec::new();

        loop {
            match self.get_token().unwrap().kind {
                Kind::RightBracket => break,
                _ => body.push(self.seek_expression().unwrap()),
            }
        }

        self.consume_token(Kind::RightBracket).expect("Expected }");

        let branching = self.match_else_if_statement().map(Box::new);

        Some(Expression::IfStatement(IfStatementExpr {
            condition: Box::new(condition),
            body,
            branching,
        }))
    }

    fn match_else_if_statement(&mut self) -> Option<Expression> {
        self.consume_token(Kind::Else)?;

        if self.match_token(Kind::If).is_some() {
            match self.match_if_statement().unwrap() {
                Expression::IfStatement(IfStatementExpr {
                    condition,
                    body,
                    branching,
                }) => Some(Expression::ElifStatement(ElifStatementExpr {
                    condition,
                    body,
                    branching,
                })),
                _ => todo!(),
            }
        } else {
            self.consume_token(Kind::LeftBracket)
                .expect("Expected { after else keyword");

            let mut body = Vec::new();

            loop {
                match self.get_token().unwrap().kind {
                    Kind::RightBracket => break,
                    _ => body.push(self.seek_expression().unwrap()),
                }
            }

            Some(Expression::ElseStatement(ElseStatementExpr { body }))
        }
    }

    fn match_while_statement(&mut self) -> Option<Expression> {
        self.consume_token(Kind::While)?;

        self.consume_token(Kind::LeftParen)
            .expect("Expected ( after for keyword");

        let condition = self.match_conditional_expr().unwrap();

        self.consume_token(Kind::RightParen).expect("Expected )");

        self.consume_token(Kind::LeftBracket)
            .expect("Expected { after while (...) keyword");

        let mut body = Vec::new();

        loop {
            match self.get_token().unwrap().kind {
                Kind::RightBracket => break,
                _ => body.push(self.seek_expression().unwrap()),
            }
        }

        self.consume_token(Kind::RightBracket).expect("Expected }");

        Some(Expression::WhileStatement(WhileStatementExpr {
            condition: Box::new(condition),
            body,
        }))
    }

    fn match_var_assignment(&mut self) -> Option<Expression> {
        let name = self.consume_token(Kind::Identifier)?;
        self.consume_token(Kind::Equal);

        let value = self
            .match_arithmetic_expr()
            .expect("Expected expression after let _ =");

        Some(Expression::VarAssignment(VarAssignmentExpr {
            name,
            value: Box::new(value),
        }))
    }

    fn match_var_declaration(&mut self) -> Option<Expression> {
        self.consume_token(Kind::Let);
        let assignment = self.match_var_assignment().unwrap();

        match assignment {
            Expression::VarAssignment(VarAssignmentExpr { name, value }) => {
                Some(Expression::VarDeclaration(VarDeclarationExpr {
                    name,
                    value,
                }))
            }
            _ => panic!(),
        }
    }

    fn match_identifier(&mut self) -> Option<Expression> {
        if self.peek_token(Kind::Equal).is_some() {
            self.match_var_assignment()
        } else {
            self.match_fn_chain()
        }
    }

    fn match_fn_chain(&mut self) -> Option<Expression> {
        let mut invocations = Vec::new();

        loop {
            invocations.push(self.match_fn_call());
            if self.consume_token(Kind::Period).is_none() {
                break;
            }
        }

        Some(Expression::FnChain(FnChain { invocations }))
    }

    fn match_fn_call(&mut self) -> Expression {
        let name = self
            .consume_token_of_multiple_kinds(&[Kind::Identifier, Kind::Command])
            .expect("Missing function identifier");

        self.consume_token(Kind::LeftParen)
            .expect("Missing ( after identifier ");

        let args = self.match_fn_arguments();

        self.consume_token(Kind::RightParen)
            .expect("Missing ) after parameters list");

        match name.kind {
            Kind::Identifier => Expression::FnCall(FnCall { name, args }),
            Kind::Command => Expression::CmdCall(CmdCall { name, args }),
            _ => panic!("Invalid match_fn_call invocation with {:?}", name),
        }
    }

    fn match_fn_arguments(&mut self) -> Vec<Expression> {
        let mut args = Vec::new();

        loop {
            if let Some(arg) =
                self.consume_token_of_multiple_kinds(&[Kind::String, Kind::RawString, Kind::Option])
            {
                args.push(Expression::Token(TokenExpr { value: arg }));
            } else {
                match self.match_arithmetic_expr() {
                    Some(expr) => args.push(expr),
                    None => break,
                }
            }

            if self.consume_token(Kind::Coma).is_none() {
                break;
            }
        }

        args
    }
}
