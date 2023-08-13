use crate::{
    expression::*,
    token::{Token, TokenKind},
};

pub struct Parser {
    index: usize,
    tokens: Vec<Token>,
}

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
            TokenKind::Identifier => self.match_identifier(),
            TokenKind::Command => self.match_fn_chain(),
            TokenKind::Let => self.match_var_declaration(),
            TokenKind::Comment => None,
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

    fn match_token(&self, kind: TokenKind) -> Option<&Token> {
        if self.get_token()?.kind == kind {
            self.get_token()
        } else {
            None
        }
    }

    fn peek_token(&self, kind: TokenKind) -> Option<&Token> {
        let token = self.tokens.get(self.index + 1)?;
        if token.kind == kind {
            Some(token)
        } else {
            None
        }
    }

    fn consume_token(&mut self, kind: TokenKind) -> Option<Token> {
        match self.match_token(kind) {
            Some(token) => {
                let token = token.clone();
                self.index += 1;
                Some(token)
            }
            None => None,
        }
    }

    fn consume_token_of_multiple_kinds(&mut self, valid_kinds: &[TokenKind]) -> Option<Token> {
        for kind in valid_kinds {
            if let Some(token) = self.consume_token(kind.to_owned()) {
                return Some(token);
            }
        }

        None
    }

    fn match_value_expr(&mut self) -> Option<Expression> {
        let value =
            self.consume_token_of_multiple_kinds(&[TokenKind::Number, TokenKind::Identifier])?;
        Some(Expression::Token(TokenExpr { value }))
    }

    fn match_arithmetic_expr(&mut self) -> Option<Expression> {
        let lhs = self.match_value_expr().or_else(|| {
            self.consume_token(TokenKind::LeftBracket);

            match self.match_arithmetic_expr().unwrap() {
                Expression::Token(value) => Some(Expression::Token(value)),
                value => {
                    self.consume_token(TokenKind::RightBracket).unwrap();
                    Some(Expression::Parenthesis(ParenthesisExpr {
                        value: Box::new(value),
                    }))
                }
            }
        })?;

        let operator = self.consume_token_of_multiple_kinds(&[
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Asterisk,
            TokenKind::Slash,
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

    fn match_var_assignment(&mut self) -> Option<Expression> {
        let name = self.consume_token(TokenKind::Identifier)?;
        self.consume_token(TokenKind::Equal);

        let value = self
            .match_arithmetic_expr()
            .expect("Expected expression after let _ =");

        Some(Expression::VarAssignment(VarAssignmentExpr {
            name,
            value: Box::new(value),
        }))
    }

    fn match_var_declaration(&mut self) -> Option<Expression> {
        self.consume_token(TokenKind::Let);
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
        if self.peek_token(TokenKind::Equal).is_some() {
            self.match_var_assignment()
        } else {
            self.match_fn_chain()
        }
    }

    fn match_fn_chain(&mut self) -> Option<Expression> {
        let mut invocations = Vec::new();

        loop {
            invocations.push(self.match_fn_call());
            if let None = self.consume_token(TokenKind::Period) {
                break;
            }
        }

        Some(Expression::FnChain(FnChain { invocations }))
    }

    fn match_fn_call(&mut self) -> Expression {
        let name = self
            .consume_token_of_multiple_kinds(&[TokenKind::Identifier, TokenKind::Command])
            .expect("Missing function identifier");

        self.consume_token(TokenKind::LeftBracket)
            .expect("Missing ( after identifier ");

        let args = self.match_fn_arguments();

        self.consume_token(TokenKind::RightBracket)
            .expect("Missing ) after parameters list");

        match name.kind {
            TokenKind::Identifier => Expression::FnCall(FnCall { name, args }),
            TokenKind::Command => Expression::CmdCall(CmdCall { name, args }),
            _ => panic!("Invalid match_fn_call invocation with {:?}", name),
        }
    }

    fn match_fn_arguments(&mut self) -> Vec<Box<Expression>> {
        let mut args = Vec::new();

        loop {
            if let Some(arg) = self.consume_token_of_multiple_kinds(&[
                TokenKind::String,
                TokenKind::RawString,
                TokenKind::Option,
            ]) {
                args.push(Box::new(Expression::Token(TokenExpr { value: arg })));
            } else {
                match self.match_arithmetic_expr() {
                    Some(expr) => args.push(Box::new(expr)),
                    None => break,
                }
            }

            if let None = self.consume_token(TokenKind::Coma) {
                break;
            }
        }

        args
    }
}
