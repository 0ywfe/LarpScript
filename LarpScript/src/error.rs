use crate::parser::Rule;

#[derive(Debug)]
pub enum RuntimeError {
    InvalidNumber,
    InvalidBoolean,
    InvalidOperandType,
    DivisionByZero,
    UnexpectedToken(Rule),
}
