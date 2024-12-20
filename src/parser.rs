use crate::types::{ast::{Expression, Statement}, token::{Token, TokenValue, Variant}};

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
    fn expect_err(&mut self, arg: Variant) -> Result<Token, String> {
        match self.expect(arg) {
            Some(t) => Ok(t),
            None => Err(self.err_msg())
        }
    }
    // Generate error message
    fn err_msg(&self) -> String {
        let t_pos = self.peek_token().2;
        "Syntax error at ".to_string() + t_pos.0.to_string().as_ref() + ":" + t_pos.1.to_string().as_ref()
    }
    // Parsing rules
    fn program(&mut self) -> Result<Program, String> {
        
    }
    fn stmtlist(&mut self) -> Result<Vec<Statement>, String> {

    }
    fn statement(&mut self) -> Result<Statement, String> {
        // Expect a let keyword
        self.expect_err(Variant::Let)?;
        // Expect an identifier
        let id = self.identifier()?;
    }
    fn expression(&mut self) -> Result<Expression, String> {
        
    }
    fn identifier(&mut self) -> Result<Option<String>, String> {
        // Check for ident token
        match self.expect(Variant::Ident) {
            Some((_, val, _)) => match val {
                TokenValue::Str(s) => Ok(Some(s)),
                _ => Err(self.err_msg())
            },
            // No ident, check for unit token
            _ => match self.expect(Variant::Unit) {
                Some(_) => Ok(None),
                _ => Err(self.err_msg())
            }
        }
    }
}