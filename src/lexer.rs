pub mod lexer {
    use regex::Regex;
    use crate::types::token::*;

    // Value constructor function type
    type ValueConstructor = fn(&str) -> Result<TokenValue, ()>;

    // Variant option enum
    enum VariantOption {
        Some(Variant, ValueConstructor),
        None,
        Newline
    }

    // Value constructor functions
    fn value_none(_: &str) -> Result<TokenValue, ()> { Ok(TokenValue::None) }
    fn value_ident(x: &str) -> Result<TokenValue, ()> { Ok(TokenValue::Str(x.to_string())) }

    // Number to available tokens
    const TOKEN_COUNT: usize = 7;

    // Tokens
    const TOKENS: [(&str, VariantOption); TOKEN_COUNT] = [
        (r"\\", VariantOption::Some(Variant::Lambda, value_none)),
        (r"\.", VariantOption::Some(Variant::Dot, value_none)),
        (r"\(", VariantOption::Some(Variant::LParen, value_none)),
        (r"\)", VariantOption::Some(Variant::RParen, value_none)),
        (r"[a-z][A-Z]+", VariantOption::Some(Variant::Ident, value_ident)),
        (r"\n", VariantOption::Newline),
        (r"(\s)+", VariantOption::None),
    ];

    // Lexer object
    pub struct Lexer {
        tokens: Vec<(Regex, VariantOption)>,
    }
    impl Lexer {
        // Initialize new lexer
        pub fn new() -> Lexer {
            let mut lex = Lexer{
                tokens: Vec::new()
            };
            for t in TOKENS {
                lex.tokens.push((
                    Regex::new(t.0).unwrap(),
                    t.1,
                ))
            };
            lex
        }
        // Next token in stream
        fn next(&mut self, stream: &str, pos: usize) -> Option<Token> {
            None
        }
        pub fn generate(&mut self, stream: &str) -> Result<Vec<Token>, &str> {
            // Reset row and column
            let mut row: usize = 0;
            let mut col: usize = 0;
            let mut pos: usize = 0;
            // Tokens vector
            let mut tokens: Vec<Token> = Vec::new();
            // Return
            Ok(tokens)
        }
    }
}