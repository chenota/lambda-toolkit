use crate::types::ast::*;

type Environment = Vec<(String, Expression)>;

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