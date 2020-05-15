use parser::parse_data::*;
use parser::parse_functions::{tokenize_string, evaluate_expression};

mod parser;

fn main() {
    let expression = String::from("1+2*3-4");
    let tokens_result = tokenize_string(&expression);
    let tokens = match tokens_result {
        Ok(token_vec) => token_vec,
        Err(token_err) => {
            match token_err {
                TokenizationError::EndOnInfixSymbol => eprintln!("Expression ended on an infix symbol."),
                TokenizationError::BadToken { token} => eprintln!("Encountered an invalid token: {}", token),
            };
            return;
        },
    };

    let evaluation_result = evaluate_expression(&tokens);
    match evaluation_result {
        Ok(result) => println!("Expression evaluated to {}.", result),
        Err(eval_err) => match eval_err {
            EvaluationError::SymbolValueMismatch => eprintln!("Expression was malformed, a symbol-value mismatch occurred."),
            EvaluationError::DivideByZero => eprintln!("Illegal operation: divided by zero"),
            EvaluationError::UnknownSymbol(symbol) => eprintln!("Unrecognized symbol: {}", symbol.representation),
            EvaluationError::TrailingSymbolsOrValues { leftover_symbols, leftover_values} =>
                eprintln!("Imbalanced expression, there are leftover symbols or values. Symbols: {} Values: {}", leftover_symbols, leftover_values),
        },
    };
}
