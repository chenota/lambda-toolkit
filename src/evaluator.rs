use crate::types::{ast::*, eval::EnvBody};

macro_rules! vtype {
    ($e:expr) => {
        match $e {
            Value::Number(_) => "int",
            Value::Identifier(_) => "keyword",
            Value::Boolean(_) => "bool",
            Value::Closure(_,_,_) => "closure",
            Value::Unit => "unit"
        }
    }
}

macro_rules! value {
    ($e:expr,$p:path,$l:literal) => {
        match $e {
            Expression::ValExpr(v) => match v {
                $p(x) => x.clone(),
                _ => return Err("Incompatible type '".to_string() + vtype!(v) + concat!("' with operation '", $l, "'"))
            },
            _ => return Err(concat!("Incompatible type 'expr' with operation '", $l, "'").to_string())
        }
    }
}

macro_rules! bop {
    // Use if input types == output type
    ($e1: ident, $e2: ident, $e3: ident, $val: path, $op: tt, $s: literal) => {
        {
            // Get values from e1 and e2
            let v1 = value!($e1.as_ref(), $val, $s);
            let v2 = value!($e2.as_ref(), $val, $s);
            // Update expr
            *$e3 = Expression::ValExpr($val(v1 $op v2));
            // Return true
            Ok(true)
        }
    };
    // Use if input types != output type
    ($e1: ident, $e2: ident, $e3: ident, $val1: path, $val2: path, $op: tt, $s: literal) => {
        {
            // Get values from e1 and e2
            let v1 = value!($e1.as_ref(), $val1, $s);
            let v2 = value!($e2.as_ref(), $val1, $s);
            // Update expr
            *$e3 = Expression::ValExpr($val2(v1 $op v2));
            // Return true
            Ok(true)
        }
    }
}

pub struct Environment {
    data: EnvBody
}
impl Environment {
    pub fn new() -> Environment {
        Environment {
            data: Vec::new()
        }
    }
    pub fn load(data: EnvBody) -> Environment {
        Environment {
            data
        }
    }
    pub fn push(&mut self, ident: &String, item: Expression) {
        self.data.push((ident.clone(), item))
    }
    pub fn pop(&mut self, n: usize) {
        for _ in 0..n { self.data.pop(); }
    }
    pub fn get_body(&self) -> EnvBody {
        self.data.clone()
    }
    pub fn clear(&mut self) {
        self.data.clear()
    }
    pub fn read(&self, ident: &String) -> Option<Expression> {
        // Iterate backwards through vector, return first match
        match self.data.iter().rev().find(|r| r.0 == *ident) {
            Some((_,ex)) => Some(ex.clone()),
            None => None
        }
    }
}

pub struct Evaluator {
    env: Environment,
}
impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator{  
            env: Environment::new()
        }
    }
    fn step(&mut self, expr: &mut Expression) -> Result<bool, String> {
        match expr {
            Expression::ValExpr(v) => {
                match v {
                    // If ident val, check if available in environment
                    Value::Identifier(ident) => match self.env.read(ident) {
                        // Available
                        Some(ex) => {
                            // Update self
                            *expr = ex;
                            // Return true
                            Ok(true)
                        },
                        // Not available, return false
                        _ => Ok(false)
                    },
                    _ => Ok(false)
                }
            },
            Expression::UopExpr(op, e1) => {
                // Attempt to step e1
                let e1_step = self.step(e1.as_mut())?;
                // If was able to step e1, return true
                if e1_step { return Ok(true) };
                // If e1 fully reduced, perform unary operation
                match op {
                    Uop::NotUop => {
                        // Get boolean value from e1
                        let bool_val = value!(e1.as_ref(), Value::Boolean, "!");
                        // Update expr
                        *expr = Expression::ValExpr(Value::Boolean(!bool_val));
                        // Return true
                        Ok(true)
                    },
                    Uop::NegUop => {
                        // Get integer value from e1
                        let int_val = value!(e1.as_ref(), Value::Number, "unary -");
                        // Update expr
                        *expr = Expression::ValExpr(Value::Number(-int_val));
                        // Return true
                        Ok(true)
                    }
                }
            },
            Expression::BopExpr(op, e1, e2) => {
                // Attempt to step e1
                let e1_step = self.step(e1.as_mut())?;
                // If was able to step e1, return true
                if e1_step { return Ok(true) };
                // Attempt to step e2
                let e2_step = self.step(e2.as_mut())?;
                // If was able to step e2, return true
                if e2_step { return Ok(true) };
                // If both fully reduced, perform binary operation
                match op {
                    Bop::PlusBop => bop!(e1, e2, expr, Value::Number, +, "+"),
                    Bop::MinusBop => bop!(e1, e2, expr, Value::Number, -, "-"),
                    Bop::TimesBop => bop!(e1, e2, expr, Value::Number, *, "*"),
                    Bop::DivBop => bop!(e1, e2, expr, Value::Number, /, "/"),
                    Bop::AndBop => bop!(e1, e2, expr, Value::Boolean, &&, "&"),
                    Bop::OrBop => bop!(e1, e2, expr, Value::Boolean, ||, "|"),
                    Bop::XorBop => bop!(e1, e2, expr, Value::Boolean, ^, "^"),
                    Bop::GtBop => bop!(e1, e2, expr, Value::Number, Value::Boolean, >, ">"),
                    Bop::GteBop => bop!(e1, e2, expr, Value::Number, Value::Boolean, >=, ">="),
                    Bop::LtBop => bop!(e1, e2, expr, Value::Number, Value::Boolean, <, "<="),
                    Bop::LteBop => bop!(e1, e2, expr, Value::Number, Value::Boolean, <=, "<="),
                    Bop::EqBop => bop!(e1, e2, expr, Value::Number, Value::Boolean, ==, "=")
                }
            },
            Expression::FuncExpr(params, body) => {
                // Change to a closure value
                *expr = Expression::ValExpr(Value::Closure(params.to_owned(), body.to_owned(), self.env.get_body()));
                // Return true
                Ok(true)
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
        // Clear environment
        self.env.clear();
        // Statements
        for stmt in prog.0 {
            match stmt.0 {
                Some(ident) => {
                    // Evaluate expression
                    let eval_e = self.eval_expr(stmt.1)?;
                    // Store in environment
                    self.env.push(&ident, eval_e)
                },
                None => ()
            }
        };
        // Program body
        self.eval_expr(prog.1)
    }
}