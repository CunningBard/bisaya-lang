use std::collections::VecDeque;
use std::fs;
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;


#[derive(Debug)]
pub enum Factor {
    Int(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Identifier(String),
}

#[derive(Debug)]
pub enum Term {
    Parenthesis(Box<Expression>),
    Div(Factor, Factor),
    Mul(Factor, Factor),
    Solo(Factor),
}


#[derive(Debug)]
pub enum Expression {
    Add(Term, Term),
    Sub(Term, Term),
    Solo(Term),

}

#[derive(Debug)]
pub enum BisayaValue {
    Program {
        statements: Vec<BisayaValue>
    },
    Variable {
        name: String,
        value: Expression
    },
    None
}

// IntermediateForOperatorPrecedence
enum IFOP {
    Factor(Factor),
    Parenthesis(Vec<IFOP>),
    Add,
    Sub,
    Mul,
    Div,

    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct BisayaParser;

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
        for i in pair.into_inner(){
            println!("{}", i)
        }
        unimplemented!();
        match pair.as_rule() {
            Rule::expr => {
                let mut parts: VecDeque<Pair<Rule>> = pair.into_inner().collect();
                let left = Self::parse_term(parts.pop_front().unwrap());
                if parts.len() == 0 {
                    Expression::Solo(left)
                } else {
                    let op = parts.pop_front().unwrap().as_rule();
                    let right = Self::parse_term(parts.pop_front().unwrap());
                    match op {
                        // Rule::add => {
                        //     intermediate.push(IFOP::Add);
                        //     unimplemented!()
                        // }
                        // Rule::sub => {
                        //     Expression::Sub(left, right)
                        // }
                        // Rule::mul => {
                        //     Expression::Mul(left, right)
                        // }
                        // Rule::div => {
                        //     Expression::Div(left, right)
                        // }
                        _ => { unreachable!() }
                    }
                }
            }
            _ => {
                unreachable!()
            }
        }
    }
    fn parse_term(pair: Pair<Rule>) -> Term {
        match pair.as_rule() {
            Rule::factor => {
                Self::parse_factor(pair.into_inner().next().unwrap())
            }
            _ => {
                unreachable!()
            }
        }
    }

    fn parse_factor(pair: Pair<Rule>) -> Term {
        match pair.as_rule() {
            // Rule::num => {
            //     let num_str = pair.as_span().as_str().to_string();
            //     if num_str.contains("."){
            //         Term::Float(num_str.parse::<f32>().unwrap())
            //     } else {
            //         Term::Int(num_str.parse::<i32>().unwrap())
            //     }
            // }
            _ => {
                println!("{}", pair);
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
    pub fn parse_source(source: &str) -> Result<BisayaValue, Error<Rule>>{
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
        Ok(
            BisayaValue::Program {
                statements: remaining
            }
        )

    }
    pub fn parse_file(file: &str) -> Result<BisayaValue, Error<Rule>>{
        let file_data = fs::read_to_string(file).unwrap();
        Self::parse_source(&*file_data)

    }
}