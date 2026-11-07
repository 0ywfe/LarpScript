use crate::error::RuntimeError;
use crate::parser::Rule;
use crate::runtime::Value;
use pest::iterators::Pair;
use pest::pratt_parser::{Assoc, Op, PrattParser};
use std::sync::LazyLock;

static PRATT_PARSER: LazyLock<PrattParser<Rule>> = LazyLock::new(|| {
    PrattParser::new()
        .op(Op::infix(Rule::logical_or, Assoc::Left))
        .op(Op::infix(Rule::logical_and, Assoc::Left))
        .op(Op::infix(Rule::equal, Assoc::Left) | Op::infix(Rule::not_equal, Assoc::Left))
        .op(Op::infix(Rule::greater_than, Assoc::Left)
            | Op::infix(Rule::less_than, Assoc::Left)
            | Op::infix(Rule::greater_equal, Assoc::Left)
            | Op::infix(Rule::less_equal, Assoc::Left))
        .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::subtract, Assoc::Left))
        .op(Op::infix(Rule::multiply, Assoc::Left)
            | Op::infix(Rule::divide, Assoc::Left)
            | Op::infix(Rule::modulo, Assoc::Left))
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
            "fax" => Ok(Value::Boolean(true)),
            "cap" => Ok(Value::Boolean(false)),
            _ => Err(RuntimeError::InvalidBoolean),
        },
        Rule::null => Ok(Value::Null),
        Rule::expr => evaluate_expression(primary),
        rule => Err(RuntimeError::UnexpectedToken(rule)),
    }
}

fn evaluate_primary_with_lookup(
    primary: Pair<Rule>,
    lookup: &mut dyn FnMut(&str) -> Result<Value, RuntimeError>,
) -> Result<Value, RuntimeError> {
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
            "fax" => Ok(Value::Boolean(true)),
            "cap" => Ok(Value::Boolean(false)),
            _ => Err(RuntimeError::InvalidBoolean),
        },
        Rule::null => Ok(Value::Null),
        Rule::identifier => lookup(primary.as_str()),
        Rule::expr => evaluate_expression_with_lookup(primary, lookup),
        rule => Err(RuntimeError::UnexpectedToken(rule)),
    }
}

pub fn apply_unary(operator: Pair<Rule>, operand: Value) -> Result<Value, RuntimeError> {
    match operator.as_rule() {
        Rule::negative => match operand {
            Value::Number(num) => Ok(Value::Number(-num)),
            _ => Err(RuntimeError::InvalidOperandType),
        },
        Rule::logical_not => match operand {
            Value::Boolean(value) => Ok(Value::Boolean(!value)),
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
    let rule = operator.as_rule();
    match rule {
        Rule::equal => Ok(Value::Boolean(left == right)),
        Rule::not_equal => Ok(Value::Boolean(left != right)),
        Rule::logical_and | Rule::logical_or => match (left, right) {
            (Value::Boolean(l), Value::Boolean(r)) => {
                let result = match rule {
                    Rule::logical_and => l && r,
                    Rule::logical_or => l || r,
                    _ => unreachable!(),
                };
                Ok(Value::Boolean(result))
            }
            _ => Err(RuntimeError::InvalidOperandType),
        },
        Rule::add => match (left, right) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
            (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
            _ => Err(RuntimeError::InvalidOperandType),
        },
        Rule::subtract
        | Rule::multiply
        | Rule::divide
        | Rule::modulo
        | Rule::power
        | Rule::greater_than
        | Rule::less_than
        | Rule::greater_equal
        | Rule::less_equal => {
            let (l, r) = match (left, right) {
                (Value::Number(l), Value::Number(r)) => (l, r),
                _ => return Err(RuntimeError::InvalidOperandType),
            };
            match rule {
                Rule::subtract => Ok(Value::Number(l - r)),
                Rule::multiply => Ok(Value::Number(l * r)),
                Rule::divide => {
                    if r == 0.0 {
                        Err(RuntimeError::DivisionByZero)
                    } else {
                        Ok(Value::Number(l / r))
                    }
                }
                Rule::modulo => {
                    if r == 0.0 {
                        Err(RuntimeError::DivisionByZero)
                    } else {
                        Ok(Value::Number(l % r))
                    }
                }
                Rule::power => Ok(Value::Number(l.powf(r))),
                Rule::greater_than => Ok(Value::Boolean(l > r)),
                Rule::less_than => Ok(Value::Boolean(l < r)),
                Rule::greater_equal => Ok(Value::Boolean(l >= r)),
                Rule::less_equal => Ok(Value::Boolean(l <= r)),
                _ => unreachable!(),
            }
        }
        rule => Err(RuntimeError::UnexpectedToken(rule)),
    }
}

pub fn evaluate_expression(expression: Pair<Rule>) -> Result<Value, RuntimeError> {
    let mut lookup = |name: &str| Err(RuntimeError::UndefinedVariable(name.to_string()));
    evaluate_expression_with_lookup(expression, &mut lookup)
}

pub fn evaluate_expression_with_lookup(
    expression: Pair<Rule>,
    lookup: &mut dyn FnMut(&str) -> Result<Value, RuntimeError>,
) -> Result<Value, RuntimeError> {
    PRATT_PARSER
        .map_primary(|primary| evaluate_primary_with_lookup(primary, lookup))
        .map_prefix(|operator, operand| apply_unary(operator, operand?))
        .map_postfix(|operand, operator| apply_postfix(operator, operand?))
        .map_infix(|left, operator, right| apply_binary(left?, operator, right?))
        .parse(expression.into_inner())
}
