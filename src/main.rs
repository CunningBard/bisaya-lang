extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Debug)]
enum Term {
    Num(i32),
    String(String),
    Boolean(bool),
    Identifier(String),
    Expression(Box<Expression>)
}

#[derive(Debug)]
enum Expression {
    Add(Term, Term),
    Sub(Term, Term),
    Div(Term, Term),
    Mul(Term, Term),
    Pow(Term, Term)
}

#[derive(Debug)]
enum BisayaValue {
    None
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct BisayaParser;

impl BisayaParser {
    fn parse_file(file: &str) -> Result<BisayaValue, Error<Rule>>{
        let file = Self::parse(Rule::whole_program, file)?.next().unwrap();
        println!("{:?}", file);
        Ok(BisayaValue::None)
    }
}

fn main() {
    let result = BisayaParser::parse_file("test.bis");
    println!("{:?}", result)

}
