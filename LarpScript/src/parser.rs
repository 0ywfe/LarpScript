use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct LarpParser;

pub fn parse_program(source: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    LarpParser::parse(Rule::program, source)
}

pub fn parse_expression(source: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    LarpParser::parse(Rule::expression, source)
}
