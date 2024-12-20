pub mod ast {

}

pub mod token {
    // Token variant
    pub enum Variant {
        Lambda,
        Dot,
        LParen,
        RParen,
        Ident,
    }

    // Token value
    pub enum TokenValue {
        Str(String),
        None
    }
    // Position stores row and column
    pub type Position = (u64, u64);
    // Tuple of both
    pub type Token = (Variant, TokenValue, Position);
}

pub mod eval {

}