use crate::types::{ast::{Expression, Statement, Value}, token::{Token, TokenValue, Variant}};

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
    fn expect(&mut self, arg: Variant) -> Result<Token, String> {
        let token = self.peek_token();
        if token.0 == arg {
            Ok(self.get_token())
        } else {
            Err(self.err_msg())
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
        self.expect(Variant::Let)?;
        // Expect an identifier
        let id = self.identifier()?;
    }
    fn expression(&mut self) -> Result<Expression, String> {
        // Parse an e1
        let head = self.e1()?;
        // Vector of applications
        let mut app_vec = vec![ head ];
        // Loop and consume e1s until error
        loop {
            let pos = self.mark();
            match self.e1() {
                Ok(ex) => app_vec.push(ex),
                _ => {
                    self.reset(pos);
                    break
                }
            }
        };
        // If single item, just return item
        if app_vec.len() == 1 {
            Ok(app_vec.pop().unwrap())
        } else {
            Ok(Expression::ApplicationExpr(app_vec))
        }
    }
    fn e1(&mut self) -> Result<Expression, String> {
        // Expect a lambda character
        match self.expect(Variant::Lambda) {
            // Found backslash
            Ok(_) => {
                // Parse an identlist
                let ilist = self.identlist()?;
                // Expect a dot
                self.expect(Variant::Dot)?;
                // Parse an expression
                let body = self.expression()?;
                // Return
                Ok(Expression::FuncExpr(ilist, Box::new(body)))
            },
            // If error, parse an e1
            _ => self.e2()
        }
    }
    fn e2(&mut self) -> Result<Expression, String> {
        // Parse an e3
        let head = self.e3()?;
    }
    fn e3(&mut self) -> Result<Expression, String> {
        // Parse a value
        let head = self.value()?;
    }
    fn value(&mut self) -> Result<Value, String> {

    }
    fn identifier(&mut self) -> Result<Option<String>, String> {
        // Check for ident token
        match self.expect(Variant::Ident) {
            Ok((_, val, _)) => match val {
                TokenValue::Str(s) => Ok(Some(s)),
                _ => Err(self.err_msg())
            },
            // No ident, check for unit token
            _ => match self.expect(Variant::Unit) {
                Ok(_) => Ok(None),
                Err(msg) => Err(msg)
            }
        }
    }
    fn identlist(&mut self) -> Result<Vec<Option<String>>, String> {
        // Check for at least one identifier
        let head = self.identifier()?;
        // Vector of identifiers
        let mut id_vec = vec![ head ];
        // Loop and consume idents until error
        loop {
            let pos = self.mark();
            match self.identifier() {
                Ok(s) => id_vec.push(s),
                _ => {
                    self.reset(pos);
                    break
                }
            }
        };
        // Return vector
        Ok(id_vec)
    }
}