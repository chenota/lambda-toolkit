pub mod ast {

}

pub mod token {
    // Token variant
    #[derive(Clone)]
    pub enum Variant {
        Lambda,
        Dot,
        LParen,
        RParen,
        Ident,
        EOF,
    }

    // Token value
    pub enum TokenValue {
        Str(String),
        None
    }
    // Position stores row and column
    pub type Position = (usize, usize);
    // Tuple of both
    pub type Token = (Variant, TokenValue, Position);
}

pub mod eval {

}