pub mod lexer {
    use regex::Regex;
    use crate::types::token::*;

    type ValueConstructor = fn(&str) -> Result<Value, ()>;

    fn value_none(_: &str) -> Result<Value, ()> { Ok(Value::None) }
    fn value_ident(x: &str) -> Result<Value, ()> { Ok(Value::Str(x.to_string())) }

    const TOKEN_COUNT: usize = 5;

    const TOKENS: [(&str, Variant, ValueConstructor); TOKEN_COUNT] = [
        (r"\\", Variant::Lambda, value_none),
        (r"\.", Variant::Dot, value_none),
        (r"\(", Variant::LParen, value_none),
        (r"\)", Variant::RParen, value_none),
        (r"[a-z][A-Z]+", Variant::Ident, value_ident)
    ];

    pub struct Lexer {
        tokens: [(Regex, Variant, ValueConstructor); TOKEN_COUNT],
    }
}