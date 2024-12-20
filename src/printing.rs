pub mod printing {
    use crate::types::token;

    pub fn print_token_stream(stream: &Vec<token::Token>) -> () {
        print!("[");
        for t in stream {
            // Print static tokens
            print!("{}", match t.0 {
                token::Variant::Lambda => "LAMBDA",
                token::Variant::LParen => "LPAREN",
                token::Variant::RParen => "RPAREN",
                token::Variant::Dot => "DOT",
                token::Variant::EOF => "EOF",
                _ => ""
            });
            // Print dynamic tokens
            print!("{}", match t.0 {
                token::Variant::Ident => {
                    let s = match &t.1 {
                        token::TokenValue::Str(s) => s,
                        _ => ""
                    };
                    "IDENT(".to_string() + s + ")"
                },
                _ => "".to_string()
            });
            // Print semicolon
            print!("; ")
        }
        println!("]")
    }
}