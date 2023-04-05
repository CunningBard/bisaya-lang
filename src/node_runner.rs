use std::collections::{HashMap, VecDeque};
use crate::eval_parser::{ExprAst, EvalValue};
use crate::parser::{Statement, Block};

const LOOP_LIMIT: usize = 100000;


pub struct NodeRunner {
    locals: HashMap<String, EvalValue>,
    locals_tracker: Vec<String>,
    locals_scope: Vec<usize>,

    // globals: HashMap<String, Value>,
}

impl NodeRunner {
    pub fn new() -> NodeRunner {
        Self {
            locals: HashMap::new(),
            locals_tracker: Vec::new(),
            locals_scope: vec![],
        }
    }

    pub fn get_value_type(&self, val: &EvalValue) -> i32 {
        match val {
            EvalValue::Reference { val } => {
                self.get_value_type(self.locals.get(val).ok_or_else(|| {
                    panic!("couldnt find variable {}", val)
                }).unwrap())
            }
            EvalValue::IntegerLiteral { .. } => { 1 }
            EvalValue::Stringliteral { .. } => { 2 }
            EvalValue::FloatLiteral { .. } => { 3 }
            EvalValue::BooleanLiteral { .. } => { 4 }
        }
    }

    pub fn evaluate_expr(&mut self, expr: ExprAst) -> EvalValue {
        match expr {
            ExprAst::Value { val } => {
                match val {
                    EvalValue::Reference { val } => {
                        self.locals.get(&*val).ok_or_else(|| {
                            panic!("couldnt find variable {}", val)
                        }).unwrap().clone()
                    }
                    _ => { val }
                }
            }
            ExprAst::FunctionCall { name, args } => {
                match self.handle_function_call(name, args) {
                    Some(val) => { val }
                    None => { EvalValue::Stringliteral { val: "NONE".to_string() } }
                }
            }
            ExprAst::Addition { lhs, rhs } => {
                let mut lhs = self.evaluate_expr(*lhs);
                let mut rhs = self.evaluate_expr(*rhs);

                let lhs_type = self.get_value_type(&lhs);
                let rhs_type = self.get_value_type(&rhs);

                return match (lhs_type, rhs_type) {
                    (1, 1) => { EvalValue::IntegerLiteral { val: (lhs.get_int() + rhs.get_int()).to_string() } }
                    (2, 2) => { EvalValue::Stringliteral { val: lhs.get_string() + &rhs.get_string() } }
                    (3, 3) => { EvalValue::FloatLiteral { val: (lhs.get_float() + rhs.get_float()).to_string() } }
                    (1, 3) => { EvalValue::FloatLiteral { val: (lhs.get_int() as f32 + rhs.get_float()).to_string() } }
                    (3, 1) => { EvalValue::FloatLiteral { val: (lhs.get_float() + rhs.get_int() as f32).to_string() } }
                    _ => unreachable!("invalid types for addition")
                }
            }
            ExprAst::Subtraction { lhs, rhs } => {
                let mut lhs = self.evaluate_expr(*lhs);
                let mut rhs = self.evaluate_expr(*rhs);

                let lhs_type = self.get_value_type(&lhs);
                let rhs_type = self.get_value_type(&rhs);

                return match (lhs_type, rhs_type) {
                    (1, 1) => { EvalValue::IntegerLiteral { val: (lhs.get_int() - rhs.get_int()).to_string() } }
                    (3, 3) => { EvalValue::FloatLiteral { val: (lhs.get_float() - rhs.get_float()).to_string() } }
                    (1, 3) => { EvalValue::FloatLiteral { val: (lhs.get_int() as f32 - rhs.get_float()).to_string() } }
                    (3, 1) => { EvalValue::FloatLiteral { val: (lhs.get_float() - rhs.get_int() as f32).to_string() } }
                    _ => unreachable!("invalid types for subtraction")
                }
            }
            ExprAst::Division { lhs, rhs } => {
                let mut lhs = self.evaluate_expr(*lhs);
                let mut rhs = self.evaluate_expr(*rhs);

                let lhs_type = self.get_value_type(&lhs);
                let rhs_type = self.get_value_type(&rhs);

                return match (lhs_type, rhs_type) {
                    (1, 1) => { EvalValue::FloatLiteral { val: (lhs.get_int() as f32 / rhs.get_int() as f32).to_string() } }
                    (3, 3) => { EvalValue::FloatLiteral { val: (lhs.get_float() / rhs.get_float()).to_string() } }
                    (1, 3) => { EvalValue::FloatLiteral { val: (lhs.get_int() as f32 / rhs.get_float()).to_string() } }
                    (3, 1) => { EvalValue::FloatLiteral { val: (lhs.get_float() / rhs.get_int() as f32).to_string() } }
                    _ => unreachable!("invalid types for subtraction")
                }
            }
            ExprAst::Multiplication { lhs, rhs } => {
                let mut lhs = self.evaluate_expr(*lhs);
                let mut rhs = self.evaluate_expr(*rhs);

                let lhs_type = self.get_value_type(&lhs);
                let rhs_type = self.get_value_type(&rhs);

                return match (lhs_type, rhs_type) {
                    (1, 1) => { EvalValue::IntegerLiteral { val: (lhs.get_int() * rhs.get_int()).to_string() } }
                    (3, 3) => { EvalValue::FloatLiteral { val: (lhs.get_float() * rhs.get_float()).to_string() } }
                    (1, 3) => { EvalValue::FloatLiteral { val: (lhs.get_int() as f32 * rhs.get_float()).to_string() } }
                    (3, 1) => { EvalValue::FloatLiteral { val: (lhs.get_float() * rhs.get_int() as f32).to_string() } }
                    _ => unreachable!("invalid types for subtraction")
                }
            }
            ExprAst::Eq { lhs, rhs } => {
                let mut lhs = self.evaluate_expr(*lhs);
                let mut rhs = self.evaluate_expr(*rhs);

                let lhs_type = self.get_value_type(&lhs);
                let rhs_type = self.get_value_type(&rhs);

                return match (lhs_type, rhs_type) {
                    (1, 1) => { EvalValue::BooleanLiteral { val: lhs.get_int() == rhs.get_int() } }
                    (2, 2) => { EvalValue::BooleanLiteral { val: lhs.get_string() == rhs.get_string() } }
                    (3, 3) => { EvalValue::BooleanLiteral { val: lhs.get_float() == rhs.get_float() } }
                    (1, 3) => { EvalValue::BooleanLiteral { val: lhs.get_int() as f32 == rhs.get_float() } }
                    (3, 1) => { EvalValue::BooleanLiteral { val: lhs.get_float() == rhs.get_int() as f32 } }
                    _ => unreachable!("invalid types for subtraction")
                }
            }
            ExprAst::Neq { lhs, rhs } => {
                let mut lhs = self.evaluate_expr(*lhs);
                let mut rhs = self.evaluate_expr(*rhs);

                let lhs_type = self.get_value_type(&lhs);
                let rhs_type = self.get_value_type(&rhs);

                return match (lhs_type, rhs_type) {
                    (1, 1) => { EvalValue::BooleanLiteral { val: lhs.get_int() != rhs.get_int() } }
                    (2, 2) => { EvalValue::BooleanLiteral { val: lhs.get_string() != rhs.get_string() } }
                    (3, 3) => { EvalValue::BooleanLiteral { val: lhs.get_float() != rhs.get_float() } }
                    (1, 3) => { EvalValue::BooleanLiteral { val: lhs.get_int() as f32 != rhs.get_float() } }
                    (3, 1) => { EvalValue::BooleanLiteral { val: lhs.get_float() != rhs.get_int() as f32 } }
                    _ => unreachable!("invalid types for subtraction")
                }
            }
            ExprAst::GtEq { lhs, rhs } => {
                let mut lhs = self.evaluate_expr(*lhs);
                let mut rhs = self.evaluate_expr(*rhs);

                let lhs_type = self.get_value_type(&lhs);
                let rhs_type = self.get_value_type(&rhs);

                return match (lhs_type, rhs_type) {
                    (1, 1) => { EvalValue::BooleanLiteral { val: lhs.get_int() >= rhs.get_int() } }
                    (3, 3) => { EvalValue::BooleanLiteral { val: lhs.get_float() >= rhs.get_float() } }
                    (1, 3) => { EvalValue::BooleanLiteral { val: lhs.get_int() as f32 >= rhs.get_float() } }
                    (3, 1) => { EvalValue::BooleanLiteral { val: lhs.get_float() >= rhs.get_int() as f32 } }
                    _ => unreachable!("invalid types for subtraction")
                }
            }
            ExprAst::LtEq { lhs, rhs } => {
                let mut lhs = self.evaluate_expr(*lhs);
                let mut rhs = self.evaluate_expr(*rhs);

                let lhs_type = self.get_value_type(&lhs);
                let rhs_type = self.get_value_type(&rhs);

                return match (lhs_type, rhs_type) {
                    (1, 1) => { EvalValue::BooleanLiteral { val: lhs.get_int() <= rhs.get_int() } }
                    (3, 3) => { EvalValue::BooleanLiteral { val: lhs.get_float() <= rhs.get_float() } }
                    (1, 3) => { EvalValue::BooleanLiteral { val: lhs.get_int() as f32 <= rhs.get_float() } }
                    (3, 1) => { EvalValue::BooleanLiteral { val: lhs.get_float() <= rhs.get_int() as f32 } }
                    _ => unreachable!("invalid types for subtraction")
                }
            }
            ExprAst::Gt { lhs, rhs } => {
                let mut lhs = self.evaluate_expr(*lhs);
                let mut rhs = self.evaluate_expr(*rhs);

                let lhs_type = self.get_value_type(&lhs);
                let rhs_type = self.get_value_type(&rhs);

                return match (lhs_type, rhs_type) {
                    (1, 1) => { EvalValue::BooleanLiteral { val: lhs.get_int() > rhs.get_int() } }
                    (3, 3) => { EvalValue::BooleanLiteral { val: lhs.get_float() > rhs.get_float() } }
                    (1, 3) => { EvalValue::BooleanLiteral { val: lhs.get_int() as f32 > rhs.get_float() } }
                    (3, 1) => { EvalValue::BooleanLiteral { val: lhs.get_float() > rhs.get_int() as f32 } }
                    _ => unreachable!("invalid types for subtraction")
                }
            }
            ExprAst::Lt { lhs, rhs } => {
                let mut lhs = self.evaluate_expr(*lhs);
                let mut rhs = self.evaluate_expr(*rhs);

                let lhs_type = self.get_value_type(&lhs);
                let rhs_type = self.get_value_type(&rhs);

                return match (lhs_type, rhs_type) {
                    (1, 1) => { EvalValue::BooleanLiteral { val: lhs.get_int() < rhs.get_int() } }
                    (3, 3) => { EvalValue::BooleanLiteral { val: lhs.get_float() < rhs.get_float() } }
                    (1, 3) => { EvalValue::BooleanLiteral { val: (lhs.get_int() as f32) < rhs.get_float() } }
                    (3, 1) => { EvalValue::BooleanLiteral { val: lhs.get_float() < rhs.get_int() as f32 } }
                    _ => unreachable!("invalid types for subtraction")
                }
            }
        }
    }

    fn handle_function_call(&mut self, name: String, mut args: Vec<ExprAst>) -> Option<EvalValue> {
        if ["print", "println", "format"].contains(&name.as_str()) {
            self.call_builtin_function(name, args.into_iter().collect())
        } else {
            unimplemented!()
        }
    }

    fn call_builtin_function(&mut self, name: String, mut args: VecDeque<ExprAst>) -> Option<EvalValue> {
        match name.as_str() {
            "print" => {
                let mut args = args.into_iter().map(|arg| self.evaluate_expr(arg)).collect::<Vec<EvalValue>>();
                let mut output = String::new();
                for arg in args {
                    output.push_str(&arg.to_string());
                    output.push(' ')
                }
                output.pop();
                print!("{}", output);
                None
            },
            "println" => {
                let mut args = args.into_iter().map(|arg| self.evaluate_expr(arg)).collect::<Vec<EvalValue>>();
                let mut output = String::new();
                for arg in args {
                    output.push_str(&arg.to_string());
                    output.push(' ')
                }
                output.pop();
                println!("{}", output);
                None
            },
            "format" => {
                if args.len() < 1 {
                    panic!("format function takes 1 or more arguments, {} given", args.len())
                }

                let mut string = match self.evaluate_expr(args.pop_front().unwrap()) {
                    EvalValue::Stringliteral { val } => val,
                    _ => panic!("format function takes a string as first argument")
                };

                while args.len() > 0 {
                    let replace_with = args.pop_front().unwrap();
                    match self.evaluate_expr(replace_with) {
                        EvalValue::IntegerLiteral { val } |
                        EvalValue::Stringliteral { val } |
                        EvalValue::FloatLiteral { val } => {
                            string = string.replacen("{}", &val.to_string(), 1);
                        }
                        EvalValue::BooleanLiteral { val } => {
                            string = string.replacen("{}", &val.to_string(), 1);
                        }
                        _ => unreachable!("how?")
                    }
                }

                Some(EvalValue::Stringliteral { val: string })
            }
            _ => unimplemented!("builtin function {} is not implemented", name)
        }
    }

    fn execute(&mut self, statement: Statement, from_loop: bool) -> Option<u8>{
        match statement {
            Statement::VariableAssignment { name, value } => {
                let expr = self.evaluate_expr(value);
                self.locals.insert(name.clone(), expr);
                self.locals_tracker.push(name);
                let locals_scope_len = self.locals_scope.len() - 1;
                self.locals_scope[locals_scope_len] += 1;
                // println!("{:#?}", self.locals);
            },
            Statement::VariableReassignment { name, value } => {
                if !self.locals.contains_key(&name){
                    panic!("variable {} is not defined", name)
                }

                let expr = self.evaluate_expr(value);
                self.locals.insert(name.clone(), expr);
            },
            Statement::FunctionCall { name, args } => {
                self.handle_function_call(name, args);
            },
            Statement::FunctionDefinition { name, args, body } => {
                unimplemented!()
            },
            Statement::ForLoop { var_name, start, end, body } => {
                // when implementing, remember a loop might be before it
                unimplemented!()
            },
            Statement::Conditional { condition, body, else_if_conditions, else_body } => {
                let result = match self.evaluate_expr(condition) {
                    EvalValue::BooleanLiteral { val } => { val }
                    _ => unreachable!("invalid condition")
                };

                if result {
                    let res = self.run(body, from_loop);
                    if res.is_some() { return res }
                } else {
                    let mut ran = false;
                    for (condition, body) in else_if_conditions {
                        match self.evaluate_expr(condition) {
                            EvalValue::BooleanLiteral { val } => {
                                if val {
                                    let res = self.run(body, from_loop);
                                    if res.is_some() { return res }

                                    ran = true;
                                    break;
                                }
                            }
                            _ => unreachable!("invalid condition")
                        };
                    }
                    if !ran && else_body.is_some() {
                        self.run(else_body.unwrap(), from_loop);
                    }
                }


            }
            Statement::WhileLoop { condition, body } => {
                while match self.evaluate_expr(condition.clone()) {
                    EvalValue::BooleanLiteral { val } => { val }
                    _ => unreachable!("invalid condition")
                } {
                    let res = self.run(body.clone(), true);
                    if res.is_some() {
                        let res = res.unwrap();
                        if res == 0 {
                            break;
                        } else if res == 1 {
                            continue;
                        } else {
                            unreachable!()
                        }
                    }
                }
            }

            Statement::Break => {
                if from_loop {
                    return Some(0)
                }
                panic!("break outside of loop")
            }
            Statement::Continue => {
                if from_loop {
                    return Some(1)
                }

                panic!("continue outside of loop")
            }
            _ => { unimplemented!("statement not implemented") }
        }
        None
    }

    pub fn run(&mut self, stmts: Block, from_loop: bool) -> Option<u8>{
        self.locals_scope.push(0);
        for statement in stmts {
            let res = self.execute(statement, from_loop);
            if res.is_some() { return  res }
        }

        for _ in 0..self.locals_scope.pop().unwrap() {
            self.locals.remove(&self.locals_tracker.pop().unwrap());
        }
        None
    }
}