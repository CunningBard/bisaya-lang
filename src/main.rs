extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::VecDeque;
use std::fs;
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Debug)]
enum Term {
    Int(i32),
    Float(f32),
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
    Pow(Term, Term),
    Solo(Term)
}

#[derive(Debug)]
enum BisayaValue {
    Program {
        statements: Vec<BisayaValue>
    },
    Variable {
        name: String,
        value: Expression
    },
    None
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct BisayaParser;

impl BisayaParser {
    fn parse_value_str(pair: Pair<Rule>) -> String {
        match pair.as_rule() {
            Rule::identifier => {
                pair.as_span().as_str().to_string()
            }
            _ => {
                unreachable!()
            }
        }
    }
    fn parse_expr(pair: Pair<Rule>) -> Expression {
        match pair.as_rule() {
            Rule::expr => {
                let mut parts: VecDeque<Pair<Rule>> = pair.into_inner().collect();
                let left = Self::parse_term(parts.pop_front().unwrap());
                if parts.len() == 0 {
                    Expression::Solo(left)
                } else {
                    unimplemented!()
                }
            }
            _ => {
                unreachable!()
            }
        }
    }
    fn parse_term(pair: Pair<Rule>) -> Term {
        match pair.as_rule() {
            Rule::num => {
                let num_str = pair.as_span().as_str().to_string();
                if num_str.contains("."){
                    Term::Float(num_str.parse::<f32>().unwrap())
                } else {
                    Term::Int(num_str.parse::<i32>().unwrap())
                }
            }
            _ => {
                unreachable!()
            }
        }
    }
    fn parse_value(pair: Pair<Rule>) -> Option<BisayaValue> {
        match pair.as_rule() {
            Rule::variable => {
                let mut parts: VecDeque<Pair<Rule>> = pair.into_inner().collect();
                let name = Self::parse_value_str(parts.pop_front().unwrap());
                let value = Self::parse_expr(parts.pop_front().unwrap());
                Some(
                    BisayaValue::Variable
                    {
                        name,
                        value
                    }
                )
            }
            Rule::EOI => { None }
            _ => {
                println!("-> {}", pair);
                unreachable!()
            }
        }
    }
    fn parse_file(source: &str) -> Result<Vec<BisayaValue>, Error<Rule>>{
        let file = Self::parse(Rule::program, source)?;
        let mut remaining = vec![];
        for part in file {
            match Self::parse_value(part) {
                None => {
                    break
                }
                Some(val) => {
                    remaining.push(val)
                }
            }
        }
        Ok(remaining)

    }
}

fn main() {
    let file_data = fs::read_to_string("test.bis").unwrap();
    // println!("{:?}", file_data);
    let result = BisayaParser::parse_file(&*(file_data + "\r\n"));
    match result {
        Ok(bvs) => {
            for bv in bvs {
                println!("{:?}", bv);
            }
        }
        Err(err) => {
            println!("{}", err)
        }
    }

}
