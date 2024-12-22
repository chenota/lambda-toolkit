use crate::types::{ast::{Bop, Expression, Program, Statement, Uop, Value, Ident}, token::{Token, TokenValue, Variant}};

// Macros
macro_rules! token_value {
    ($e:expr, $i:path) => {
        match $e.1 { $i(x) => x, _ => return Err("Token value error".to_string()) }
    }
}

// Constants
const LOGICAL_LOW: [(Variant, Bop); 1] = [
    (Variant::And, Bop::AndBop),
];

const LOGICAL_HIGH: [(Variant, Bop); 2] = [
    (Variant::Or, Bop::OrBop),
    (Variant::Xor, Bop::XorBop)
];

const COMPARISON_OPS: [(Variant, Bop); 5] = [
    (Variant::Gt, Bop::GtBop),
    (Variant::Gte, Bop::GteBop),
    (Variant::Lt, Bop::LtBop),
    (Variant::Lte, Bop::LteBop),
    (Variant::Eq, Bop::EqBop)
];

const ARITHMETIC_LOW: [(Variant, Bop); 2] = [
    (Variant::Plus, Bop::PlusBop),
    (Variant::Minus, Bop::MinusBop),
];

const ARITHMETIC_HIGH: [(Variant, Bop); 2] = [
    (Variant::Times, Bop::TimesBop),
    (Variant::Div, Bop::DivBop)
];

// Parser
pub struct Parser {
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
        if self.pos >= self.tokens.len() { return self.tokens.get(self.tokens.len() - 1).unwrap().clone() }
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
    // Parse
    pub fn parse_program(&mut self, tokens: Vec<Token>) -> Result<Program, String> {
        // Set up parse
        self.tokens = tokens;
        self.reset(0);
        // Parse
        self.program()
    }
    // Parsing rules
    fn program(&mut self) -> Result<Program, String> {
        // Statement list
        let slist = self.stmtlist()?;
        // Expression
        let e = self.expression()?;
        // Expect EOF
        self.expect(Variant::EOF)?;
        // Put together
        Ok((slist, e))
    }
    fn stmtlist(&mut self) -> Result<Vec<Statement>, String> {
        // Empty statement vector
        let mut stmt_vec = Vec::new();
        // Consume statements
        loop {
            // Mark position
            let pos = self.mark();
            // Consume statement
            match self.statement() {
                Ok(s) => {
                    stmt_vec.push(s)
                },
                Err(_) => {
                    self.reset(pos);
                    break
                }
            }
        };
        // Return list of statements
        Ok(stmt_vec)
    }
    fn statement(&mut self) -> Result<Statement, String> {
        // Expect a let keyword
        self.expect(Variant::Let)?;
        // Expect an identifier
        let id = self.identifier()?;
        // Expect an equal sign
        self.expect(Variant::Eq)?;
        // Parse an expression
        let e = self.expression()?;
        // Expect an in keyword
        self.expect(Variant::In)?;
        // Put together
        Ok((id, e))
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
            // If error, parse an e2
            _ => self.e2()
        }
    }
    fn parse_bops(&mut self, oplist: &[(Variant, Bop)], f: fn(&mut Self) -> Result<Expression, String>) -> Result<Expression, String> {
        // Parse
        let head = f(self)?;
        // List of expressions
        let mut expr_list= Vec::new();
        // Consume until can't
        loop {
            // Mark position
            let pos = self.mark();
            // Pop token variant
            let peek_var = self.get_token().0;
            // Parse expression after and symbol if exists, otherwise if no and symbol break
            match oplist.iter().position(|r| r.0 == peek_var) {
                Some(i) => {
                    // Push operator and following expression
                    expr_list.push((oplist[i].1.clone(), f(self)?));
                },
                None => {
                    self.reset(pos);
                    break
                }
            }
        };
        // If nothing, return first item
        if expr_list.len() <= 0 {
            Ok(head)
        }
        // If found exprs, glue together as left associative
        else {
            Ok(
                expr_list
                    // Use a drain since never using original list again
                    .drain(0..)
                    // Fold left to apply in leftmost manner
                    .fold(
                        head, 
                        |acc, val| Expression::BopExpr(val.0, Box::new(acc), Box::new(val.1))
                    )
            )
        }
    }
    fn e2(&mut self) -> Result<Expression, String> {
        // Parse logical operators
        self.parse_bops(&LOGICAL_LOW, Self::e3)
    }
    fn e3(&mut self) -> Result<Expression, String> {
        // Parse logical operators
        self.parse_bops(&LOGICAL_HIGH, Self::e4)
    }
    fn e4(&mut self) -> Result<Expression, String> {
        // Parse comparison operators
        self.parse_bops(&COMPARISON_OPS, Self::e5)
    }
    fn e5(&mut self) -> Result<Expression, String> {
        // Parse arithmetic operators
        self.parse_bops(&ARITHMETIC_LOW, Self::e6)
    }
    fn e6(&mut self) -> Result<Expression, String> {
        // Parse arithmetic operators
        self.parse_bops(&ARITHMETIC_HIGH, Self::e7)
    }
    fn e7(&mut self) -> Result<Expression, String> {
        // Mark position
        let pos = self.mark();
        // Check for uops, reset if didn't find
        Ok(match self.get_token().0 {
            Variant::Not => Expression::UopExpr(Uop::NotUop, Box::new(self.e5()?)),
            Variant::Minus => Expression::UopExpr(Uop::NegUop, Box::new(self.e5()?)),
            Variant::LParen => {
                // Parse expression
                let e = self.expression()?;
                // Exect rparen
                self.expect(Variant::RParen)?;
                // Return
                e
            }
            _ => {
                self.reset(pos);
                Expression::ValExpr(self.value()?)
            }
        })
    }
    fn value(&mut self) -> Result<Value, String> {
        // Head of token list
        let token_head = self.get_token();
        // Check head of token list
        Ok(match token_head.0 {
            Variant::Ident => Value::Identifier(token_value!(token_head, TokenValue::Str)),
            Variant::Boolean => Value::Boolean(token_value!(token_head, TokenValue::Boolean)),
            Variant::Number => Value::Number(token_value!(token_head, TokenValue::Number)),
            Variant::Unit => Value::Unit,
            _ => return Err(self.err_msg())
        })
    }
    fn identifier(&mut self) -> Result<Ident, String> {
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
    fn identlist(&mut self) -> Result<Vec<Ident>, String> {
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