use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct JSONParser;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Parsing failed: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
}

pub type Result<T> = std::result::Result<T, ParseError>;

impl JSONParser {
    pub fn parse_json(input: &str) -> Result<pest::iterators::Pairs<Rule>> {
        JSONParser::parse(Rule::JSON, input).map_err(ParseError::from)
    }
}
