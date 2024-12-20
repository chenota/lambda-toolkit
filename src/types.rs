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
        CNumber,
        CBoolean,
        Number,
        Boolean,
    }

    // Token value
    pub enum TokenValue {
        Str(String),
        Number(u128),
        None
    }
    // Position stores row and column
    pub type Position = (usize, usize);
    // Tuple of both
    pub type Token = (Variant, TokenValue, Position);
}

pub mod eval {

}