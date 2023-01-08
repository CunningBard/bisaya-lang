mod parser;
mod cmd;
mod virtual_machine;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::parser::{BisayaParser, BisayaNode};
use crate::virtual_machine::VirtualMachine;


fn main() {
    let result = BisayaParser::parse_file("test.bis");
    match result {
        Ok(bvs) => {
            VirtualMachine::execute_tree(bvs);
        }
        Err(err) => {
            println!("{}", err)
        }
    }

}
