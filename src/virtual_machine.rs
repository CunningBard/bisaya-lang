use std::collections::HashMap;
use std::collections::vec_deque::VecDeque;


const BUILTIN_FUNCTIONS: [&str; 3] = ["print", "println", "format"];


#[derive(Clone, Debug)]
pub enum IntValue {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
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
    pub fn add(self, other: Self) -> Self {
        match &self.use_size_of_biggest(&other) {
            1 => IntValue::Int8(self.as_i8() + other.as_i8()),
            2 => IntValue::Int16(self.as_i16() + other.as_i16()),
            4 => IntValue::Int32(self.as_i32() + other.as_i32()),
            8 => IntValue::Int64(self.as_i64() + other.as_i64()),
            _ => panic!("Invalid byte size for integer value"),
        }
    }
    pub fn sub(self, other: Self) -> Self {
        match &self.use_size_of_biggest(&other) {
            1 => IntValue::Int8(self.as_i8() - other.as_i8()),
            2 => IntValue::Int16(self.as_i16() - other.as_i16()),
            4 => IntValue::Int32(self.as_i32() - other.as_i32()),
            8 => IntValue::Int64(self.as_i64() - other.as_i64()),
            _ => panic!("Invalid byte size for integer value"),
        }
    }
    pub fn mul(self, other: Self) -> Self {
        match &self.use_size_of_biggest(&other) {
            1 => IntValue::Int8(self.as_i8() * other.as_i8()),
            2 => IntValue::Int16(self.as_i16() * other.as_i16()),
            4 => IntValue::Int32(self.as_i32() * other.as_i32()),
            8 => IntValue::Int64(self.as_i64() * other.as_i64()),
            _ => panic!("Invalid byte size for integer value"),
        }
    }
    pub fn div(self, other: Self) -> Self {
        match &self.use_size_of_biggest(&other) {
            1 => IntValue::Int8(self.as_i8() / other.as_i8()),
            2 => IntValue::Int16(self.as_i16() / other.as_i16()),
            4 => IntValue::Int32(self.as_i32() / other.as_i32()),
            8 => IntValue::Int64(self.as_i64() / other.as_i64()),
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

#[derive(Clone, Debug)]
pub enum FloatValue {
    Float32(f32),
    Float64(f64),
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
    pub fn add(self, other: Self) -> Self {
        match &self.use_size_of_biggest(&other) {
            4 => FloatValue::Float32(self.as_f32() + other.as_f32()),
            8 => FloatValue::Float64(self.as_f64() + other.as_f64()),
            _ => panic!("Invalid byte size for float value"),
        }
    }
    pub fn sub(self, other: Self) -> Self {
        match &self.use_size_of_biggest(&other) {
            4 => FloatValue::Float32(self.as_f32() - other.as_f32()),
            8 => FloatValue::Float64(self.as_f64() - other.as_f64()),
            _ => panic!("Invalid byte size for float value"),
        }
    }
    pub fn mul(self, other: Self) -> Self {
        match &self.use_size_of_biggest(&other) {
            4 => FloatValue::Float32(self.as_f32() * other.as_f32()),
            8 => FloatValue::Float64(self.as_f64() * other.as_f64()),
            _ => panic!("Invalid byte size for float value"),
        }
    }
    pub fn div(self, other: Self) -> Self {
        match &self.use_size_of_biggest(&other) {
            4 => FloatValue::Float32(self.as_f32() / other.as_f32()),
            8 => FloatValue::Float64(self.as_f64() / other.as_f64()),
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

#[derive(Clone, Debug)]
enum CharValue {
    Char(char),
}

#[derive(Clone, Debug)]
pub struct  StringValue {
    value: String
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

#[derive(Clone, Debug)]
pub struct BoolValue {
    value: bool,
}

impl BoolValue {
    pub fn new(value: bool) -> Self {
        BoolValue { value }
    }
}

#[derive(Clone, Debug)]
pub enum ValueType {
    Int(IntValue),
    Float(FloatValue),
    Bool(BoolValue),
    // Char(Char),
    String(StringValue),
    // Vector(Vec<Value>),
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
        }
    }
    pub fn add(self, other: Self) -> Self {
        match (self, other) {
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
            _ => {
                panic!("Invalid types for add operation");
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

#[derive(Clone, Debug)]
pub enum Value {
    Struct(Struct),
    Value(ValueType)
}

impl Value {
    fn to_val(&self) -> ValueType {
        match self {
            Value::Struct(s) => s.get_value().clone(),
            Value::Value(v) => v.clone()
        }
    }
    pub fn from_int_val(val: IntValue) -> Self {
        Self::Value(ValueType::Int(val))
    }
}


#[derive(Clone, Debug)]
pub struct Struct {
    name: String,
    values: HashMap<String, Value>
}

impl Struct {
    pub fn new(name: String, val: Value) -> Self {
        let mut values = HashMap::new();
        values.insert("__value__".to_string(), val);
        Self { name, values }
    }
    fn get_value(&self) -> ValueType {
        self.values.get("__value__").unwrap().clone().to_val()
    }
    fn get_member(&self, name: String) -> ValueType {
        self.values.get(&name).unwrap().clone().to_val()
    }
    fn set_value(&mut self, val: Value) {
        self.values.insert("__value__".to_string(), val);
    }
    fn set_member(&mut self, name: String, val: Value) {
        self.values.insert(name, val);
    }
}

pub struct  StructCreator {
    name: String,
    members: Vec<String>,
}

impl StructCreator {
    pub fn new(name: String) -> Self {
        Self { name, members: Vec::new() }
    }
    pub fn add_member(&mut self, name: String) {
        self.members.push(name);
    }
    pub fn create(&self, members: Vec<Value>) -> Struct {
        if members.len() != self.members.len() + 1 {
            panic!("Expected {} number of members for {} struct, got {}", self.members.len() + 1, self.name, members.len())
        }
        let mut s = Struct::new(self.name.clone(), members[0].clone());
        for i in 1..members.len() {
            s.set_member(self.members[i-1].clone(), members[i].clone());
        }
        s
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
    NewVariable(String, Struct),
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
    stack: Vec<ValueType>,
    rom: Vec<Instruction>,
    pc: usize,
    heap: HashMap<String, Struct>,
    functions: HashMap<String, usize>,
    call_stack: Vec<usize>,
}

impl VirtualMachine {
    pub fn new(mut new_rom: Vec<Instruction>) -> Self {
        let mut rom = vec![Instruction::Nop];
        rom.append(&mut new_rom);
        Self {
            stack: Vec::new(),
            rom,
            pc: 0,
            heap: HashMap::new(),
            functions: Default::default(),
            call_stack: vec![]
        }
    }
    pub fn emulate(instructions: Vec<Instruction>){
        Self::new(instructions).run();
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

                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                self.stack.push(
                    lhs.add(rhs)
                )
            }
            Instruction::Sub => {
                if self.stack.len() < 2 {
                    panic!("Stack underflow");
                }

                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                self.stack.push(
                    lhs.sub(rhs)
                )
            }
            Instruction::Mul => {
                if self.stack.len() < 2 {
                    panic!("Stack underflow");
                }

                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                self.stack.push(
                    lhs.mul(rhs)
                )
            }
            Instruction::Div => {
                if self.stack.len() < 2 {
                    panic!("Stack underflow");
                }

                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                self.stack.push(
                    lhs.div(rhs)
                )
            }
            Instruction::Eq => {
                if self.stack.len() < 2 {
                    panic!("Stack underflow");
                }

                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                self.stack.push(
                    lhs.eq(rhs)
                )
            }
            Instruction::Neq => {
                if self.stack.len() < 2 {
                    panic!("Stack underflow");
                }

                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                self.stack.push(
                    lhs.ne(rhs)
                )
            }
            Instruction::Lt => {
                if self.stack.len() < 2 {
                    panic!("Stack underflow");
                }

                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                self.stack.push(
                    lhs.lt(rhs)
                )
            }
            Instruction::Gt => {
                if self.stack.len() < 2 {
                    panic!("Stack underflow");
                }

                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                self.stack.push(
                    lhs.gt(rhs)
                )
            }
            Instruction::LtEq => {
                if self.stack.len() < 2 {
                    panic!("Stack underflow");
                }

                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                self.stack.push(
                    lhs.le(rhs)
                )
            }
            Instruction::GtEq => {
                if self.stack.len() < 2 {
                    panic!("Stack underflow");
                }

                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                self.stack.push(
                    lhs.ge(rhs)
                )
            }
            Instruction::Load(name) => {
                if let Some(val) = self.heap.get(&name) {
                    self.stack.push(val.get_value());
                } else {
                    panic!("Struct not found");
                }
            }
            Instruction::Store(name) => {
                let mut path: VecDeque<&str> = name.split(".").into_iter().collect::<VecDeque<&str>>();
                // todo: struct member store
                let var = path.pop_front().unwrap();
                if let Some(val) = self.heap.get_mut(&var.to_string()) {
                    val.set_value(Value::Value(self.stack.pop().unwrap()));
                } else {
                    panic!("Struct not found");
                }
            }
            Instruction::NewVariable(name, value) => {
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
                    let res = match val {
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
                    let res = match val {
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
                }
                else if let Some(line) = self.functions.get(&function_name) {
                    self.call_stack.push(self.pc);
                    self.pc = line - 1;
                } else {
                    panic!("Function not found");
                }
            }
            Instruction::Push(value) => {
                self.stack.push(value);
            }
            Instruction::Pop => { unimplemented!("Pop"); }
            Instruction::Nop => {}
            Instruction::Return => {
                if let Some(line) = self.call_stack.pop() {
                    self.pc = line - 1;
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
    fn call_builtin(&mut self, name: &str){
        let len = match self.stack.pop().unwrap_or_else(|| panic!("{} Call, Stack underflow", name)) {
            ValueType::Int(int) => int.as_i64(),
            _ => panic!("{} Call, Invalid type", name)
        };
        let args = VecDeque::from_iter(self.stack.drain(self.stack.len() - len as usize..));
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

                unimplemented!();
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

    pub fn get_heap(&self) -> &HashMap<String, Struct> {
        &self.heap
    }
}

