mod parser;
mod cmd;
mod virtual_machine;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::parser::{BisayaParser, BisayaNode};


fn main() {
    let result = BisayaParser::parse_file("test.bis");
    match result {
        Ok(bvs) => {
            match bvs {
                BisayaNode::Program {
                    statements
                } => {
                    for statement in statements {
                        println!("{:#?}", statement);

                    }
                }
                _ => { unreachable!() }
            }
        }
        Err(err) => {
            println!("{}", err)
        }
    }

}
