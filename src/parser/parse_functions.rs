use super::parse_data::*;
use Option::{None, Some};
use Token::*;
use TokenizationError::*;
use crate::parser::parse_data::EvaluationError::{DivideByZero, SymbolValueMismatch, TrailingSymbolsOrValues, UnknownSymbol};


pub fn tokenize_string(equation: &String) -> Result<Vec<Token>, TokenizationError> {
    let mut token_vec: Vec<Token> = Vec::new();
    let mut value_being_built: Option<i32> = None;

    for character in equation.chars() {
        match character {
            '0'..='9' => value_being_built = match value_being_built {
                Some(current_value) => Some(current_value * 10 + (character as i32 - '0' as i32)),
                None =>  Some(character as i32  - '0' as i32)
            },
            '+' | '-' | '*' | '/' => {
                if let Some(constructed_value) = value_being_built {
                    token_vec.push(Value(constructed_value));
                    value_being_built = None;
                }

                match character {
                    '+' => token_vec.push(Symbol(ADD)),
                    '-' => token_vec.push(Symbol(SUBTRACT)),
                    '*' => token_vec.push(Symbol(MULTIPLY)),
                    '/' => token_vec.push(Symbol(DIVIDE)),
                    _ => unreachable!()
                }
            },
            _ => return Err(BadToken { token: character })
        }
    }

    if let Some(constructed_value) = value_being_built {
        token_vec.push(Value(constructed_value))
    } else {
        return Err(EndOnInfixSymbol)
    }

    Ok(token_vec)
}

pub fn evaluate_expression(expression: &Vec<Token>) -> Result<i32, EvaluationError> {
    let mut value_stack: Vec<i32> = Vec::new();
    let mut symbol_stack: Vec<&SymbolData> = Vec::new();

    for token in expression {
        match token {
            Value(raw_value) => value_stack.push(raw_value.clone()),
            Symbol(symbol_value) => {
                loop {
                    // If there's a symbol on the stack we need to deal with that
                    if let Some(top_symbol) = symbol_stack.last() {
                        // When the symbol we're currently encountering has a greater or equal precedence, evaluate it
                        if top_symbol.precedence >= symbol_value.precedence {
                            execute_symbol(&mut value_stack, &mut symbol_stack)?;
                        } else {
                            // Otherwise just push the symbol and move on
                            symbol_stack.push(symbol_value);
                            break;
                        }
                    } else {
                        // Otherwise we can just drop the symbol onto the stack and move on
                        symbol_stack.push(symbol_value);
                        break
                    }
                }
            },
        }
    }

    // Execute any remaining symbols
    while !symbol_stack.is_empty() {
        execute_symbol(&mut value_stack, &mut symbol_stack)?;
    }

    let final_value = value_stack.pop();

    if !value_stack.is_empty() || !symbol_stack.is_empty() {
        Err(TrailingSymbolsOrValues { leftover_values: value_stack.len(), leftover_symbols: symbol_stack.len() })
    } else {
        final_value.ok_or(SymbolValueMismatch)
    }
}

fn execute_symbol(value_stack: &mut Vec<i32>, symbol_stack: &mut Vec<&SymbolData>) -> Result<(), EvaluationError> {
    let second_value = value_stack.pop().ok_or_else(|| SymbolValueMismatch)?;
    let first_value = value_stack.pop().ok_or_else(|| SymbolValueMismatch)?;
    let the_top_symbol = symbol_stack.pop().unwrap();

    match *the_top_symbol {
        ADD => value_stack.push(first_value + second_value),
        SUBTRACT => value_stack.push(first_value - second_value),
        MULTIPLY => value_stack.push(first_value * second_value),
        DIVIDE => {
            if second_value == 0 {
                return Err(DivideByZero);
            }
            value_stack.push(first_value / second_value);
        },
        _ => return Err(UnknownSymbol(the_top_symbol.clone())),
    }

    Ok(())
}

