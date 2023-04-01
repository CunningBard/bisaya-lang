mod parser;
mod eval_parser;
mod node_runner;
mod virtual_machine;
mod instruction_compiler;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use pest::Parser;
use crate::parser::{Rule, BareParser, parse_file_data, Statement};
use crate::eval_parser::{ExprAst, parse_expressions};
use crate::node_runner::NodeRunner;
use crate::virtual_machine::{Instruction, IntValue, StructCreator, Value, ValueType, VirtualMachine};

fn main() {
    let file_contents = fs::read_to_string("test.txt").expect("couldnt read file");
    let statements = parse_file_data(&file_contents);
    println!("------ Parsed Statements ----------");
    println!("{:#?}", statements);
    println!("------ Compiled Instruction ----------");
    let instructions =  instruction_compiler::compile(statements);
    for instruction in &instructions {
        println!("{:?}", instruction)
    }
    println!("------ Virtual Machine Output ----------");
    let mut vm = VirtualMachine::new(instructions);
    vm.run();
    // println!("{:#?}", vm.get_heap())
}