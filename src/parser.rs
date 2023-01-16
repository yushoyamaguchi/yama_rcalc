use pest::iterators::Pair;
 use pest::Parser;
 use pest_derive::Parser;
 use tracing::debug;

 #[derive(Parser)]
 #[grammar = "calc.pest"]
 struct CalcParser;

pub fn parse(form1: &str) -> i32{
    CalcParser::parse(Rule::form,form1);
    0
}