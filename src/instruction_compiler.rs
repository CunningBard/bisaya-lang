use std::collections::HashMap;
use crate::eval_parser::{EvalValue, ExprAst};
use crate::parser::{Block, Statement};
use crate::virtual_machine::{BoolValue, FloatValue, Instruction, IntValue, StringValue, Object, ObjectCreator, Value, ValueType};

#[derive(Debug, Clone)]
enum Translation {
    Instruction(Instruction),
    Label(usize),
    Jump(u8, usize),
}

pub struct InstructionCompiler {
    instructions: Vec<Instruction>,
    function_translations: Vec<Translation>,
    functions_label_locations: Vec<(String, usize)>,
    functions_locations: HashMap<String, usize>,
    labels: HashMap<usize, usize>,
    label_count: usize,
    class_creators: HashMap<String, ObjectCreator>,
    class_details: HashMap<String, (usize, Vec<String>)>
}

impl InstructionCompiler {
    pub fn new() -> Self {
        Self {
            instructions: vec![],
            function_translations: vec![],
            functions_label_locations: vec![],
            functions_locations: HashMap::new(),
            labels: HashMap::new(),
            label_count: 0,
            class_creators: HashMap::new(),
            class_details: HashMap::new()
        }
    }

    fn compile_expr_ast(&self, expr: ExprAst) -> Vec<Instruction> {
        let mut inst = vec![];
        match expr {
            ExprAst::Value { val } => {
                match val {
                    EvalValue::IntegerLiteral { val } => {
                        if val.parse::<i8>().is_ok() {
                            inst.push(Instruction::Push(ValueType::Int(IntValue::Int8(val.parse::<i8>().unwrap()))))
                        } else if val.parse::<i16>().is_ok() {
                            inst.push(Instruction::Push(ValueType::Int(IntValue::Int16(val.parse::<i16>().unwrap()))))
                        } else if val.parse::<i32>().is_ok() {
                            inst.push(Instruction::Push(ValueType::Int(IntValue::Int32(val.parse::<i32>().unwrap()))))
                        } else if val.parse::<i64>().is_ok() {
                            inst.push(Instruction::Push(ValueType::Int(IntValue::Int64(val.parse::<i64>().unwrap()))))
                        } else {
                            unreachable!("too big of an int, I suggest using floats instead")
                        }
                    }
                    EvalValue::FloatLiteral { val } => {
                        if val.parse::<f32>().is_ok() {
                            inst.push(Instruction::Push(ValueType::Float(FloatValue::Float32(val.parse::<f32>().unwrap()))))
                        } else if val.parse::<f64>().is_ok() {
                            inst.push(Instruction::Push(ValueType::Float(FloatValue::Float64(val.parse::<f64>().unwrap()))))
                        } else {
                            unreachable!("too big of a float, unfortunately no fix yet, unless f128 is implemented")
                        }
                    }
                    EvalValue::Stringliteral { val} => {
                        inst.push(Instruction::Push(ValueType::String(StringValue::new(val.clone()))))
                    }
                    EvalValue::BooleanLiteral { val } => {
                        inst.push(Instruction::Push(ValueType::Bool(BoolValue::new(val))))
                    }
                    EvalValue::Reference { val } => {
                        inst.push(Instruction::Load(val.clone()))
                    }
                    EvalValue::List { val } => {
                        inst.push(Instruction::Push(ValueType::Vector(vec![])));
                        for value in val {
                            inst.append(&mut self.compile_expr_ast(value));
                            inst.push(Instruction::Push(ValueType::Int(IntValue::Int32(2))));
                            inst.push(Instruction::Call("push".to_string()));
                        }
                    }
                    _ => { unreachable!() }
                }
            }
            ExprAst::FunctionCall { name, args } => {
                let args_len = args.len();
                if self.class_details.contains_key(&*name){

                }
                for arg in args {
                    inst.append(&mut self.compile_expr_ast(arg))
                }
                inst.push(Instruction::Push(ValueType::Int(IntValue::Int32(args_len as i32))));
                inst.push(Instruction::Call(name.clone()))
            }
            ExprAst::Addition { lhs, rhs } => {
                inst.append(&mut self.compile_expr_ast(*lhs));
                inst.append(&mut self.compile_expr_ast(*rhs));
                inst.push(Instruction::Add)
            }
            ExprAst::Subtraction { lhs, rhs } => {
                inst.append(&mut self.compile_expr_ast(*lhs));
                inst.append(&mut self.compile_expr_ast(*rhs));
                inst.push(Instruction::Sub)
            }
            ExprAst::Division { lhs, rhs } => {
                inst.append(&mut self.compile_expr_ast(*lhs));
                inst.append(&mut self.compile_expr_ast(*rhs));
                inst.push(Instruction::Div)
            }
            ExprAst::Multiplication { lhs, rhs } => {
                inst.append(&mut self.compile_expr_ast(*lhs));
                inst.append(&mut self.compile_expr_ast(*rhs));
                inst.push(Instruction::Mul)
            }
            ExprAst::Eq { lhs, rhs } => {
                inst.append(&mut self.compile_expr_ast(*lhs));
                inst.append(&mut self.compile_expr_ast(*rhs));
                inst.push(Instruction::Eq)
            }
            ExprAst::Neq { lhs, rhs } => {
                inst.append(&mut self.compile_expr_ast(*lhs));
                inst.append(&mut self.compile_expr_ast(*rhs));
                inst.push(Instruction::Neq)
            }
            ExprAst::GtEq { lhs, rhs } => {
                inst.append(&mut self.compile_expr_ast(*lhs));
                inst.append(&mut self.compile_expr_ast(*rhs));
                inst.push(Instruction::GtEq)
            }
            ExprAst::LtEq { lhs, rhs } => {
                inst.append(&mut self.compile_expr_ast(*lhs));
                inst.append(&mut self.compile_expr_ast(*rhs));
                inst.push(Instruction::LtEq)
            }
            ExprAst::Gt { lhs, rhs } => {
                inst.append(&mut self.compile_expr_ast(*lhs));
                inst.append(&mut self.compile_expr_ast(*rhs));
                inst.push(Instruction::Gt)
            }
            ExprAst::Lt { lhs, rhs } => {
                inst.append(&mut self.compile_expr_ast(*lhs));
                inst.append(&mut self.compile_expr_ast(*rhs));
                inst.push(Instruction::Lt)
            }
        }
        inst
    }

    fn compile_block(&mut self, statements: Block, start_label: Option<usize>, end_label: Option<usize>, in_a_function: bool) -> Vec<Translation> {
        let mut translations = vec![];
        let mut assignments = vec![];
        for statement in statements {
            match statement {
                Statement::VariableAssignment { name, value } => {
                    assignments.push(name.clone());
                    translations.append(&mut self.compile_expr_ast(value).iter().map(|x| Translation::Instruction(x.clone())).collect());
                    translations.push(Translation::Instruction(Instruction::NewVariable(name.clone())));
                }
                Statement::VariableReassignment { name, value} => {
                    translations.append(&mut self.compile_expr_ast(value).iter().map(|x| Translation::Instruction(x.clone())).collect());
                    translations.push(Translation::Instruction(Instruction::Store(name.clone())));
                }
                Statement::FunctionCall { name, args } => {
                    let args_len = args.len();
                    for arg in args {
                        translations.append(&mut self.compile_expr_ast(arg).iter().map(|x| Translation::Instruction(x.clone())).collect())
                    }
                    translations.push(Translation::Instruction(Instruction::Push(ValueType::Int(IntValue::Int32(args_len as i32)))));
                    translations.push(Translation::Instruction(Instruction::Call(name.clone())))
                }
                Statement::Conditional { condition, body, else_if_conditions, else_body } => {
                    translations.append(&mut self.compile_expr_ast(condition).iter().map(|x| Translation::Instruction(x.clone())).collect());
                    let end_label = self.label_count;
                    self.label_count += 1;
                    let body_end_label = self.label_count;
                    self.label_count += 1;
                    translations.push(Translation::Jump(2, body_end_label));
                    for statement in self.compile_block(body, None, None, in_a_function){
                        translations.push(statement)
                    }
                    translations.push(Translation::Jump(0, end_label));
                    translations.push(Translation::Label(body_end_label));

                    for (condition, body) in else_if_conditions {
                        translations.append(&mut self.compile_expr_ast(condition).iter().map(|x| Translation::Instruction(x.clone())).collect());
                        let body_end_label = self.label_count;
                        self.label_count += 1;
                        translations.push(Translation::Jump(2, body_end_label));
                        for statement in self.compile_block(body, None, None, in_a_function){
                            translations.push(statement)
                        }
                        translations.push(Translation::Jump(0, end_label));
                        translations.push(Translation::Label(body_end_label));
                    }
                    match else_body {
                        Some(body) => {
                            for statement in self.compile_block(body, None, None, false){
                                translations.push(statement)
                            }
                        }
                        None => {}
                    }
                    translations.push(Translation::Label(end_label));
                }
                Statement::WhileLoop { condition, body } => {
                    let start_label = self.label_count;
                    self.label_count += 1;
                    let end_label = self.label_count;
                    self.label_count += 1;
                    translations.push(Translation::Label(start_label));
                    translations.append(&mut self.compile_expr_ast(condition).iter().map(|x| Translation::Instruction(x.clone())).collect());
                    translations.push(Translation::Jump(2, end_label));
                    for statement in self.compile_block(body, Some(start_label), Some(end_label), in_a_function){
                        translations.push(statement)
                    }
                    translations.push(Translation::Jump(0, start_label));
                    translations.push(Translation::Label(end_label));
                }
                Statement::Continue => {
                    match start_label {
                        Some(label) => { translations.push(Translation::Jump(0, label)) }
                        None => { panic!("continue statement outside of loop") }
                    }
                }
                Statement::Break => {
                    match end_label {
                        Some(label) => { translations.push(Translation::Jump(0, label)) }
                        None => { panic!("break statement outside of loop") }
                    }
                }
                Statement::FunctionDefinition { name, args, mut body } => {
                    if self.functions_locations.contains_key(&*name) {
                        panic!("Function {} already defined", name)
                    } else if self.class_details.contains_key(&*name) {
                        panic!("Function {} already defined as a class", name)
                    }

                    let mut local_assignments = vec![];
                    let end_label = self.label_count;
                    self.label_count += 1;
                    translations.push(Translation::Jump(0, end_label));
                    translations.push(Translation::Label(self.label_count));
                    self.functions_label_locations.push((name.clone(), self.label_count));
                    self.label_count += 1;
                    translations.push(Translation::Instruction(Instruction::Push(ValueType::Int(IntValue::Int32(args.len() as i32)))));
                    translations.push(Translation::Instruction(Instruction::Eq));
                    translations.push(Translation::Instruction(Instruction::Push(ValueType::String(StringValue::new(format!("Error: Expected {} arguments", args.len()))))));
                    translations.push(Translation::Instruction(Instruction::Push(ValueType::Int(IntValue::Int32(2)))));
                    translations.push(Translation::Instruction(Instruction::Call("assert".to_string())));

                    let mut args = args;
                    args.reverse();
                    for arg in args {
                        translations.push(Translation::Instruction(Instruction::NewVariable(arg.clone())));
                        local_assignments.push(arg.clone());
                    }
                    let mut compiled_body = self.compile_block(body.clone(), None, None, true);

                    let mut add_at = vec![];
                    for (i, instruction) in compiled_body.clone().iter().enumerate() {
                        match instruction {
                            Translation::Instruction(Instruction::Return) => {
                                add_at.push(i);
                            }
                            _ => {}
                        }
                    }
                    add_at.reverse();
                    for assignment in local_assignments {
                        for location in &add_at {
                            compiled_body.insert(*location, Translation::Instruction(Instruction::Delete(assignment.clone())))
                        }
                    }
                    translations.append(&mut compiled_body);
                    translations.push(Translation::Label(end_label));
                }
                Statement::Return { returns } => {
                    if !in_a_function {
                        panic!("return statement outside of function")
                    }
                    for return_value in returns {
                        translations.append(&mut self.compile_expr_ast(return_value).iter().map(|x| Translation::Instruction(x.clone())).collect());
                    }
                    translations.push(Translation::Instruction(Instruction::Return));
                }
                Statement::ClassDeclaration { class_name, members } => {
                    if self.functions_locations.contains_key(&class_name) {
                        panic!("Class {} already defined as a function", class_name)
                    } else if self.class_details.contains_key(&class_name) {
                        panic!("Class {} already defined", class_name)
                    }

                    let mut creator = ObjectCreator::new(class_name.clone());
                    for member in members.clone() {
                        creator.add_member(member);
                    }

                    self.class_details.insert(class_name.clone(), (members.len(), members));
                    self.class_creators.insert(class_name.clone(), creator);
                }
                Statement::VariableMultiAssignment { variables, value } => {
                    translations.append(&mut self.compile_expr_ast(value).iter().map(|x| Translation::Instruction(x.clone())).collect());
                    for variable in variables {
                        translations.push(Translation::Instruction(Instruction::NewVariable(variable.clone())));
                        assignments.push(variable.clone());
                    }
                }
                _ => { unimplemented!("statement {:?}: not implemented", statement) }
            }
        }
        for assignment in assignments {
            translations.push(Translation::Instruction(Instruction::Delete(assignment)))
        }
        translations
    }
    fn compile_translation(&mut self, translations: Vec<Translation>) -> Vec<Instruction>{
        let mut instructions = vec![];
        let mut last_iterations = vec![];
        for translation in translations {
            // println!("{:?}", translation);
            match translation {
                Translation::Label(label_id) => {
                    self.labels.insert(label_id, last_iterations.len() + 1);
                }
                _ => {
                    last_iterations.push(translation)
                }
            }
        }

        for translation in last_iterations {
            match translation {
                Translation::Instruction(instruction) => {
                    instructions.push(instruction)
                }
                Translation::Jump(jump, label) => {
                    // println!("{:?}", self.labels);
                    let label_line = *self.labels.get(&label).unwrap_or_else(|| panic!("label {} not found", label));
                    match jump {
                        0 => {
                            instructions.push(Instruction::Jump(label_line))
                        }
                        1 => {
                            instructions.push(Instruction::JumpIfTrue(label_line))
                        }
                        2 => {
                            instructions.push(Instruction::JumpIfFalse(label_line))
                        }
                        _ => { unreachable!("rust wtf") }
                    }
                }
                Translation::Label(_) => {
                    unreachable!("rust wtf")
                }
            }
        }
        instructions
    }
    pub fn run(&mut self, statements: Vec<Statement>) {
        let translations = self.compile_block(statements, None, None, false);
        let mut compiled = self.compile_translation(translations);
        self.instructions.append(&mut compiled);
        for label in self.functions_label_locations.clone() {
            let label_line = *self.labels.get(&label.1).unwrap_or_else(|| panic!("label {} not found", label.1));
            self.functions_locations.insert(label.0, label_line );
        }
    }
    pub fn compile(vec: Vec<Statement>) -> (Vec<Instruction>, HashMap<String, usize>, HashMap<String, ObjectCreator>){
        let mut compiler = InstructionCompiler::new();
        compiler.run(vec);
        (compiler.instructions, compiler.functions_locations, compiler.class_creators)
    }
}

pub fn compile(vec: Vec<Statement>) -> (Vec<Instruction>, HashMap<String, usize>, HashMap<String, ObjectCreator>) {
    InstructionCompiler::compile(vec)
}