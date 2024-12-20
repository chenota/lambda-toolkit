use regex::Regex;
use crate::types::token::*;

// Macro that puts carrot at the beginning of regex
macro_rules! reg {
    ($e: expr) => {
        concat!("^", $e)
    }
}

// Value constructor function type
type ValueConstructor = fn(&str) -> TokenValue;

// Variant option enum
enum VariantOption {
    Some(Variant, ValueConstructor),
    None,
    Newline
}

// Value constructor functions
fn value_none(_: &str) -> TokenValue { TokenValue::None }
fn value_ident(x: &str) -> TokenValue { TokenValue::Str(x.to_string()) }

// Number to available tokens
const TOKEN_COUNT: usize = 7;

// Tokens
const TOKENS: [(&str, VariantOption); TOKEN_COUNT] = [
    (reg!(r"\\"), VariantOption::Some(Variant::Lambda, value_none)),
    (reg!(r"\."), VariantOption::Some(Variant::Dot, value_none)),
    (reg!(r"\("), VariantOption::Some(Variant::LParen, value_none)),
    (reg!(r"\)"), VariantOption::Some(Variant::RParen, value_none)),
    (reg!(r"[a-z][A-Z]+"), VariantOption::Some(Variant::Ident, value_ident)),
    (reg!(r"\n"), VariantOption::Newline),
    (reg!(r"(\s)+"), VariantOption::None),
];

// Lexer object
pub struct Lexer {
    pos: usize,
    row: usize,
    col: usize,
    tokens: Vec<(Regex, VariantOption)>,
}
impl Lexer {
    // Initialize new lexer
    pub fn new() -> Lexer {
        let mut lex = Lexer{
            pos: 0,
            row: 0,
            col: 0,
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
    fn next(&mut self, stream: &str) -> Option<Token> {
        // Length of longest match
        let mut longest_match: usize = 0;
        // Index pointing to variant of longest match (initialize to zero, doesn't really matter)
        let mut longest_variant: usize = 0;
        // Iterate through each token, find longest match
        for (i, token_def) in self.tokens.iter().enumerate() {
            match token_def.0.find_at(stream, self.pos) {
                Some(m) => {
                    if m.len() > longest_match {
                        longest_match = m.len();
                        longest_variant = i;
                    }
                },
                None => ()
            }
        };
        // If found token
        if longest_match > 0 {
            // Update col
            self.col += longest_match;
            // Update position
            self.pos += longest_match;
            // Check matched token
            match &self.tokens.get(longest_variant).unwrap().1 {
                // If matched usable token, get value and return
                VariantOption::Some(var, producer) => {
                    Some((
                        var.clone(), // Token variant
                        producer(&stream[self.pos..self.pos+longest_match]), // Token value
                        (self.row, self.col - longest_match) // Token position (need to revert to old col)
                    ))
                },
                // If matched throwaway token, return none
                VariantOption::None => {
                    None
                },
                // If matched newline, update row and column then return none
                VariantOption::Newline => {
                    self.row += 1;
                    self.col = 0;
                    None
                }
            }
        }
        // Did not find token, return none
        else {
            None
        }
    }
    // Generate stream of tokens
    pub fn generate(&mut self, stream: &str) -> Result<Vec<Token>, String> {
        // Reset pos, row, and column
        self.pos = 0;
        self.row = 0;
        self.col = 0;
        // Tokens vector
        let mut tokens: Vec<Token> = Vec::new();
        // Iterate through stream
        loop {
            // Save old position
            let old_pos = self.pos;
            // Generate next token
            let next_token = self.next(stream);
            // Match next token
            match next_token {
                Some(t) => {
                    // Push generated token
                    tokens.push(t)
                },
                None => {
                    // If current position = old position, error
                    if self.pos == old_pos {
                        return Err("Unexpected character at line ".to_string() + self.row.to_string().as_ref() + ", character " + self.col.to_string().as_ref())
                    }
                }
            }
            // If reached end of stream, break
            if self.pos >= stream.len() {
                break
            }
        }
        // Add EOF to end of stream
        tokens.push((Variant::EOF, TokenValue::None, (self.row, self.col)));
        // Return
        Ok(tokens)
    }
}