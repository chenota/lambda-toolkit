use crate::types::ast::*;

type Environment = Vec<(String, Expression)>;

macro_rules! vtype {
    ($e:expr) => {
        match $e {
            Value::Number(_) => "int",
            Value::Identifier(_) => "keyword",
            Value::Boolean(_) => "bool",
            Value::Unit => "unit"
        }
    }
}

macro_rules! value {
    ($e:expr,$p:path,$l:literal) => {
        match $e {
            Expression::ValExpr(v) => match v {
                $p(x) => x.clone(),
                _ => return Err("Incompatible type ".to_string() + vtype!(v) + concat!(" with operation ", $l))
            },
            _ => return Err("Incompatible type expr".to_string() + concat!(" with operation ", $l))
        }
    }
}

pub struct Evaluator {
    env: Environment,
}
impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator{  
            env: Vec::new()
        }
    }
    fn step(&mut self, expr: &mut Expression) -> Result<bool, String> {
        match expr {
            Expression::ValExpr(_) => Ok(false),
            Expression::UopExpr(op, e1) => {
                // Attempt to step e1
                let e1_step = self.step(e1.as_mut())?;
                // If was able to step e1, return true
                if e1_step { return Ok(true) };
                // If e1 fully reduced, perform unary operation
                match op {
                    Uop::NotUop => {
                        // Get boolean value from e1
                        let bool_val = value!(e1.as_ref(), Value::Boolean, "not");
                        // Update expr
                        *expr = Expression::ValExpr(Value::Boolean(!bool_val));
                        // Return true
                        Ok(true)
                    },
                    Uop::NegUop => {
                        // Get integer value from e1
                        let int_val = value!(e1.as_ref(), Value::Number, "negation");
                        // Update expr
                        *expr = Expression::ValExpr(Value::Number(-int_val));
                        // Return true
                        Ok(true)
                    }
                }
            }
            _ => Err("Unimplemented".to_string())
        }
    }
    fn eval_expr(&mut self, mut expr: Expression) -> Result<Expression, String> {
        // Reduce expression until fixed point
        loop { if !(self.step(&mut expr)?) { break } };
        // Return reduced expression
        Ok(expr)
    }
    pub fn eval_program(&mut self, prog: Program) -> Result<Expression, String> {
        // Statements
        // Program body
        self.eval_expr(prog.1)
    }
}