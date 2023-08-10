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
            match self.tokens.get(self.index).unwrap().kind {
                TokenKind::Comment => self.skip_token(),
                TokenKind::Identifier | TokenKind::Command => {
                    expressions.push(Expression::FnChain(self.match_fn_chain()))
                }
                _ => todo!(
                    "Expression from {:#?} not implemented",
                    self.tokens.get(self.index).unwrap()
                ),
            };
        }

        expressions
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

    fn match_fn_chain(&mut self) -> FnChain {
        let mut invocations = Vec::new();

        loop {
            invocations.push(self.match_fn_call());
            if let None = self.consume_token(TokenKind::Period) {
                break;
            }
        }

        FnChain { invocations }
    }

    fn match_fn_call(&mut self) -> Expression {
        let name = match self.consume_token(TokenKind::Identifier) {
            Some(name) => name,
            None => self
                .consume_token(TokenKind::Command)
                .expect("Missing identifier in function call"),
        };

        self.consume_token(TokenKind::LeftBracket)
            .expect("Missing ( after identifier ");

        let mut args = Vec::new();

        loop {
            match self.match_fn_argument() {
                Some(arg) => {
                    args.push(arg);
                    if let None = self.consume_token(TokenKind::Coma) {
                        break;
                    }
                }
                None => break,
            }
        }

        self.consume_token(TokenKind::RightBracket)
            .expect("Missing ) after parameters list");

        match name.kind {
            TokenKind::Identifier => Expression::FnCall(FnCall { name, args }),
            TokenKind::Command => Expression::CmdCall(CmdCall { name, args }),
            _ => panic!("Invalid match_fn_call invocation with {:?}", name),
        }
    }

    fn match_fn_argument(&mut self) -> Option<Token> {
        self.consume_token_of_multiple_kinds(&[
            TokenKind::String,
            TokenKind::RawString,
            TokenKind::Number,
            TokenKind::Option,
        ])
    }
}
