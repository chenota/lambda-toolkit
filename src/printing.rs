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