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
    VariableMultiAssignment {
        variables: Vec<String>,
        value: ExprAst },
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
    Continue,
    Return { returns: Vec<ExprAst>},
    ClassDeclaration {
        class_name: String,
        members: Vec<String>
    }
}

fn parse_body(pairs: Pairs<Rule>) -> Block {
    let mut statements = Vec::new();

    for pair in pairs {
        match parse_statement(pair) {
            Some(statement) => statements.push(statement),
            None => {}
        }
    }

    statements
}

fn parse_statement(pair: Pair<Rule>) -> Option<Statement> {
    return match pair.as_rule() {
        Rule::variable_assignment => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            if name.contains("."){
               panic!("Variable name cannot contain a period .");
            }
            let value = parse_expression(inner.next().unwrap());

            Some(Statement::VariableAssignment { name, value })
        },
        Rule::variable_multi_assignment => {
            let mut inner = pair.into_inner();
            let mut variables = vec![];
            for part in inner {
                if part.as_rule() != Rule::identifier {
                    let value = parse_expression(part);
                    return Some(Statement::VariableMultiAssignment { variables, value })
                }

                let name = part.as_str().to_string();
                if name.contains("."){
                    panic!("Variable name cannot contain a period .");
                }
                variables.push(name);
            }

            unreachable!("Grammar error: no value for multi assignment")
        }
        Rule::variable_reassignment => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let value = parse_expression(inner.next().unwrap());

            Some(Statement::VariableReassignment { name, value })
        },
        Rule::function_call_statement => {
            let mut inner = pair.into_inner().next().unwrap().into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let mut args = Vec::new();

            for arg in inner {
                args.push(parse_expression(arg));
            }

            Some(Statement::FunctionCall { name, args })
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

            Some(Statement::Conditional { condition, body, else_if_conditions, else_body })
        },
        Rule::while_loop => {
            let mut inner = pair.into_inner().collect::<VecDeque<Pair<Rule>>>();
            let condition = parse_expression(inner.pop_front().expect("Grammar error: no condition"));
            let body = parse_body(inner.pop_front().unwrap().into_inner());

            Some(Statement::WhileLoop { condition, body })
        }
        Rule::break_kw => Some(Statement::Break),
        Rule::continue_kw => Some(Statement::Continue),
        Rule::return_kw => {
            let mut inner = pair.into_inner();
            let mut returns = Vec::new();

            for expr in inner {
                returns.push(parse_expression(expr));
            }

            Some(Statement::Return { returns })
        },
        Rule::function_declaration => {
            // pest grammar: function_declaration = {"proseso" ~ identifier ~ "(" ~ identifier? ~ ("," ~ identifier)* ~ ","* ~ ")" ~ block}
            let mut inner = pair.into_inner().collect::<VecDeque<Pair<Rule>>>();
            let name = inner.pop_front().unwrap().as_str().to_string();
            let mut args = Vec::new();
            loop {
                match inner.pop_front() {
                    Some(arg) => {
                        match arg.as_rule() {
                            Rule::identifier => {
                                args.push(arg.as_str().to_string());
                            },
                            Rule::block => {
                                let body = parse_body(arg.into_inner());
                                return Some(Statement::FunctionDefinition { name, args, body })
                            },
                            _ => { unreachable!("not an identifier or block") }
                        }
                    },
                    None => { unreachable!("no block found") }
                }
            }
        },
        Rule::class_declaration =>  {
            let mut inner = pair.into_inner().collect::<VecDeque<Pair<Rule>>>();
            let class_name = inner.pop_front().unwrap().as_str().to_string();
            let mut members = vec![];
            for pair in inner {
                members.push(pair.as_str().to_string())
            }
            Some(Statement::ClassDeclaration { class_name, members })
        }
        Rule::comment => {
            // println!("{}", pair.as_str());
            None
        }
        _ => unreachable!("not a statement: {:?}", pair)
    }

}

pub fn parse_file_data(file_data: &str) -> Vec<Statement> {
    let file_data = file_data.to_string() + "\n";
    let res = match BareParser::parse(Rule::program, &*file_data){
        Ok(res) => res,
        Err(e) => panic!("Error: {}", e)
    };
    // println!("{:#?}", res);

    let mut statements = Vec::new();

    // println!("{:#?}", res);
    for pair in res {
        // println!("{:?}", pair);
        match pair.as_rule() {
            Rule::EOI => { break }
            _ => {
                match parse_statement(pair) {
                    Some(statement) => {
                        // println!("{:#?}", statement);
                        statements.push(statement)
                    },
                    None => {  }
                }
            }
        }
    }
    statements
}