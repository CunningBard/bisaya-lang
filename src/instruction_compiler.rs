use crate::eval_parser::{EvalValue, ExprAst};
use crate::parser::Statement;
use crate::virtual_machine::{BoolValue, FloatValue, Instruction, IntValue, StringValue, Struct, Value, ValueType};

pub struct InstructionCompiler {
    instructions: Vec<Instruction>
}

impl InstructionCompiler {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new()
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
                    _ => { unreachable!() }
                }
            }
            ExprAst::FunctionCall { name, args } => {
                let args_len = args.len();
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

    pub fn run(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            match statement {
                Statement::VariableAssignment { name, value } => {
                    self.instructions.push(Instruction::NewVariable(name.clone(), Struct::new("".to_string(), Value::from_int_val(IntValue::Int32(0)))));
                    self.instructions.append(&mut self.compile_expr_ast(value));
                    self.instructions.push(Instruction::Store(name.clone()));
                }
                Statement::FunctionCall { name, args } => {
                    let args_len = args.len();
                    for arg in args {
                        self.instructions.append(&mut self.compile_expr_ast(arg))
                    }
                    self.instructions.push(Instruction::Push(ValueType::Int(IntValue::Int32(args_len as i32))));
                    self.instructions.push(Instruction::Call(name.clone()))
                }
                _ => { unimplemented!("statement {:?}: not implemented", statement) }
            }
        }
    }
    pub fn compile(vec: Vec<Statement>) -> Vec<Instruction>{
        let mut compiler = InstructionCompiler::new();
        compiler.run(vec);
        compiler.instructions
    }
}

pub fn compile(vec: Vec<Statement>) -> Vec<Instruction>{
    InstructionCompiler::compile(vec)
}