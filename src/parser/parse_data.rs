#[derive(PartialEq, Eq, Clone)]
pub struct SymbolData {
    pub representation: char,
    pub precedence: u8,
}

pub enum TokenizationError {
    EndOnInfixSymbol,
    BadToken { token: char },
}

pub enum Token {
    Value(i32),
    Symbol(SymbolData),
}

pub const MULTIPLY: SymbolData = SymbolData {
    representation: '*',
    precedence: 2,
};
pub const DIVIDE: SymbolData = SymbolData {
    representation: '/',
    precedence: 2,
};
pub const ADD: SymbolData = SymbolData {
    representation: '+',
    precedence: 1,
};
pub const SUBTRACT: SymbolData = SymbolData {
    representation: '-',
    precedence: 1,
};

pub enum EvaluationError {
    DivideByZero,
    SymbolValueMismatch,
    UnknownSymbol(SymbolData),
    TrailingSymbolsOrValues { leftover_symbols: usize, leftover_values: usize }
}
