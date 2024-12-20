use crate::types::token;

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
            token::Variant::Ident => {
                let s = match &t.1 {
                    token::TokenValue::Str(s) => s,
                    _ => "ERR"
                };
                "IDENT(".to_string() + s + ")"
            },
            token::Variant::CNumber => {
                let s = match &t.1 {
                    token::TokenValue::Number(n) => &n.to_string(),
                    _ => "ERR"
                };
                "CNUMBER(".to_string() + s + ")"
            },
            token::Variant::Number => {
                let s = match &t.1 {
                    token::TokenValue::Number(n) => &n.to_string(),
                    _ => "ERR"
                };
                "NUMBER(".to_string() + s + ")"
            },
            token::Variant::CBoolean => {
                let s = match &t.1 {
                    token::TokenValue::Boolean(b) => if *b {"true"} else {"false"},
                    _ => "ERR"
                };
                "CBOOLEAN(".to_string() + s + ")"
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