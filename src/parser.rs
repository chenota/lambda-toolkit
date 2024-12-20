use crate::types::{ast::Expression, token::{Token, Variant}};

struct Parser {
    pos: usize,
    tokens: Vec<Token>
}
impl Parser {
    // Utility Functions
    pub fn new() -> Parser {
        Parser {
            pos: 0,
            tokens: Vec::new()
        }
    }
    fn mark(&self) -> usize {
        self.pos
    }
    fn reset(&mut self, pos: usize) -> () {
        self.pos = pos
    }
    fn get_token(&mut self) -> Token {
        let token = self.peek_token();
        self.pos += 1;
        token
    }
    fn peek_token(&self) -> Token {
        self.tokens.get(self.pos).unwrap().clone()
    }
    fn expect(&mut self, arg: Variant) -> Option<Token> {
        let token = self.peek_token();
        if token.0 == arg {
            Some(self.get_token())
        } else {
            None
        }
    }
    // Parsing rules
    fn program(&mut self) -> Program {
        
    }
    fn stmtlist(&mut self) -> Vec<Statement> {
        
    }
}