use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::evaluator::evaluate_expression_with_lookup;
use crate::parser::{Rule, parse_program};
use crate::runtime::Value;
use pest::iterators::Pair;

#[derive(Debug)]
pub enum LarpError {
    Syntax(String),
    Runtime(RuntimeError),
}

pub struct Interpreter {
    environment: Environment,
    output: Vec<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            output: Vec::new(),
        }
    }
    pub fn run_source(&mut self, source: &str) -> Result<(), LarpError> {
        let mut pairs =
            parse_program(source).map_err(|error| LarpError::Syntax(error.to_string()))?;
        let program = pairs
            .next()
            .ok_or_else(|| LarpError::Syntax("No program found".to_string()))?;
        self.eval_program(program).map_err(LarpError::Runtime)
    }
    pub fn take_output(&mut self) -> Vec<String> {
        std::mem::take(&mut self.output)
    }
    fn eval_program(&mut self, program: Pair<Rule>) -> Result<(), RuntimeError> {
        for statement in program.into_inner() {
            if statement.as_rule() == Rule::EOI {
                continue;
            }
            self.eval_statement(statement)?;
        }
        Ok(())
    }
    fn eval_statement(&mut self, statement: Pair<Rule>) -> Result<(), RuntimeError> {
        match statement.as_rule() {
            Rule::variable_decl => self.eval_declaration(statement, false),
            Rule::constant_decl => self.eval_declaration(statement, true),
            Rule::assignment => self.eval_assignment(statement),
            Rule::print_stmt => self.eval_print(statement),
            rule => Err(RuntimeError::UnexpectedToken(rule)),
        }
    }
    fn eval_declaration(
        &mut self,
        statement: Pair<Rule>,
        is_constant: bool,
    ) -> Result<(), RuntimeError> {
        let mut inner = statement.into_inner();
        let name = inner
            .next()
            .ok_or(RuntimeError::MalformedStatement)?
            .as_str()
            .to_string();
        let value = match inner.next() {
            Some(expression) => self.eval_expression(expression)?,
            None => Value::Null,
        };
        self.environment.define_variable(name, value, is_constant)
    }
    fn eval_assignment(&mut self, statement: Pair<Rule>) -> Result<(), RuntimeError> {
        let mut inner = statement.into_inner();
        let name = inner
            .next()
            .ok_or(RuntimeError::MalformedStatement)?
            .as_str()
            .to_string();
        let expression = inner.next().ok_or(RuntimeError::MalformedStatement)?;
        let value = self.eval_expression(expression)?;
        self.environment.assign_variable(&name, value)
    }
    fn eval_print(&mut self, statement: Pair<Rule>) -> Result<(), RuntimeError> {
        let expression = statement
            .into_inner()
            .next()
            .ok_or(RuntimeError::MalformedStatement)?;
        let value = self.eval_expression(expression)?;
        self.output.push(value.to_string());
        Ok(())
    }
    fn eval_expression(&self, expression: Pair<Rule>) -> Result<Value, RuntimeError> {
        let mut lookup = |name: &str| self.environment.get_variable(name);
        evaluate_expression_with_lookup(expression, &mut lookup)
    }
}
