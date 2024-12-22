use crate::types::token;
use crate::types::ast;

pub fn print_token_stream(stream: &Vec<token::Token>) -> () {
    print!("[");
    for (i, t) in stream.iter().enumerate() {
        // Print token
        print!("{}", match t.0 {
            token::Variant::Lambda => "LAMBDA".to_string(),
            token::Variant::LParen => "LPAREN".to_string(),
            token::Variant::RParen => "RPAREN".to_string(),
            token::Variant::Dot => "DOT".to_string(),
            token::Variant::EOF => "EOF".to_string(),
            token::Variant::Plus => "PLUS".to_string(),
            token::Variant::Minus => "MINUS".to_string(),
            token::Variant::Times => "TIMES".to_string(),
            token::Variant::Div => "DIVIDE".to_string(),
            token::Variant::Gt => "GT".to_string(),
            token::Variant::Gte => "GTE".to_string(),
            token::Variant::Lt => "LT".to_string(),
            token::Variant::Lte => "LTE".to_string(),
            token::Variant::Eq => "EQUALS".to_string(),
            token::Variant::Not => "NOT".to_string(),
            token::Variant::And => "AND".to_string(),
            token::Variant::Or => "OR".to_string(),
            token::Variant::Xor => "XOR".to_string(),
            token::Variant::Let => "LET".to_string(),
            token::Variant::In => "IN".to_string(),
            token::Variant::Unit => "UNIT".to_string(),
            token::Variant::Ident => {
                let s = match &t.1 {
                    token::TokenValue::Str(s) => s,
                    _ => "ERR"
                };
                "IDENT(".to_string() + s + ")"
            },
            token::Variant::Number => {
                let s = match &t.1 {
                    token::TokenValue::Number(n) => &n.to_string(),
                    _ => "ERR"
                };
                "NUMBER(".to_string() + s + ")"
            },
            token::Variant::Boolean => {
                let s = match &t.1 {
                    token::TokenValue::Boolean(b) => if *b {"true"} else {"false"},
                    _ => "ERR"
                };
                "BOOLEAN(".to_string() + s + ")"
            },
        });
        // Print semicolon and space
        if i < stream.len() - 1 {
            print!("; ")
        }
    }
    println!("]")
}

fn print_string(tree: &String, level: usize) {
    // Print level
    for _ in 0..level { print!("|  ") }
    // Print line
    println!("{}", tree);
}

fn print_str(tree: &str, level: usize) {
    // Print level
    for _ in 0..level { print!("|  ") }
    // Print line
    println!("{}", tree);
}

fn print_expression(tree: &ast::Expression, level: usize) {
    // Print level
    for _ in 0..level { print!("|  ") }
    // Check expression type
    match tree {
        ast::Expression::ApplicationExpr(elist) => {
            // Header
            println!("ApplicationExpr");
            // Print list of applications
            for ex in elist { print_expression(ex, level + 1) }
        },
        ast::Expression::BopExpr(b, e1, e2) => {
            // Header
            println!("BopExpr");
            // Operator
            print_str(match b {
                ast::Bop::AndBop => "&",
                ast::Bop::OrBop => "|",
                ast::Bop::XorBop => "^",
                ast::Bop::PlusBop => "+",
                ast::Bop::MinusBop => "-",
                ast::Bop::TimesBop => "*",
                ast::Bop::DivBop => "/",
                ast::Bop::LtBop => "<",
                ast::Bop::LteBop => "<=",
                ast::Bop::GtBop => ">",
                ast::Bop::GteBop => ">=",
                ast::Bop::EqBop => "="
            }, level + 1);
            // Expressions
            print_expression(e1.as_ref(), level + 1);
            print_expression(e2.as_ref(), level + 1);
        },
        ast::Expression::UopExpr(u, e) => {
            // Header
            println!("UopExpr");
            // Operator
            print_str(match u {
                ast::Uop::NegUop => "-",
                ast::Uop::NotUop => "!",
            }, level + 1);
            // Expression
            print_expression(e.as_ref(), level + 1);
        },
        ast::Expression::FuncExpr(ilist, body) => {
            // Header
            println!("FuncExpr");
            // Parameters
            for id in ilist {
                match id {
                    Some(s) => print_string(s, level + 1),
                    None => print_str("_", level + 1)
                }
            };
            // Body
            print_expression(body.as_ref(), level + 1)
        },
        ast::Expression::ValExpr(v) => {
            // Header
            println!("Value");
            // Print value
            match v {
                ast::Value::Boolean(x) => {
                    let s = if *x {"true"} else {"false"};
                    print_string(&("Boolean(".to_string() + s + ")"), level + 1)
                },
                ast::Value::Identifier(x) => {
                    print_string(&("Identifier(".to_string() + x + ")"), level + 1)
                },
                ast::Value::Number(x) => {
                    let s = x.to_string();
                    print_string(&("Number(".to_string() + &s + ")"), level + 1)
                },
                ast::Value::Unit => {
                    print_str("Unit(_)", level + 1)
                }
            }
        }
    }
    // Newline
    println!()
}

fn print_statement(tree: &ast::Statement, level: usize) {
    // Print level
    for _ in 0..level { print!("|  ") }
    // Header
    println!("Let");
    // Identifier
    match &tree.0 {
        Some(s) => print_string(s, level + 1),
        None => print_str("_", level + 1)
    };
    // Expression
    print_expression(&tree.1, level + 1);
    // Newline
    println!()
}

pub fn print_program(tree: &ast::Program) {
    // Header
    println!("Program");
    // Print statements
    for s in &tree.0 {
        print_statement(s, 1)
    };
    // Print expression
    print_expression(&tree.1, 1);
    // Newline
    println!()
}