use crate::{
    expression::{CmdCall, Expression, FnCall, FnChain},
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
            TokenKind::Identifier | TokenKind::Command => self.match_fn_chain(),
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

    fn match_fn_arguments(&mut self) -> Vec<Token> {
        let mut args = Vec::new();

        loop {
            if let Some(arg) = self.consume_token_of_multiple_kinds(&[
                TokenKind::String,
                TokenKind::RawString,
                TokenKind::Number,
                TokenKind::Option,
            ]) {
                args.push(arg);
                if let None = self.consume_token(TokenKind::Coma) {
                    break;
                }
            } else {
                break;
            }
        }

        args
    }
}
