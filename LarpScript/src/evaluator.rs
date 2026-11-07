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

fn parse_string(raw: &str) -> String {
    let inner = &raw[1..raw.len() - 1];
    let mut result = String::new();
    let mut chars = inner.chars();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('r') => result.push('\r'),
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(ch);
        }
    }

    result
}

fn evaluate_primary(
    primary: Pair<Rule>,
    lookup: &mut dyn FnMut(&str) -> Result<Value, RuntimeError>,
    call: &mut dyn FnMut(&str, Vec<Value>) -> Result<Value, RuntimeError>,
) -> Result<Value, RuntimeError> {
    match primary.as_rule() {
        Rule::number => {
            let number = primary
                .as_str()
                .parse::<f64>()
                .map_err(|_| RuntimeError::InvalidNumber)?;
            if !number.is_finite() {
                return Err(RuntimeError::InvalidNumber);
            }
            Ok(Value::Number(number))
        }
        Rule::string => Ok(Value::String(parse_string(primary.as_str()))),
        Rule::boolean => match primary.as_str() {
            "fax" => Ok(Value::Boolean(true)),
            "cap" => Ok(Value::Boolean(false)),
            _ => Err(RuntimeError::InvalidBoolean),
        },
        Rule::null => Ok(Value::Null),
        Rule::identifier => lookup(primary.as_str()),
        Rule::expr => evaluate_expression_with_callbacks(primary, lookup, call),
        Rule::call => {
            let mut inner = primary.into_inner();
            let name = inner
                .next()
                .ok_or(RuntimeError::MalformedStatement)?
                .as_str()
                .to_string();
            let mut arguments = Vec::new();
            for expression in inner {
                arguments.push(evaluate_expression_with_callbacks(
                    expression, lookup, call,
                )?);
            }
            call(&name, arguments)
        }
        rule => Err(RuntimeError::UnexpectedToken(rule)),
    }
}

fn apply_unary(operator: Pair<Rule>, operand: Value) -> Result<Value, RuntimeError> {
    match operator.as_rule() {
        Rule::negative => match operand {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(RuntimeError::InvalidOperandType),
        },
        Rule::logical_not => match operand {
            Value::Boolean(value) => Ok(Value::Boolean(!value)),
            _ => Err(RuntimeError::InvalidOperandType),
        },
        rule => Err(RuntimeError::UnexpectedToken(rule)),
    }
}

fn apply_postfix(operand: Value, operator: Pair<Rule>) -> Result<Value, RuntimeError> {
    match operator.as_rule() {
        Rule::fac => {
            let n = match operand {
                Value::Number(n) => n,
                _ => return Err(RuntimeError::InvalidOperandType),
            };
            if !n.is_finite() || n < 0.0 || n.fract() != 0.0 || n > 170.0 {
                return Err(RuntimeError::InvalidOperandType);
            }
            let mut result = 1.0;
            for i in 1..=(n as u64) {
                result *= i as f64;
            }
            Ok(Value::Number(result))
        }
        rule => Err(RuntimeError::UnexpectedToken(rule)),
    }
}

fn apply_binary(left: Value, operator: Pair<Rule>, right: Value) -> Result<Value, RuntimeError> {
    use Value::*;
    match operator.as_rule() {
        Rule::equal => Ok(Boolean(left == right)),
        Rule::not_equal => Ok(Boolean(left != right)),
        Rule::logical_and | Rule::logical_or => match (left, right) {
            (Boolean(l), Boolean(r)) => Ok(Boolean(if operator.as_rule() == Rule::logical_and {
                l && r
            } else {
                l || r
            })),
            _ => Err(RuntimeError::InvalidOperandType),
        },
        Rule::add => match (left, right) {
            (Number(l), Number(r)) => Ok(Number(l + r)),
            (String(l), String(r)) => Ok(String(l + &r)),
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
                (Number(l), Number(r)) => (l, r),
                _ => return Err(RuntimeError::InvalidOperandType),
            };
            match operator.as_rule() {
                Rule::subtract => Ok(Number(l - r)),
                Rule::multiply => Ok(Number(l * r)),
                Rule::divide | Rule::modulo if r == 0.0 => Err(RuntimeError::DivisionByZero),
                Rule::divide => Ok(Number(l / r)),
                Rule::modulo => Ok(Number(l % r)),
                Rule::power => Ok(Number(l.powf(r))),

                Rule::greater_than => Ok(Boolean(l > r)),
                Rule::less_than => Ok(Boolean(l < r)),
                Rule::greater_equal => Ok(Boolean(l >= r)),
                Rule::less_equal => Ok(Boolean(l <= r)),

                _ => unreachable!(),
            }
        }

        rule => Err(RuntimeError::UnexpectedToken(rule)),
    }
}

pub fn evaluate_expression_with_callbacks(
    expression: Pair<Rule>,
    lookup: &mut dyn FnMut(&str) -> Result<Value, RuntimeError>,
    call: &mut dyn FnMut(&str, Vec<Value>) -> Result<Value, RuntimeError>,
) -> Result<Value, RuntimeError> {
    PRATT_PARSER
        .map_primary(|primary| evaluate_primary(primary, lookup, call))
        .map_prefix(|operator, operand| apply_unary(operator, operand?))
        .map_postfix(|operand, operator| apply_postfix(operand?, operator))
        .map_infix(|left, operator, right| apply_binary(left?, operator, right?))
        .parse(expression.into_inner())
}
