use crate::error::RuntimeError;
use crate::parser::Rule;
use crate::runtime::Value;

use pest::iterators::Pair;
use pest::pratt_parser::{Assoc, Op, PrattParser};
use std::sync::LazyLock;

static PRATT_PARSER: LazyLock<PrattParser<Rule>> = LazyLock::new(|| {
    PrattParser::new()
        .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::subtract, Assoc::Left))
        .op(Op::infix(Rule::multiply, Assoc::Left) | Op::infix(Rule::divide, Assoc::Left))
        .op(Op::infix(Rule::power, Assoc::Right))
        .op(Op::prefix(Rule::negative) | Op::prefix(Rule::logical_not))
        .op(Op::postfix(Rule::fac))
});

pub fn evaluate_primary(primary: Pair<Rule>) -> Result<Value, RuntimeError> {
    match primary.as_rule() {
        Rule::number => {
            let number: f64 = primary
                .as_str()
                .parse()
                .map_err(|_| RuntimeError::InvalidNumber)?;

            Ok(Value::Number(number))
        }
        Rule::string => {
            let raw = primary.as_str();

            let value = raw
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .unwrap_or(raw);

            Ok(Value::String(value.to_string()))
        }
        Rule::boolean => match primary.as_str() {
            "true" => Ok(Value::Boolean(true)),
            "false" => Ok(Value::Boolean(false)),
            _ => Err(RuntimeError::InvalidBoolean),
        },
        Rule::null => Ok(Value::Null),
        Rule::expr => evaluate_expression(primary),
        rule => Err(RuntimeError::UnexpectedToken(rule)),
    }
}

pub fn apply_unary(operator: Pair<Rule>, operand: Value) -> Result<Value, RuntimeError> {
    match operator.as_rule() {
        Rule::negative => match operand {
            Value::Number(num) => Ok(Value::Number(-num)),
            _ => Err(RuntimeError::InvalidOperandType),
        },
        rule => Err(RuntimeError::UnexpectedToken(rule)),
    }
}

pub fn apply_postfix(operator: Pair<Rule>, operand: Value) -> Result<Value, RuntimeError> {
    match operator.as_rule() {
        Rule::fac => {
            let num = match operand {
                Value::Number(num) => num,
                _ => return Err(RuntimeError::InvalidOperandType),
            };
            if !num.is_finite() || num < 0.0 || num.fract() != 0.0 || num > 170.0 {
                return Err(RuntimeError::InvalidOperandType);
            }
            let mut result = 1.0;
            for i in 1..=(num as u64) {
                result *= i as f64;
            }
            Ok(Value::Number(result))
        }
        rule => Err(RuntimeError::UnexpectedToken(rule)),
    }
}

pub fn apply_binary(
    left: Value,
    operator: Pair<Rule>,
    right: Value,
) -> Result<Value, RuntimeError> {
    let (left, right) = match (left, right) {
        (Value::Number(l), Value::Number(r)) => (l, r),
        _ => return Err(RuntimeError::InvalidOperandType),
    };
    match operator.as_rule() {
        Rule::add => Ok(Value::Number(left + right)),
        Rule::subtract => Ok(Value::Number(left - right)),
        Rule::multiply => Ok(Value::Number(left * right)),
        Rule::divide => {
            if right == 0.0 {
                Err(RuntimeError::DivisionByZero)
            } else {
                Ok(Value::Number(left / right))
            }
        }
        Rule::power => Ok(Value::Number(left.powf(right))),
        rule => Err(RuntimeError::UnexpectedToken(rule)),
    }
}

pub fn evaluate_expression(expression: Pair<Rule>) -> Result<Value, RuntimeError> {
    PRATT_PARSER
        .map_primary(|primary| evaluate_primary(primary))
        .map_prefix(|operator, operand| apply_unary(operator, operand?))
        .map_postfix(|operand, operator| apply_postfix(operator, operand?))
        .map_infix(|left, operator, right| apply_binary(left?, operator, right?))
        .parse(expression.into_inner())
}
