mod parser;
mod eval_parser;
mod node_runner;
mod virtual_machine;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use pest::Parser;
use crate::parser::{Rule, BareParser, parse_file_data};
use crate::eval_parser::parse_expressions;
use crate::node_runner::NodeRunner;

fn main() {
    let file_contents = fs::read_to_string("test.txt").expect("couldnt read file");
    let statements = parse_file_data(&file_contents);
    println!("------ Parsed Statements ----------");
    println!("{:#?}", statements);
    let mut vm = NodeRunner::new();
    println!("------ Virtual Machine Output ----------");
    vm.run(statements, false);
}