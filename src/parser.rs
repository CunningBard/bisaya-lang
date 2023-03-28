use std::collections::VecDeque;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use crate::eval_parser::{ExprAst, parse_expression};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct BareParser;

pub type Block = Vec<Statement>;

#[derive(Debug, Clone)]
pub enum Statement {
    VariableAssignment {
        name: String,
        value: ExprAst
    },
    VariableReassignment {
        name: String,
        value: ExprAst
    },
    FunctionCall {
        name: String,
        args: Vec<ExprAst>
    },
    FunctionDefinition {
        name: String,
        args: Vec<String>,
        body: Block
    },
    Conditional {
        condition: ExprAst,
        body: Block,
        else_if_conditions: Vec<(ExprAst, Block)>,
        else_body: Option<Block>
    },
    ForLoop {
        var_name: String,
        start: ExprAst,
        end: ExprAst,
        body: Block
    },
    WhileLoop {
        condition: ExprAst,
        body: Block
    },
    Break,
    Continue
}

fn parse_body(pairs: Pairs<Rule>) -> Block {
    let mut statements = Vec::new();

    for pair in pairs {
        statements.push(parse_statement(pair));
    }

    statements
}

fn parse_statement(pair: Pair<Rule>) -> Statement {
    return match pair.as_rule() {
        Rule::variable_assignment => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let value = parse_expression(inner.next().unwrap());

            Statement::VariableAssignment { name, value }
        },
        Rule::variable_reassignment => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let value = parse_expression(inner.next().unwrap());

            Statement::VariableReassignment { name, value }
        },
        Rule::function_call_statement => {
            let mut inner = pair.into_inner().next().unwrap().into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let mut args = Vec::new();

            for arg in inner {
                args.push(parse_expression(arg));
            }

            Statement::FunctionCall { name, args }
        },
        Rule::conditional => {
            let mut inner = pair.into_inner().collect::<VecDeque<Pair<Rule>>>();
            let mut if_statement = inner.pop_front().unwrap().into_inner().collect::<VecDeque<Pair<Rule>>>();
            let condition = parse_expression(if_statement.pop_front().expect("Grammar error: no condition"));
            let mut body = parse_body(if_statement.pop_front().unwrap().into_inner());


            let mut else_if_conditions = vec![];
            let mut else_body = None;

            for statement in inner {
                match statement.as_rule() {
                    Rule::conditional_else_if => {
                        let mut else_if_statement = statement.into_inner().collect::<VecDeque<Pair<Rule>>>();
                        let condition = parse_expression(else_if_statement.pop_front().expect("Grammar error: no condition"));
                        let mut body = parse_body(else_if_statement.pop_front().unwrap().into_inner());

                        else_if_conditions.push((condition, body));
                    },
                    Rule::conditional_else => {
                        let mut else_statement = statement.into_inner().collect::<VecDeque<Pair<Rule>>>();
                        let mut body = parse_body(else_statement.pop_front().unwrap().into_inner());

                        else_body = Some(body);
                        break
                    },
                    _ => { unreachable!("not an else if or else statement") }
                }
            }

            Statement::Conditional { condition, body, else_if_conditions, else_body }
        },
        Rule::while_loop => {
            let mut inner = pair.into_inner().collect::<VecDeque<Pair<Rule>>>();
            let condition = parse_expression(inner.pop_front().expect("Grammar error: no condition"));
            let body = parse_body(inner.pop_front().unwrap().into_inner());

            Statement::WhileLoop { condition, body }
        }
        Rule::break_kw => Statement::Break,
        Rule::continue_kw => Statement::Continue,
        _ => unreachable!("not a statement: {:?}", pair)
    }

}

pub fn parse_file_data(file_data: &str) -> Vec<Statement> {
    let file_data = file_data.to_string() + "\n";
    let res = match BareParser::parse(Rule::program, &*file_data){
        Ok(res) => res,
        Err(e) => panic!("Error: {}", e)
    };

    let mut statements = Vec::new();


    for pair in res {
        match pair.as_rule() {
            Rule::EOI => { break }
            _ => {}
        }

        statements.push(parse_statement(pair));
    }
    statements
}