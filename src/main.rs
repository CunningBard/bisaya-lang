mod parser;
mod eval_parser;
mod node_runner;
mod virtual_machine;
mod instruction_compiler;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::VecDeque;
use std::fs;
use std::process::exit;
use pest::Parser;
use crate::parser::{Rule, BareParser, parse_file_data, Statement};
use crate::eval_parser::{ExprAst, parse_expressions};
use crate::node_runner::NodeRunner;
use crate::virtual_machine::{Instruction, IntValue, ObjectCreator, Value, ValueType, VirtualMachine};

fn main() {
    let mut debug_mode = false;
    let mut args: VecDeque<String> = std::env::args().into_iter().collect();
    let _executable_path = args.pop_front().expect("impossible");
    let file_name = args.pop_front().expect("no file name given");
    if file_name == "--help" || file_name == "-h" {
        println!("Usage: {} <file> [--debug]", _executable_path);
        return;
    }
    for arg in args {
        match &*arg {
            "--debug" | "-d" => debug_mode = true,
            "--help"  | "-h"=> { println!("Usage: {} <file> [--debug]", _executable_path); exit(0) },
            _ => unimplemented!("{} has not been implemented or its invalid", arg)
        }
    }
    let file_contents = fs::read_to_string(file_name).expect("couldnt read file");
    let statements = parse_file_data(&file_contents);
    let (instructions, function_locations, class_creators) =  instruction_compiler::compile(statements.clone());

    if debug_mode {
        println!("{:?}", file_contents);
        println!("------ Parsed Statements ----------");
        println!("{:#?}", statements);
        println!("------ Compiled Instruction ----------");
        for instruction in &instructions {
            println!("{:?}", instruction)
        }
        println!("------ Virtual Machine Output ----------");
    }

    let mut vm = VirtualMachine::new(instructions, function_locations, class_creators);
    vm.run();
}