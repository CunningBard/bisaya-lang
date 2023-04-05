use std::collections::HashMap;
use std::collections::vec_deque::VecDeque;


const BUILTIN_FUNCTIONS: [&str; 8] = [
    "print",
    "println",
    "format",
    "assert",
    "push",
    "pop",
    "read_element",
    "write_element"
];


#[derive(Clone, Debug)]
pub enum IntValue {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
}

#[derive(Clone, Debug)]
enum CharValue {
    Char(char),
}

#[derive(Clone, Debug)]
pub struct  StringValue {
    value: String
}

#[derive(Clone, Debug)]
pub struct BoolValue {
    value: bool,
}

#[derive(Clone, Debug)]
pub enum FloatValue {
    Float32(f32),
    Float64(f64),
}

#[derive(Clone, Debug)]
pub enum ValueType {
    Int(IntValue),
    Float(FloatValue),
    Bool(BoolValue),
    // Char(Char),
    String(StringValue),
    Vector(Vec<Value>),
}

#[derive(Clone, Debug)]
pub enum Value {
    Object(Object),
    Value(ValueType)
}

impl Value {
    fn to_val(&self) -> ValueType {
        match self {
            Value::Object(s) => s.get_value().unwrap().clone(),
            Value::Value(v) => v.clone()
        }
    }
    pub fn as_string(&self) -> String {
        match self {
            Value::Object(s) => {
                match s.get_value() {
                    Some(v) => v.as_string(),
                    None => {
                        format!("{:?} {:?}", s.name, s.values)
                    }
                }
            },
            Value::Value(v) => v.as_string()
        }
    }
    pub fn from_int_val(val: IntValue) -> Self {
        Self::Value(ValueType::Int(val))
    }

    pub fn from_bool(val: bool) -> Self {
        Self::Value(ValueType::Bool(BoolValue::new(val)))
    }
}


#[derive(Clone, Debug)]
pub struct Object {
    name: String,
    values: HashMap<String, Value>
}

impl BoolValue {
    pub fn new(value: bool) -> Self {
        BoolValue { value }
    }
}

#[derive(Clone, Debug)]
pub enum Instruction {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Load(String),
    Store(String),
    NewVariable(String),
    Delete(String),
    Jump(usize),
    JumpIfFalse(usize),
    JumpIfTrue(usize),
    Call(String),
    Push(ValueType),
    Pop,
    Nop,
    Return,
    Clone,
    Swap,
    Rotate,
    MoveBack(usize)
}


#[derive(Debug)]
pub struct VirtualMachine {
    stack: Vec<Value>,
    rom: Vec<Instruction>,
    pc: usize,
    heap: HashMap<String, Object>,
    functions: HashMap<String, usize>,
    call_stack: Vec<usize>,
    class_definitions: HashMap<String, ObjectCreator>,
}

impl IntValue {
    pub fn as_i8(&self) -> i8 {
        match self {
            IntValue::Int8(i) => *i,
            IntValue::Int16(i) => *i as i8,
            IntValue::Int32(i) => *i as i8,
            IntValue::Int64(i) => *i as i8,
        }
    }
    pub fn as_i16(&self) -> i16 {
        match self {
            IntValue::Int8(i) => *i as i16,
            IntValue::Int16(i) => *i,
            IntValue::Int32(i) => *i as i16,
            IntValue::Int64(i) => *i as i16,
        }
    }
    pub fn as_i32(&self) -> i32 {
        match self {
            IntValue::Int8(i) => *i as i32,
            IntValue::Int16(i) => *i as i32,
            IntValue::Int32(i) => *i,
            IntValue::Int64(i) => *i as i32,
        }
    }
    pub fn as_i64(&self) -> i64 {
        match self {
            IntValue::Int8(i) => *i as i64,
            IntValue::Int16(i) => *i as i64,
            IntValue::Int32(i) => *i as i64,
            IntValue::Int64(i) => *i,
        }
    }
    pub fn get_byte_size(&self) -> usize {
        match self {
            IntValue::Int8(_) => 1,
            IntValue::Int16(_) => 2,
            IntValue::Int32(_) => 4,
            IntValue::Int64(_) => 8,
        }
    }
    pub fn use_size_of_biggest(&self, other: &Self) -> usize {
        if self.get_byte_size() > other.get_byte_size() {
            self.get_byte_size()
        } else {
            other.get_byte_size()
        }
    }
    pub fn add_8(lhs: i8, rhs: i8) -> IntValue {
        match lhs.checked_add(rhs) {
            Some(r) => IntValue::Int8(r),
            None => { Self::add_16(lhs as i16, rhs as i16) }
        }
    }
    pub fn add_16(lhs: i16, rhs: i16) -> IntValue {
        match lhs.checked_add(rhs) {
            Some(r) => IntValue::Int16(r),
            None => { Self::add_32(lhs as i32, rhs as i32) }
        }
    }
    pub fn add_32(lhs: i32, rhs: i32) -> IntValue {
        match lhs.checked_add(rhs) {
            Some(r) => IntValue::Int32(r),
            None => { Self::add_64(lhs as i64, rhs as i64) }
        }
    }
    pub fn add_64(lhs: i64, rhs: i64) -> IntValue {
        match lhs.checked_add(rhs) {
            Some(r) => IntValue::Int64(r),
            None => { panic!("Integer overflow") }
        }
    }
    pub fn add(self, other: Self) -> Self {
        match &self.use_size_of_biggest(&other) {
            1 => Self::add_8(self.as_i8(), other.as_i8()),
            2 => Self::add_16(self.as_i16(), other.as_i16()),
            4 => Self::add_32(self.as_i32(), other.as_i32()),
            8 => Self::add_64(self.as_i64(), other.as_i64()),
            _ => panic!("Invalid byte size for integer value"),
        }
    }
    pub fn sub_8(lhs: i8, rhs: i8) -> IntValue {
        match lhs.checked_sub(rhs) {
            Some(r) => IntValue::Int8(r),
            None => { Self::sub_16(lhs as i16, rhs as i16) }
        }
    }
    pub fn sub_16(lhs: i16, rhs: i16) -> IntValue {
        match lhs.checked_sub(rhs) {
            Some(r) => IntValue::Int16(r),
            None => { Self::sub_32(lhs as i32, rhs as i32) }
        }
    }
    pub fn sub_32(lhs: i32, rhs: i32) -> IntValue {
        match lhs.checked_sub(rhs) {
            Some(r) => IntValue::Int32(r),
            None => { Self::sub_64(lhs as i64, rhs as i64) }
        }
    }
    pub fn sub_64(lhs: i64, rhs: i64) -> IntValue {
        match lhs.checked_sub(rhs) {
            Some(r) => IntValue::Int64(r),
            None => { panic!("Integer overflow") }
        }
    }
    pub fn sub(self, other: Self) -> Self {
        match &self.use_size_of_biggest(&other) {
            1 => Self::sub_8(self.as_i8(), other.as_i8()),
            2 => Self::sub_16(self.as_i16(), other.as_i16()),
            4 => Self::sub_32(self.as_i32(), other.as_i32()),
            8 => Self::sub_64(self.as_i64(), other.as_i64()),
            _ => panic!("Invalid byte size for integer value"),
        }
    }
    pub fn mul_8(lhs: i8, rhs: i8) -> IntValue {
        match lhs.checked_mul(rhs) {
            Some(r) => IntValue::Int8(r),
            None => { Self::mul_16(lhs as i16, rhs as i16) }
        }
    }
    pub fn mul_16(lhs: i16, rhs: i16) -> IntValue {
        match lhs.checked_mul(rhs) {
            Some(r) => IntValue::Int16(r),
            None => { Self::mul_32(lhs as i32, rhs as i32) }
        }
    }
    pub fn mul_32(lhs: i32, rhs: i32) -> IntValue {
        match lhs.checked_mul(rhs) {
            Some(r) => IntValue::Int32(r),
            None => { Self::mul_64(lhs as i64, rhs as i64) }
        }
    }
    pub fn mul_64(lhs: i64, rhs: i64) -> IntValue {
        match lhs.checked_mul(rhs) {
            Some(r) => IntValue::Int64(r),
            None => { panic!("Integer overflow") }
        }
    }
    pub fn mul(self, other: Self) -> Self {
        match &self.use_size_of_biggest(&other) {
            1 => Self::mul_8(self.as_i8(), other.as_i8()),
            2 => Self::mul_16(self.as_i16(), other.as_i16()),
            4 => Self::mul_32(self.as_i32(), other.as_i32()),
            8 => Self::mul_64(self.as_i64(), other.as_i64()),
            _ => panic!("Invalid byte size for integer value"),
        }
    }
    pub fn div_8(lhs: i8, rhs: i8) -> IntValue {
        match lhs.checked_div(rhs) {
            Some(r) => IntValue::Int8(r),
            None => { Self::div_16(lhs as i16, rhs as i16) }
        }
    }
    pub fn div_16(lhs: i16, rhs: i16) -> IntValue {
        match lhs.checked_div(rhs) {
            Some(r) => IntValue::Int16(r),
            None => { Self::div_32(lhs as i32, rhs as i32) }
        }
    }
    pub fn div_32(lhs: i32, rhs: i32) -> IntValue {
        match lhs.checked_div(rhs) {
            Some(r) => IntValue::Int32(r),
            None => { Self::div_64(lhs as i64, rhs as i64) }
        }
    }
    pub fn div_64(lhs: i64, rhs: i64) -> IntValue {
        match lhs.checked_div(rhs) {
            Some(r) => IntValue::Int64(r),
            None => { panic!("Integer overflow") }
        }
    }
    pub fn div(self, other: Self) -> Self {
        match &self.use_size_of_biggest(&other) {
            1 => Self::div_8(self.as_i8(), other.as_i8()),
            2 => Self::div_16(self.as_i16(), other.as_i16()),
            4 => Self::div_32(self.as_i32(), other.as_i32()),
            8 => Self::div_64(self.as_i64(), other.as_i64()),
            _ => panic!("Invalid byte size for integer value"),
        }
    }
    pub fn eq(self, other: Self) -> bool {
        match &self.use_size_of_biggest(&other) {
            1 => self.as_i8() == other.as_i8(),
            2 => self.as_i16() == other.as_i16(),
            4 => self.as_i32() == other.as_i32(),
            8 => self.as_i64() == other.as_i64(),
            _ => panic!("Invalid byte size for integer value"),
        }
    }
    pub fn ne(self, other: Self) -> bool {
        match &self.use_size_of_biggest(&other) {
            1 => self.as_i8() != other.as_i8(),
            2 => self.as_i16() != other.as_i16(),
            4 => self.as_i32() != other.as_i32(),
            8 => self.as_i64() != other.as_i64(),
            _ => panic!("Invalid byte size for integer value"),
        }
    }
    pub fn lt(self, other: Self) -> bool {
        match &self.use_size_of_biggest(&other) {
            1 => self.as_i8() < other.as_i8(),
            2 => self.as_i16() < other.as_i16(),
            4 => self.as_i32() < other.as_i32(),
            8 => self.as_i64() < other.as_i64(),
            _ => panic!("Invalid byte size for integer value"),
        }
    }
    pub fn le(self, other: Self) -> bool {
        match &self.use_size_of_biggest(&other) {
            1 => self.as_i8() <= other.as_i8(),
            2 => self.as_i16() <= other.as_i16(),
            4 => self.as_i32() <= other.as_i32(),
            8 => self.as_i64() <= other.as_i64(),
            _ => panic!("Invalid byte size for integer value"),
        }
    }
    pub fn gt(self, other: Self) -> bool {
        match &self.use_size_of_biggest(&other) {
            1 => self.as_i8() > other.as_i8(),
            2 => self.as_i16() > other.as_i16(),
            4 => self.as_i32() > other.as_i32(),
            8 => self.as_i64() > other.as_i64(),
            _ => panic!("Invalid byte size for integer value"),
        }
    }
    pub fn ge(self, other: Self) -> bool {
        match &self.use_size_of_biggest(&other) {
            1 => self.as_i8() >= other.as_i8(),
            2 => self.as_i16() >= other.as_i16(),
            4 => self.as_i32() >= other.as_i32(),
            8 => self.as_i64() >= other.as_i64(),
            _ => panic!("Invalid byte size for integer value"),
        }
    }
}


impl FloatValue {
    pub fn as_f32(&self) -> f32 {
        match self {
            FloatValue::Float32(f) => *f,
            FloatValue::Float64(f) => *f as f32,
        }
    }
    pub fn as_f64(&self) -> f64 {
        match self {
            FloatValue::Float32(f) => *f as f64,
            FloatValue::Float64(f) => *f,
        }
    }
    pub fn get_size(&self) -> usize {
        match self {
            FloatValue::Float32(_) => 4,
            FloatValue::Float64(_) => 8,
        }
    }
    pub fn use_size_of_biggest(&self, other: &Self) -> usize {
        if self.get_size() > other.get_size() {
            self.get_size()
        } else {
            other.get_size()
        }
    }
    pub fn add_32(lhs: f32, rhs: f32) -> FloatValue {
        let res = lhs + rhs;
        if res.is_finite() {
            FloatValue::Float32(res)
        } else {
            Self::add_64(lhs as f64, rhs as f64)
        }
    }
    pub fn add_64(lhs: f64, rhs: f64) -> FloatValue {
        FloatValue::Float64(lhs + rhs)
    }
    pub fn add(self, other: Self) -> Self {
        match &self.use_size_of_biggest(&other) {
            4 => Self::add_32(self.as_f32(), other.as_f32()),
            8 => Self::add_64(self.as_f64(), other.as_f64()),
            _ => panic!("Invalid byte size for float value"),
        }
    }
    pub fn sub_32(lhs: f32, rhs: f32) -> FloatValue {
        let res = lhs - rhs;
        if res.is_finite() {
            FloatValue::Float32(res)
        } else {
            Self::sub_64(lhs as f64, rhs as f64)
        }
    }
    pub fn sub_64(lhs: f64, rhs: f64) -> FloatValue {
        FloatValue::Float64(lhs - rhs)
    }
    pub fn sub(self, other: Self) -> Self {
        match &self.use_size_of_biggest(&other) {
            4 => Self::sub_32(self.as_f32(), other.as_f32()),
            8 => Self::sub_64(self.as_f64(), other.as_f64()),
            _ => panic!("Invalid byte size for float value"),
        }
    }
    pub fn mul_32(lhs: f32, rhs: f32) -> FloatValue {
        let res = lhs * rhs;
        if res.is_finite() {
            FloatValue::Float32(res)
        } else {
            Self::mul_64(lhs as f64, rhs as f64)
        }
    }
    pub fn mul_64(lhs: f64, rhs: f64) -> FloatValue {
        FloatValue::Float64(lhs * rhs)
    }
    pub fn mul(self, other: Self) -> Self {
        match &self.use_size_of_biggest(&other) {
            4 => Self::mul_32(self.as_f32(), other.as_f32()),
            8 => Self::mul_64(self.as_f64(), other.as_f64()),
            _ => panic!("Invalid byte size for float value"),
        }
    }
    pub fn div_32(lhs: f32, rhs: f32) -> FloatValue {
        let res = lhs / rhs;
        if res.is_finite() {
            FloatValue::Float32(res)
        } else {
            Self::div_64(lhs as f64, rhs as f64)
        }
    }
    pub fn div_64(lhs: f64, rhs: f64) -> FloatValue {
        FloatValue::Float64(lhs / rhs)
    }
    pub fn div(self, other: Self) -> Self {
        match &self.use_size_of_biggest(&other) {
            4 => Self::div_32(self.as_f32(), other.as_f32()),
            8 => Self::div_64(self.as_f64(), other.as_f64()),
            _ => panic!("Invalid byte size for float value"),
        }
    }
    pub fn eq(self, other: Self) -> bool {
        match &self.use_size_of_biggest(&other) {
            4 => self.as_f32() == other.as_f32(),
            8 => self.as_f64() == other.as_f64(),
            _ => panic!("Invalid byte size for float value"),
        }
    }
    pub fn ne(self, other: Self) -> bool {
        match &self.use_size_of_biggest(&other) {
            4 => self.as_f32() != other.as_f32(),
            8 => self.as_f64() != other.as_f64(),
            _ => panic!("Invalid byte size for float value"),
        }
    }
    pub fn lt(self, other: Self) -> bool {
        match &self.use_size_of_biggest(&other) {
            4 => self.as_f32() < other.as_f32(),
            8 => self.as_f64() < other.as_f64(),
            _ => panic!("Invalid byte size for float value"),
        }
    }
    pub fn le(self, other: Self) -> bool {
        match &self.use_size_of_biggest(&other) {
            4 => self.as_f32() <= other.as_f32(),
            8 => self.as_f64() <= other.as_f64(),
            _ => panic!("Invalid byte size for float value"),
        }
    }
    pub fn gt(self, other: Self) -> bool {
        match &self.use_size_of_biggest(&other) {
            4 => self.as_f32() > other.as_f32(),
            8 => self.as_f64() > other.as_f64(),
            _ => panic!("Invalid byte size for float value"),
        }
    }
    pub fn ge(self, other: Self) -> bool {
        match &self.use_size_of_biggest(&other) {
            4 => self.as_f32() >= other.as_f32(),
            8 => self.as_f64() >= other.as_f64(),
            _ => panic!("Invalid byte size for float value"),
        }
    }
}

impl StringValue {
    pub fn new(value: String) -> Self {
        Self { value }
    }
    pub fn add(self, other: Self) -> Self {
        match (self, other) {
            (StringValue { value: lhs}, StringValue { value: rhs }) => {
                StringValue::new(lhs + &rhs)
            }
        }
    }
    pub fn eq(self, other: Self) -> bool {
        match (self, other) {
            (StringValue { value: lhs}, StringValue { value: rhs }) => lhs == rhs,
        }
    }
    pub fn ne(self, other: Self) -> bool {
        match (self, other) {
            (StringValue { value: lhs}, StringValue { value: rhs }) => lhs != rhs,
        }
    }
}

impl ValueType {
    pub fn as_string(&self) -> String {
        match self {
            ValueType::Int(int) => {
                int.as_i64().to_string()
            }
            ValueType::Float(float) => {
                float.as_f64().to_string()
            }
            ValueType::Bool(bool) => {
                bool.value.to_string()
            }
            ValueType::String(string) => {
                string.value.clone()
            }
            ValueType::Vector(vec) => {
                let mut str = "[".to_string();
                for value in vec {
                    str += &*value.as_string();
                    str += ", "
                }
                str.pop();
                str.pop();
                str += "]";
                str
            }
        }
    }
    pub fn add(self, other: Self) -> Self {
        match (self.clone(), other.clone()) {
            (ValueType::Int(lhs_int), ValueType::Int(rhs_int)) => {
                ValueType::Int(lhs_int.add(rhs_int))
            }
            (ValueType::Float(lhs_float), ValueType::Float(rhs_float)) => {
                ValueType::Float(lhs_float.add(rhs_float))
            }
            (ValueType::Int(lhs_int), ValueType::Float(rhs_float)) => {
                match rhs_float {
                    FloatValue::Float32(rhs_float) => {
                        ValueType::Float(FloatValue::Float32(lhs_int.as_i64() as f32 + rhs_float))
                    }
                    FloatValue::Float64(rhs_float) => {
                        ValueType::Float(FloatValue::Float64(lhs_int.as_i64() as f64  + rhs_float))
                    }
                }
            }
            (ValueType::Float(lhs_float), ValueType::Int(rhs_int)) => {
                match lhs_float {
                    FloatValue::Float32(lhs_float) => {
                        ValueType::Float(FloatValue::Float32(lhs_float + rhs_int.as_i64() as f32))
                    }
                    FloatValue::Float64(lhs_float) => {
                        ValueType::Float(FloatValue::Float64(lhs_float + rhs_int.as_i64() as f64))
                    }
                }
            }
            (ValueType::String(lhs_string), ValueType::String(rhs_string)) => {
                ValueType::String(lhs_string.add(rhs_string))
            }
            (ValueType::Vector(vec), ValueType::Vector(mut other_vec)) => {
                let mut new_vec = vec.clone();
                new_vec.append(&mut other_vec);
                ValueType::Vector(new_vec)
            }
            _ => {
                panic!("Invalid types for add operation: {:?} + {:?}", self, other);
            }
        }
    }
    pub fn sub(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Int(lhs_int), ValueType::Int(rhs_int)) => {
                ValueType::Int(lhs_int.sub(rhs_int))
            }
            (ValueType::Float(lhs_float), ValueType::Float(rhs_float)) => {
                ValueType::Float(lhs_float.sub(rhs_float))
            }
            (ValueType::Int(lhs_int), ValueType::Float(rhs_float)) => {
                match rhs_float {
                    FloatValue::Float32(rhs_float) => {
                        ValueType::Float(FloatValue::Float32(lhs_int.as_i64() as f32 - rhs_float))
                    }
                    FloatValue::Float64(rhs_float) => {
                        ValueType::Float(FloatValue::Float64(lhs_int.as_i64() as f64 - rhs_float))
                    }
                }
            }
            (ValueType::Float(lhs_float), ValueType::Int(rhs_int)) => {
                match lhs_float {
                    FloatValue::Float32(lhs_float) => {
                        ValueType::Float(FloatValue::Float32(lhs_float - rhs_int.as_i64() as f32))
                    }
                    FloatValue::Float64(lhs_float) => {
                        ValueType::Float(FloatValue::Float64(lhs_float - rhs_int.as_i64() as f64))
                    }
                }
            }
            _ => {
                panic!("Invalid types for sub operation");
            }
        }
    }
    pub fn mul(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Int(lhs_int), ValueType::Int(rhs_int)) => {
                ValueType::Int(lhs_int.mul(rhs_int))
            }
            (ValueType::Float(lhs_float), ValueType::Float(rhs_float)) => {
                ValueType::Float(lhs_float.mul(rhs_float))
            }
            (ValueType::Int(lhs_int), ValueType::Float(rhs_float)) => {
                match rhs_float {
                    FloatValue::Float32(rhs_float) => {
                        ValueType::Float(FloatValue::Float32(lhs_int.as_i64() as f32 * rhs_float))
                    }
                    FloatValue::Float64(rhs_float) => {
                        ValueType::Float(FloatValue::Float64(lhs_int.as_i64() as f64 * rhs_float))
                    }
                }
            }
            (ValueType::Float(lhs_float), ValueType::Int(rhs_int)) => {
                match lhs_float {
                    FloatValue::Float32(lhs_float) => {
                        ValueType::Float(FloatValue::Float32(lhs_float * rhs_int.as_i64() as f32))
                    }
                    FloatValue::Float64(lhs_float) => {
                        ValueType::Float(FloatValue::Float64(lhs_float * rhs_int.as_i64() as f64))
                    }
                }
            }
            _ => {
                panic!("Invalid types for mul operation");
            }
        }
    }
    pub fn div(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Int(lhs_int), ValueType::Int(rhs_int)) => {
                ValueType::Int(lhs_int.div(rhs_int))
            }
            (ValueType::Float(lhs_float), ValueType::Float(rhs_float)) => {
                ValueType::Float(lhs_float.div(rhs_float))
            }
            (ValueType::Int(lhs_int), ValueType::Float(rhs_float)) => {
                match rhs_float {
                    FloatValue::Float32(rhs_float) => {
                        ValueType::Float(FloatValue::Float32(lhs_int.as_i64() as f32 / rhs_float))
                    }
                    FloatValue::Float64(rhs_float) => {
                        ValueType::Float(FloatValue::Float64(lhs_int.as_i64() as f64 / rhs_float))
                    }
                }
            }
            (ValueType::Float(lhs_float), ValueType::Int(rhs_int)) => {
                match lhs_float {
                    FloatValue::Float32(lhs_float) => {
                        ValueType::Float(FloatValue::Float32(lhs_float / rhs_int.as_i64() as f32))
                    }
                    FloatValue::Float64(lhs_float) => {
                        ValueType::Float(FloatValue::Float64(lhs_float / rhs_int.as_i64() as f64))
                    }
                }
            }
            _ => {
                panic!("Invalid types for div operation");
            }
        }
    }
    pub fn eq(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Int(lhs_int), ValueType::Int(rhs_int)) => {
                ValueType::Bool(BoolValue::new((lhs_int.eq(rhs_int))))
            }
            (ValueType::Float(lhs_float), ValueType::Float(rhs_float)) => {
                ValueType::Bool(BoolValue::new((lhs_float.eq(rhs_float))))
            }
            (ValueType::Int(lhs_int), ValueType::Float(rhs_float)) => {
                match rhs_float {
                    FloatValue::Float32(rhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_int.as_i64() as f32 == rhs_float)))
                    }
                    FloatValue::Float64(rhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_int.as_i64() as f64 == rhs_float)))
                    }
                }
            }
            (ValueType::Float(lhs_float), ValueType::Int(rhs_int)) => {
                match lhs_float {
                    FloatValue::Float32(lhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_float == rhs_int.as_i64() as f32)))
                    }
                    FloatValue::Float64(lhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_float == rhs_int.as_i64() as f64)))
                    }
                }
            }
            (ValueType::String(lhs_string), ValueType::String(rhs_string)) => {
                ValueType::Bool(BoolValue::new((lhs_string.eq(rhs_string))))
            }
            _ => {
                panic!("Invalid types for eq operation");
            }
        }
    }
    pub fn ne(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Int(lhs_int), ValueType::Int(rhs_int)) => {
                ValueType::Bool(BoolValue::new((lhs_int.ne(rhs_int))))
            }
            (ValueType::Float(lhs_float), ValueType::Float(rhs_float)) => {
                ValueType::Bool(BoolValue::new((lhs_float.ne(rhs_float))))
            }
            (ValueType::Int(lhs_int), ValueType::Float(rhs_float)) => {
                match rhs_float {
                    FloatValue::Float32(rhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_int.as_i64() as f32 != rhs_float)))
                    }
                    FloatValue::Float64(rhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_int.as_i64() as f64 != rhs_float)))
                    }
                }
            }
            (ValueType::Float(lhs_float), ValueType::Int(rhs_int)) => {
                match lhs_float {
                    FloatValue::Float32(lhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_float != rhs_int.as_i64() as f32)))
                    }
                    FloatValue::Float64(lhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_float != rhs_int.as_i64() as f64)))
                    }
                }
            }
            (ValueType::String(lhs_string), ValueType::String(rhs_string)) => {
                ValueType::Bool(BoolValue::new((lhs_string.ne(rhs_string))))
            }
            _ => {
                panic!("Invalid types for neq operation");
            }
        }
    }
    pub fn gt(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Int(lhs_int), ValueType::Int(rhs_int)) => {
                ValueType::Bool(BoolValue::new((lhs_int.gt(rhs_int))))
            }
            (ValueType::Float(lhs_float), ValueType::Float(rhs_float)) => {
                ValueType::Bool(BoolValue::new((lhs_float.gt(rhs_float))))
            }
            (ValueType::Int(lhs_int), ValueType::Float(rhs_float)) => {
                match rhs_float {
                    FloatValue::Float32(rhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_int.as_i64() as f32 > rhs_float)))
                    }
                    FloatValue::Float64(rhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_int.as_i64() as f64 > rhs_float)))
                    }
                }
            }
            (ValueType::Float(lhs_float), ValueType::Int(rhs_int)) => {
                match lhs_float {
                    FloatValue::Float32(lhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_float > rhs_int.as_i64() as f32)))
                    }
                    FloatValue::Float64(lhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_float > rhs_int.as_i64() as f64)))
                    }
                }
            }
            _ => {
                panic!("Invalid types for gt operation");
            }
        }
    }
    pub fn lt(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Int(lhs_int), ValueType::Int(rhs_int)) => {
                ValueType::Bool(BoolValue::new((lhs_int.lt(rhs_int))))
            }
            (ValueType::Float(lhs_float), ValueType::Float(rhs_float)) => {
                ValueType::Bool(BoolValue::new((lhs_float.lt(rhs_float))))
            }
            (ValueType::Int(lhs_int), ValueType::Float(rhs_float)) => {
                match rhs_float {
                    FloatValue::Float32(rhs_float) => {
                        ValueType::Bool(BoolValue::new(((lhs_int.as_i64() as f32).lt(&rhs_float))))
                    }
                    FloatValue::Float64(rhs_float) => {
                        ValueType::Bool(BoolValue::new(((lhs_int.as_i64() as f64).lt(&rhs_float))))
                    }
                }
            }
            (ValueType::Float(lhs_float), ValueType::Int(rhs_int)) => {
                match lhs_float {
                    FloatValue::Float32(lhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_float < rhs_int.as_i64() as f32)))
                    }
                    FloatValue::Float64(lhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_float < rhs_int.as_i64() as f64)))
                    }
                }
            }
            _ => {
                panic!("Invalid types for lt operation");
            }
        }
    }
    pub fn ge(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Int(lhs_int), ValueType::Int(rhs_int)) => {
                ValueType::Bool(BoolValue::new((lhs_int.ge(rhs_int))))
            }
            (ValueType::Float(lhs_float), ValueType::Float(rhs_float)) => {
                ValueType::Bool(BoolValue::new((lhs_float.ge(rhs_float))))
            }
            (ValueType::Int(lhs_int), ValueType::Float(rhs_float)) => {
                match rhs_float {
                    FloatValue::Float32(rhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_int.as_i64() as f32 >= rhs_float)))
                    }
                    FloatValue::Float64(rhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_int.as_i64() as f64 >= rhs_float)))
                    }
                }
            }
            (ValueType::Float(lhs_float), ValueType::Int(rhs_int)) => {
                match lhs_float {
                    FloatValue::Float32(lhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_float >= rhs_int.as_i64() as f32)))
                    }
                    FloatValue::Float64(lhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_float >= rhs_int.as_i64() as f64)))
                    }
                }
            }
            _ => {
                panic!("Invalid types for gte operation");
            }
        }
    }
    pub fn le(self, other: Self) -> Self {
        match (self, other) {
            (ValueType::Int(lhs_int), ValueType::Int(rhs_int)) => {
                ValueType::Bool(BoolValue::new((lhs_int.le(rhs_int))))
            }
            (ValueType::Float(lhs_float), ValueType::Float(rhs_float)) => {
                ValueType::Bool(BoolValue::new((lhs_float.le(rhs_float))))
            }
            (ValueType::Int(lhs_int), ValueType::Float(rhs_float)) => {
                match rhs_float {
                    FloatValue::Float32(rhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_int.as_i64() as f32 <= rhs_float)))
                    }
                    FloatValue::Float64(rhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_int.as_i64() as f64 <= rhs_float)))
                    }
                }
            }
            (ValueType::Float(lhs_float), ValueType::Int(rhs_int)) => {
                match lhs_float {
                    FloatValue::Float32(lhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_float <= rhs_int.as_i64() as f32)))
                    }
                    FloatValue::Float64(lhs_float) => {
                        ValueType::Bool(BoolValue::new((lhs_float <= rhs_int.as_i64() as f64)))
                    }
                }
            }
            _ => {
                panic!("Invalid types for lte operation");
            }
        }
    }
}

impl Object {
    pub fn new(name: String) -> Self {
        let mut values = HashMap::new();
        Self { name, values }
    }
    fn get_value(&self) -> Option<ValueType> {
        match self.values.get("__value__") {
            None => { None }
            Some(val) => { Some(val.to_val()) }
        }
    }
    // fn get_member(&self, name: String) -> ValueType {
    //     self.values.get(&name).unwrap().clone().to_val()
    // }
    fn set_value(&mut self, val: Value) {
        self.values.insert("__value__".to_string(), val);
    }
    fn set_member(&mut self, name: String, val: Value) {
        if name.contains("."){
            // splits the name into the object name and the member name, then sets the member, member name can have multiple dots
            let mut split = name.split(".");
            let obj_name = split.next().unwrap().to_string();
            let member_name = split.collect::<Vec<&str>>().join(".");
            match self.values.get_mut(&obj_name).unwrap() {
                Value::Object(obj) => {
                    obj.set_member(member_name, val);
                }
                _ => {
                    panic!("Invalid member type");
                }
            };
        } else {
            self.values.insert(name, val);
        }
    }
    fn get_member(&mut self, name: String) -> ValueType {
        if name.contains("."){
            // splits the name into the object name and the member name, then sets the member, member name can have multiple dots
            let mut split = name.split(".");
            let obj_name = split.next().unwrap().to_string();
            let member_name = split.collect::<Vec<&str>>().join(".");
            match self.values.get_mut(&obj_name).unwrap(){
                Value::Object(obj) => {
                    return obj.get_member(member_name);
                }
                _ => {
                    panic!("Invalid member type");
                }
            };
        } else {
            return match self.values.get(&name){
                None => {
                    println!("{:#?}", self.values);
                    panic!("Invalid member name, '{}'", name);
                }
                Some(val) => {
                    val.to_val()
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ObjectCreator {
    name: String,
    members: Vec<String>,
}

impl ObjectCreator {
    pub fn new(name: String) -> Self {
        Self { name, members: Vec::new() }
    }
    pub fn add_member(&mut self, name: String) {
        self.members.push(name);
    }
    pub fn create(&self, members: Vec<Value>) -> Option<Object> {
        if members.len() != self.members.len() {
            return None;
        }
        let mut s = Object::new(self.name.clone());
        for i in 0..members.len() {
            s.set_member(self.members[i].clone(), members[i].clone());
        }
        Some(s)
    }
}

impl VirtualMachine {
    pub fn new(mut new_rom: Vec<Instruction>, functions: HashMap<String, usize>, class_definitions: HashMap<String, ObjectCreator>) -> Self {
        let mut rom = vec![Instruction::Nop];
        rom.append(&mut new_rom);
        Self {
            stack: Vec::new(),
            rom,
            pc: 0,
            heap: HashMap::new(),
            functions,
            call_stack: vec![],
            class_definitions
        }
    }
    pub fn emulate(instructions: Vec<Instruction>, functions: HashMap<String, usize>, classes: HashMap<String, ObjectCreator>) {
        Self::new(instructions, functions, classes).run();
    }
    fn current_instruction(&self) -> Instruction {
        self.rom[self.pc].clone()
    }

    fn next_instruction(&mut self) -> bool {
        self.pc += 1;
        self.pc < self.rom.len()
    }

    fn single_run(&mut self, instruction: Instruction){
        match instruction {
            Instruction::Add => {
                if self.stack.len() < 2 {
                    println!("{:?}", self.stack);
                    panic!("Stack underflow");
                }

                let rhs = self.stack.pop().unwrap().to_val();
                let lhs = self.stack.pop().unwrap().to_val();
                self.stack.push(
                    Value::Value(lhs.add(rhs))
                )
            }
            Instruction::Sub => {
                if self.stack.len() < 2 {
                    panic!("Stack underflow");
                }

                let rhs = self.stack.pop().unwrap().to_val();
                let lhs = self.stack.pop().unwrap().to_val();
                self.stack.push(
                    Value::Value(lhs.sub(rhs))
                )
            }
            Instruction::Mul => {
                if self.stack.len() < 2 {
                    panic!("Stack underflow");
                }

                let rhs = self.stack.pop().unwrap().to_val();
                let lhs = self.stack.pop().unwrap().to_val();
                self.stack.push(
                    Value::Value(lhs.mul(rhs))
                )
            }
            Instruction::Div => {
                if self.stack.len() < 2 {
                    panic!("Stack underflow");
                }

                let rhs = self.stack.pop().unwrap().to_val();
                let lhs = self.stack.pop().unwrap().to_val();
                self.stack.push(
                    Value::Value(lhs.div(rhs))
                )
            }
            Instruction::Eq => {
                if self.stack.len() < 2 {
                    panic!("Stack underflow");
                }

                let rhs = self.stack.pop().unwrap().to_val();
                let lhs = self.stack.pop().unwrap().to_val();
                self.stack.push(
                    Value::Value(lhs.eq(rhs))
                )
            }
            Instruction::Neq => {
                if self.stack.len() < 2 {
                    panic!("Stack underflow");
                }

                let rhs = self.stack.pop().unwrap().to_val();
                let lhs = self.stack.pop().unwrap().to_val();
                self.stack.push(
                    Value::Value(lhs.ne(rhs))
                )
            }
            Instruction::Lt => {
                if self.stack.len() < 2 {
                    panic!("Stack underflow");
                }

                let rhs = self.stack.pop().unwrap().to_val();
                let lhs = self.stack.pop().unwrap().to_val();
                self.stack.push(
                    Value::Value(lhs.lt(rhs))
                )
            }
            Instruction::Gt => {
                if self.stack.len() < 2 {
                    panic!("Stack underflow");
                }

                let rhs = self.stack.pop().unwrap().to_val();
                let lhs = self.stack.pop().unwrap().to_val();
                self.stack.push(
                    Value::Value(lhs.gt(rhs))
                )
            }
            Instruction::LtEq => {
                if self.stack.len() < 2 {
                    panic!("Stack underflow");
                }

                let rhs = self.stack.pop().unwrap().to_val();
                let lhs = self.stack.pop().unwrap().to_val();
                self.stack.push(
                    Value::Value(lhs.le(rhs))
                )
            }
            Instruction::GtEq => {
                if self.stack.len() < 2 {
                    panic!("Stack underflow");
                }

                let rhs = self.stack.pop().unwrap().to_val();
                let lhs = self.stack.pop().unwrap().to_val();
                self.stack.push(
                    Value::Value(lhs.ge(rhs))
                )
            }
            Instruction::Load(name) => {
                let mut path: VecDeque<&str> = name.split(".").into_iter().collect::<VecDeque<&str>>();
                let var = path.pop_front().unwrap();
                if let Some(val) = self.heap.get_mut(&var.to_string()) {
                    let name = path.into_iter().collect::<Vec<&str>>().join(".");
                    if name.is_empty(){
                        match val.get_value() {
                            None => {
                                let val = self.heap.get(&var.to_string()).unwrap().clone();
                                self.stack.push(Value::Object(val));
                            }
                            Some(value) => {self.stack.push(Value::Value(value))}
                        }
                    } else {
                        self.stack.push(Value::Value(val.get_member(name)));
                    }
                } else {
                    panic!("Struct {:?} not found", name);
                }
            }
            Instruction::Store(name) => {
                let mut path: VecDeque<&str> = name.split(".").into_iter().collect::<VecDeque<&str>>();
                // todo: struct member store
                let var = path.pop_front().unwrap();
                if let Some(val) = self.heap.get_mut(&var.to_string()) {
                    let name = path.into_iter().collect::<Vec<&str>>().join(".");
                    match self.stack.pop().unwrap(){
                        Value::Value(v) => {
                            if name.is_empty() {
                                val.set_value(Value::Value(v));
                            } else {
                                val.set_member(name, Value::Value(v));
                            }
                        },
                        Value::Object(obj) => {
                            if name.is_empty() {
                                self.heap.insert(var.to_string(), obj);
                            } else {
                                val.set_member(name, Value::Object(obj));
                            }
                        }
                    };
                } else {
                    panic!("Struct not found");
                }
            }
            Instruction::NewVariable(name) => {
                if self.stack.is_empty() {
                    panic!("Stack underflow");
                }

                let value = match self.stack.pop().unwrap(){
                    Value::Value(v) => {
                        let mut sd = Object::new("".to_string());
                        sd.set_value(Value::Value(v));
                        sd
                    },
                    Value::Object(obj) => obj
                };
                match self.heap.insert(name.clone(), value) {
                    None => {}
                    Some(_) => { panic!("{} already exists", name) }
                }
            }
            Instruction::Delete(name) => {
                match self.heap.remove(&*name) {
                    None => { panic!("{} doesn't exists", name) }
                    Some(_) => {}
                }
            }
            Instruction::Jump(line) => {
                self.pc = line - 1;
            }
            Instruction::JumpIfFalse(line) => {
                if let Some(val) = self.stack.pop() {
                    let res = match val.to_val() {
                        ValueType::Bool(b) => b.value,
                        _ => panic!("Invalid type")
                    };
                    if !res {
                        self.pc = line - 1;
                    }
                } else {
                    panic!("Stack underflow");
                }
            }
            Instruction::JumpIfTrue(line) => {
                if let Some(val) = self.stack.pop() {
                    let res = match val.to_val() {
                        ValueType::Bool(b) => b.value,
                        _ => panic!("Invalid type")
                    };
                    if res {
                        self.pc = line - 1;
                    }
                } else {
                    panic!("Stack underflow");
                }
            }
            Instruction::Call(function_name) => {
                if BUILTIN_FUNCTIONS.contains(&&*function_name){
                    self.call_builtin(&*function_name);
                } else if self.class_definitions.contains_key(&*function_name) {
                    self.class_call(&*function_name)
                }
                else if let Some(line) = self.functions.get(&function_name) {
                    self.call_stack.push(self.pc);
                    self.pc = line - 1;
                } else {
                    panic!("Function not found");
                }
            }
            Instruction::Push(value) => {
                self.stack.push(Value::Value(value));
            }
            Instruction::Pop => { unimplemented!("Pop"); }
            Instruction::Nop => {}
            Instruction::Return => {
                if let Some(line) = self.call_stack.pop() {
                    self.pc = line;
                } else {
                    panic!("Call stack underflow");
                }
            }
            Instruction::Clone => {
                if let Some(val) = self.stack.pop() {
                    self.stack.push(val.clone());
                    self.stack.push(val);
                } else {
                    panic!("Stack underflow");
                }
            }
            Instruction::Swap => {
                if self.stack.len() < 2 {
                    panic!("Stack underflow");
                }

                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                self.stack.push(rhs);
                self.stack.push(lhs);
            }
            Instruction::Rotate => {
                if self.stack.len() < 3 {
                    panic!("Stack underflow");
                }
                let rhs = self.stack.pop().unwrap();
                let mid = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                self.stack.push(mid);
                self.stack.push(lhs);
                self.stack.push(rhs);
            }
            Instruction::MoveBack(amount) => {
                if self.stack.len() < amount {
                    panic!("Stack underflow");
                }
                let to_move = self.stack.pop().unwrap();
                self.stack.insert(self.stack.len() - amount, to_move);
            }
        }
    }
    fn top_as_len(&mut self, name: &str) -> i64 {
        match self.stack.pop().unwrap_or_else(|| panic!("{} Call, Stack underflow", name)).to_val() {
            ValueType::Int(int) => int.as_i64(),
            _ => panic!("{} Call, Invalid type", name)
        }
    }
    fn class_call(&mut self, name: &str){
        let len = self.top_as_len(&*format!("Struct Constructor {}", name));
        let mut args = Vec::from_iter(self.stack.drain(self.stack.len() - len as usize..).into_iter());
        let creator = self.class_definitions.get(&*name).unwrap();
        // println!("{:?}", self.class_definitions);
        match creator.create(args) {
            Some(obj) => {
                self.stack.push(Value::Object(obj));
            }
            None => { panic!("Struct {} not found", name) }
        }
    }
    fn call_builtin(&mut self, name: &str){
        let len = self.top_as_len(&*format!("Function {}", name));
        let mut args = VecDeque::from_iter(self.stack.drain(self.stack.len() - len as usize..).into_iter());
        match name {
            "print" => {
                let mut output = String::new();
                for arg in args {
                    output.push_str(&*arg.as_string());
                    output.push(' ')
                }
                output.pop();
                print!("{}", output)
            },
            "println" => {
                let mut output = String::new();
                for arg in args {
                    output.push_str(&*arg.as_string());
                    output.push(' ')
                }
                output.pop();
                println!("{}", output)
            },
            "format" => {
                if args.len() < 1 {
                    panic!("format function takes 1 or more arguments, {} given", args.len())
                }

                let mut string = match args.pop_front() {
                    Some(Value::Value(ValueType::String(string))) => string.value,
                    _ => panic!("format function takes string as first argument")
                };

                for arg in args {
                    string = string.replacen("{}", &*arg.as_string(), 1)
                }
                self.stack.push(
                    Value::Value(ValueType::String(StringValue {
                        value: string
                    }))
                )
            }
            "assert" => {
                if args.len() < 2 {
                    panic!("assert function takes 2 arguments, {} given", args.len())
                }

                let mut res = match args.pop_front() {
                    Some(Value::Value(ValueType::Bool(bool))) => bool.value,
                    res => panic!("assert function takes bool as first argument, got: {:?}", res)
                };

                if !res {
                    let mut output = String::new();
                    for arg in args {
                        output.push_str(&*arg.as_string());
                        output.push(' ')
                    }
                    output.pop();
                    panic!("Assertion failed: {}", output)
                }
            }
            "push" => {
                if args.len() != 2 {
                    panic!("push function takes 2 arguments, {} given", args.len())
                }

                let mut list = match args.pop_front() {
                    Some(Value::Value(ValueType::Vector(list))) => list,
                    val => panic!("push function takes list as first argument: {:?}", val)
                };
                let value = args.pop_front().unwrap_or_else(|| panic!("push function takes value as second argument"));
                list.push(value);
                self.stack.push(Value::Value(ValueType::Vector(list)))
            }
            "pop" => {
                if args.len() != 1 {
                    panic!("pop function takes 1 argument, {} given", args.len())
                }

                let mut list = match args.pop_front() {
                    Some(Value::Value(ValueType::Vector(list))) => list,
                    val => panic!("pop function takes list as first argument: {:?}", val)
                };
                let value = list.pop().unwrap_or_else(|| panic!("pop function takes list as first argument"));
                self.stack.push(Value::Value(ValueType::Vector(list)));
                self.stack.push(value)
            }
            "read_element" => {
                if args.len() != 2 {
                    panic!("read_element function takes 2 arguments, {} given", args.len())
                }

                let list = match args.pop_front() {
                    Some(Value::Value(ValueType::Vector(list))) => list,
                    val => panic!("read_element function takes list as first argument: {:?}", val)
                };
                let index = match args.pop_front() {
                    Some(Value::Value(ValueType::Int(int))) => int.as_i64(),
                    val => panic!("read_element function takes int as second argument: {:?}", val)
                };
                let item = match list.get(index as usize) {
                    Some(item) => item.clone(),
                    None => panic!("read_element function takes index in range of list")
                };
                self.stack.push(Value::Value(ValueType::Vector(list)));
                self.stack.push(item)
            }
            "write_element" => {
                if args.len() != 3 {
                    panic!("write_element function takes 3 arguments, {} given", args.len())
                }

                let mut list = match args.pop_front() {
                    Some(Value::Value(ValueType::Vector(list))) => list,
                    val => panic!("write_element function takes list as first argument: {:?}", val)
                };
                let index = match args.pop_front() {
                    Some(Value::Value(ValueType::Int(int))) => int.as_i64(),
                    val => panic!("write_element function takes int as second argument: {:?}", val)
                };
                let val = args.pop_front().unwrap_or_else(|| panic!("write_element function takes value as third argument"));
                match list.get_mut(index as usize) {
                    Some(item) => *item = val,
                    None => panic!("write_element function takes index in range of list")
                };
                self.stack.push(Value::Value(ValueType::Vector(list)))
            }
            _ => unimplemented!("builtin function {} is not implemented", name)
        }
    }
    pub fn run(&mut self){
        loop {
            self.single_run(self.current_instruction());

            if !self.next_instruction() {
                break;
            }
        }
    }

    pub fn get_heap(&self) -> &HashMap<String, Object> {
        &self.heap
    }
}

