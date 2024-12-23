pub mod ast {
    pub type Ident = Option<String>;

    pub type Program = (Vec<Statement>, Expression);

    pub type Statement = (Ident, Expression);

    #[derive(Clone)]
    pub enum Expression {
        UopExpr(Uop, Box<Expression>),
        BopExpr(Bop, Box<Expression>, Box<Expression>),
        ApplicationExpr(Vec<Expression>),
        FuncExpr(Vec<Ident>, Box<Expression>),
        ValExpr(Value)
    }

    #[derive(Clone)]
    pub enum Value {
        Identifier(String),
        Number(i128),
        Boolean(bool),
        Unit
    }

    #[derive(Clone)]
    pub enum Uop {
        NegUop,
        NotUop,
    }

    #[derive(Clone)]
    pub enum Bop {
        PlusBop,
        MinusBop,
        TimesBop,
        DivBop,
        GtBop,
        GteBop,
        LtBop,
        LteBop,
        EqBop,
        AndBop,
        OrBop,
        XorBop
    }
}

pub mod token {
    // Token variant
    #[derive(Clone, PartialEq)]
    pub enum Variant {
        Lambda,
        Dot,
        LParen,
        RParen,
        Ident,
        EOF,
        Number,
        Boolean,
        Plus,
        Minus,
        Times,
        Div,
        Gt,
        Gte,
        Lt,
        Lte,
        Eq,
        Not,
        And,
        Or,
        Xor,
        Let,
        In,
        Unit,
    }

    #[derive(Clone)]
    // Token value
    pub enum TokenValue {
        Str(String),
        Number(i128),
        Boolean(bool),
        None
    }
    // Position stores row and column
    pub type Position = (usize, usize);
    // Tuple of both
    pub type Token = (Variant, TokenValue, Position);
}

pub mod eval {

}