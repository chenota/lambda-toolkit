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

fn bop_to_str(x: &ast::Bop) -> String {
    (match x {
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
    }).to_string()
}

fn uop_to_str(x: &ast::Uop) -> String {
    (match x {
        ast::Uop::NegUop => "-",
        ast::Uop::NotUop => "!",
    }).to_string()
}

fn print_level(level: usize) {
    for _ in 0..level { print!("| ") }
}

fn print_operator(tree: &str, level: usize) {
    // Print level
    print_level(level);
    // Print wrapper
    print!("Operator(");
    // Print line
    print!("{}", tree);
    // Print closing paren
    print!(")")
}

fn print_var(tree: &str, level: usize) {
    // Print level
    print_level(level);
    // Print wrapper
    print!("Var(");
    // Print line
    print!("{}", tree);
    // Print closing paren
    print!(")")
}

fn print_parameters(tree: &Vec<ast::Ident>, level: usize) {
    // Print level
    print_level(level);
    // Print wrapper
    print!("Parameters(");
    // Print list of parameters
    for (i, id) in tree.iter().enumerate() {
        match id {
            Some(s) => print!("{}", s),
            None => print!("_")
        }
        if i < tree.len() - 1 { print!(", ") }
    }
    // Print end paren
    print!(")")
}

fn print_expression(tree: &ast::Expression, level: usize) {
    // Print level
    print_level(level);
    // Check expression type
    match tree {
        ast::Expression::ApplicationExpr(elist) => {
            // Header
            println!("Application Chain");
            // Print list of applications
            for (i, ex) in elist.iter().enumerate() { 
                print_expression(ex, level + 1);
                // Newline
                if i < elist.len() - 1 { println!() };
            }
        },
        ast::Expression::BopExpr(b, e1, e2) => {
            // Header
            println!("Binary Operation");
            // Operator
            print_operator(bop_to_str(b).as_str(), level + 1);
            // Newline
            println!();
            // Expressions
            print_expression(e1.as_ref(), level + 1);
            // Newline
            println!();
            print_expression(e2.as_ref(), level + 1);
        },
        ast::Expression::UopExpr(u, e) => {
            // Header
            println!("Unary Operation");
            // Operator
            print_operator(uop_to_str(u).as_str(), level + 1);
            // Newline
            println!();
            // Expression
            print_expression(e.as_ref(), level + 1);
        },
        ast::Expression::FuncExpr(ilist, body) => {
            // Header
            println!("Function Definition");
            // Parameters
            print_parameters(ilist, level + 1);
            // Newline
            println!();
            // Body
            print_expression(body.as_ref(), level + 1)
        },
        ast::Expression::ValExpr(v) => {
            // Print value
            print!("{}", match v {
                ast::Value::Boolean(x) => {
                    let s = if *x {"true"} else {"false"};
                    "Bool(".to_string() + s + ")"
                },
                ast::Value::Identifier(x) => {
                    "Ident(".to_string() + x + ")"
                },
                ast::Value::Number(x) => {
                    let s = x.to_string();
                    "Num(".to_string() + &s + ")"
                },
                ast::Value::Unit => {
                    "Unit(_)".to_string()
                },
                ast::Value::Closure(_,_,_) => {
                    "Closure".to_string()
                }
            })
        },
        ast::Expression::EnvExpr(_, e) => {
            print_expression(e.as_ref(), level)
        }
    }
}

fn print_statement(tree: &ast::Statement, level: usize) {
    // Print level
    print_level(level);
    // Header
    println!("Let");
    // Identifier
    match &tree.0 {
        Some(s) => print_var(s, level + 1),
        None => print_var("_", level + 1)
    };
    // Newline
    println!();
    // Expression
    print_expression(&tree.1, level + 1);
}

pub fn print_program(tree: &ast::Program) {
    // Header
    println!("Program");
    // Print statements
    for s in &tree.0 {
        print_statement(s, 1);
        // Newline
        println!();
    };
    // Print expression
    print_expression(&tree.1, 1);
    // Newline
    println!()
}

pub fn print_grouped_expression(tree: &ast::Expression, outer: bool) {
    // Always disable outer parenthesis for values and enable for functions
    let mut outer = outer;
    match tree { 
        ast::Expression::ValExpr(_) => outer = false, 
        ast::Expression::FuncExpr(_, _) => outer = true, 
        _ => () 
    }
    // Opening paren
    if outer { print!("(") }
    // Expression
    match tree {
        ast::Expression::ApplicationExpr(alist) => {
            // Print len(vec) - 2 open parenthesis
            for _ in 0..(alist.len() - 2) { print!("(") };
            // Print first item in expression
            print_grouped_expression(alist.first().unwrap(), true);
            // Print space
            print!(" ");
            // Print rest of items in application
            for (i, ex) in alist[1..].iter().enumerate() {
                // Print expression
                print_grouped_expression(ex, true);
                // Print space except for last item
                if i < alist.len() - 2 { print!(") ") }
            }
        },
        ast::Expression::BopExpr(b, e1, e2) => {
            // Print first expression
            print_grouped_expression(e1.as_ref(), true);
            // Print bop
            print!(" {} ", bop_to_str(b));
            // Print second expression
            print_grouped_expression(e2.as_ref(), true);
        },
        ast::Expression::FuncExpr(plist, body) => {
            // Print lambda
            print!("\\");
            // Print parameters
            for (i, p) in plist.iter().enumerate() {
                match p {
                    Some(s) => print!("{}", s),
                    None => print!("_")
                }
                if i < plist.len() - 1 { print!(" ") }
            };
            // Dot
            print!(". ");
            // Body
            print_grouped_expression(body.as_ref(), false);
        },
        ast::Expression::UopExpr(op, body) => {
            // Operator
            print!("{}", uop_to_str(op));
            // Expression
            print_grouped_expression(body.as_ref(), true);
        },
        ast::Expression::ValExpr(v) => {
            print!("{}", match v {
                ast::Value::Boolean(x) => (if *x { "true" } else { "false" }).to_string(),
                ast::Value::Identifier(x) => x.to_string(),
                ast::Value::Number(x) => x.to_string(),
                ast::Value::Unit => "_".to_string(),
                ast::Value::Closure(params, body, _) => {
                    // Print as function
                    print_grouped_expression(&ast::Expression::FuncExpr(params.clone(), body.clone()), true);
                    // Return nothing
                    "".to_string()
                }
            })
        },
        ast::Expression::EnvExpr(_, e) => {
            print_grouped_expression(e.as_ref(), false)
        }
    }
    // Closing paren
    if outer { print!(")") }
}

pub fn print_group(tree: &ast::Program) {
    // Print statements
    for s in &tree.0 {
        // Let keyword
        print!("let ");
        // Identifier
        match &s.0 {
            Some(s) => print!("{}", s),
            None => print!("_")
        }
        // = keyword
        print!(" = ");
        // Expression
        print_grouped_expression(&s.1, false);
        // in keyword and newline
        println!(" in")
    };
    // Print expression
    print_grouped_expression(&tree.1, false);
    // Newline
    println!()
}