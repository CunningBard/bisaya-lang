use std::collections::{HashMap, HashSet};
use crate::parser::{BisayaNode, BisayaValue, Expr, MathOp};

pub struct VirtualMachine {
    functions: HashMap<String, Vec<BisayaNode>>,
    builtin: HashSet<String>,

    variable: HashMap<String, BisayaValue>
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            functions: Default::default(),
            builtin: HashSet::from(
                [
                    "ipakita".to_string(),
                ]
            ),
            variable: Default::default(),
        }
    }

    pub fn execute_tree(statements: BisayaNode){
        Self::new().run(statements)
    }

    fn builtin_function(&mut self, function: String, args: Vec<Expr>){
        match &*function {
            "ipakita" => {
                let mut to_print = "".to_string();
                for arg in args {
                    let arg = match self.eval_expr(arg) {
                        BisayaValue::Identifier(variable) => {
                            self.variable.get(&variable).unwrap().clone()
                        }
                        res => { res }
                    };

                    match arg {
                        BisayaValue::Int(value) => {
                            to_print += &*format!("{} ", value)
                        }
                        BisayaValue::Float(value) => {
                            to_print += &*format!("{} ", value)
                        }
                        BisayaValue::String(value) => {
                            to_print += &*format!("{} ", value)
                        }
                        BisayaValue::Boolean(value) => {
                            to_print += &*format!("{} ", value)
                        }
                        _ => unreachable!()
                    }
                }
                to_print.pop();

                println!("{}", to_print)
            }
            _ => unimplemented!()
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> BisayaValue {
        match expr {
            Expr::BinOp { left, op, right } => {
                let left = self.eval_expr(*left);
                let right = self.eval_expr(*right);

                match op {
                    MathOp::Add => { return  left + right }
                    _ => unimplemented!()
                }
            }
            Expr::Constant(value) => {
                return value
            }
        }
        unimplemented!()
    }

    fn execute_statement(&mut self, node: BisayaNode){
        match node {
            BisayaNode::Program { .. } => {
                unreachable!()
            }
            BisayaNode::VariableDeclaration { name, value } => {
                let value = self.eval_expr(value);
                self.variable.insert(name, value);
            }
            BisayaNode::FunctionCall { function_name, args } => {
                if self.builtin.contains(&function_name){
                    self.builtin_function(function_name, args)
                } else {
                    unimplemented!()
                }
            }
        }
    }

    pub fn run(&mut self, tree: BisayaNode){
        match tree {
            BisayaNode::Program { statements } => {
                for statement in statements {
                    self.execute_statement(statement)
                }
            }
            _ => unreachable!()
        }
    }
}
