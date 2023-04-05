
use std::collections::{HashMap, VecDeque};
use std::fs;
use pest::iterators::{Pair, Pairs};

use crate::parser::Rule;
use crate::virtual_machine::{BoolValue, FloatValue, Instruction, IntValue, StringValue, ValueType};

#[derive(Debug, Clone)]
pub enum EvalValue {
    Reference { val: String },
    IntegerLiteral { val: String },
    Stringliteral { val: String },
    FloatLiteral { val: String },
    BooleanLiteral { val: bool },
    List { val: Vec<ExprAst> },
}

#[derive(Debug, Clone)]
pub enum ExprAst {
    Value {val: EvalValue },
    FunctionCall { name: String, args: Vec<ExprAst> },
    Addition { lhs: Box<ExprAst>, rhs: Box<ExprAst>},
    Subtraction { lhs: Box<ExprAst>, rhs: Box<ExprAst>},
    Division { lhs: Box<ExprAst>, rhs: Box<ExprAst>},
    Multiplication { lhs: Box<ExprAst>, rhs: Box<ExprAst>},
    Eq { lhs: Box<ExprAst>, rhs: Box<ExprAst>},
    Neq { lhs: Box<ExprAst>, rhs: Box<ExprAst>},
    GtEq { lhs: Box<ExprAst>, rhs: Box<ExprAst>},
    LtEq { lhs: Box<ExprAst>, rhs: Box<ExprAst>},
    Gt { lhs: Box<ExprAst>, rhs: Box<ExprAst>},
    Lt { lhs: Box<ExprAst>, rhs: Box<ExprAst>},
}

impl EvalValue {
    pub fn get_int(&self) -> i32 {
        match self {
            Self::IntegerLiteral { val } => { val.parse::<i32>().unwrap() }
            _ => { unreachable!("{:?}", self) }
        }
    }

    pub fn get_float(&self) -> f32 {
        match self {
            Self::FloatLiteral { val } => { val.parse::<f32>().unwrap() }
            _ => { unreachable!() }
        }
    }

    pub fn get_bool(&self) -> bool {
        match self {
            Self::BooleanLiteral { val } => { *val }
            _ => { unreachable!() }
        }
    }

    pub fn get_string(&self) -> String {
        match self {
            Self::Stringliteral { val } => { val.clone() }
            _ => { unreachable!() }
        }
    }

    pub fn get_reference(&self, hash: HashMap<String, EvalValue>) -> EvalValue {
        match self {
            Self::Reference { val } => {
                hash.get(val).ok_or_else(|| { panic!("couldnt find {}", val) }).unwrap().clone()
            }
            _ => { unreachable!() }
        }
    }
}

impl ToString for EvalValue {
    fn to_string(&self) -> String {
        match self {
            Self::Reference { val } => { val.clone() }
            Self::IntegerLiteral { val } => { val.clone() }
            Self::Stringliteral { val } => { val.clone() }
            Self::FloatLiteral { val } => { val.clone() }
            Self::BooleanLiteral { val } => { val.to_string() }
            EvalValue::List { val } => {
                let mut s = "[".to_string();
                for x in val {
                    s += &*x.to_string();
                    s += ", ";
                }
                s.pop();
                s.pop();
                s
            }
        }
    }
}

impl ExprAst {
    fn to_stack_item_value(&self) -> StackItems {
        match self {
            Self::Value { val } => { return StackItems::Value(val.clone()) }
            Self::FunctionCall { name, args } => {
                let mut args = args.iter().map(|x| x.to_stack_item_value()).collect::<Vec<StackItems>>();
                return StackItems::FunctionCall(name.clone(), args);
            }
            _ => {}
        }

        let op = match self {
            ExprAst::Value { .. } => { unreachable!()}
            ExprAst::FunctionCall { .. } => { unreachable!() }
            ExprAst::Addition { .. } => { 0 }
            ExprAst::Subtraction { .. } => { 1 }
            ExprAst::Division { .. } => { 2 }
            ExprAst::Multiplication { .. } => { 3 }
            ExprAst::Eq { .. } => { 4 }
            ExprAst::Neq { .. } => { 5 }
            ExprAst::GtEq { .. } => { 6 }
            ExprAst::LtEq { .. } => { 7 }
            ExprAst::Gt { .. } => { 8 }
            ExprAst::Lt { .. } => { 9 }
        };

        match self {
            Self::Division { lhs, rhs } |
            Self::Multiplication { lhs, rhs} |
            Self::Subtraction { lhs, rhs} |
            Self::Addition { lhs, rhs}
            => {
                StackItems::OperationWithValues(
                    op,
                    Box::new(lhs.to_stack_item_value()),
                    Box::new(rhs.to_stack_item_value()),
                )
            }
            _ => unreachable!()
        }
    }
    fn to_string(&self) -> String {
        match self {
            Self::Value { val } => { val.to_string() }
            Self::FunctionCall { name, args } => {
                let mut s = name.clone();
                s += "(";
                for x in args {
                    s += &*x.to_string();
                    s += ", ";
                }
                s.pop();
                s.pop();
                s += ")";
                s
            }
            Self::Addition { lhs, rhs } => {
                let mut s = lhs.to_string();
                s += " + ";
                s += &*rhs.to_string();
                s
            }
            Self::Subtraction { lhs, rhs } => {
                let mut s = lhs.to_string();
                s += " - ";
                s += &*rhs.to_string();
                s
            }
            Self::Division { lhs, rhs } => {
                let mut s = lhs.to_string();
                s += " / ";
                s += &*rhs.to_string();
                s
            }
            Self::Multiplication { lhs, rhs } => {
                let mut s = lhs.to_string();
                s += " * ";
                s += &*rhs.to_string();
                s
            }
            Self::Eq { lhs, rhs } => {
                let mut s = lhs.to_string();
                s += " == ";
                s += &*rhs.to_string();
                s
            }
            Self::Neq { lhs, rhs } => {
                let mut s = lhs.to_string();
                s += " != ";
                s += &*rhs.to_string();
                s
            }
            Self::GtEq { lhs, rhs } => {
                let mut s = lhs.to_string();
                s += " >= ";
                s += &*rhs.to_string();
                s
            }
            Self::LtEq { lhs, rhs } => {
                let mut s = lhs.to_string();
                s += " <= ";
                s += &*rhs.to_string();
                s
            }
            Self::Gt { lhs, rhs } => {
                let mut s = lhs.to_string();
                s += " > ";
                s += &*rhs.to_string();
                s
            }
            Self::Lt { lhs, rhs } => {
                let mut s = lhs.to_string();
                s += " < ";
                s += &*rhs.to_string();
                s
            }
        }
    }
}


#[derive(Debug)]
enum StackItems{
    Value(EvalValue),
    ValueBool(bool),
    Operation(u8),
    FunctionCall(String, Vec<StackItems>),
    OperationWithValues(u8, Box<StackItems>, Box<StackItems>)
}

impl StackItems {
    fn to_node(&self) -> ExprAst {
        match self {
            Self::Value(val) => {
                ExprAst::Value { val: val.clone() }
            }
            Self::FunctionCall(name, args) => {
                let args = args.iter().map(|x| x.to_node()).collect::<Vec<ExprAst>>();
                ExprAst::FunctionCall { name: name.clone(), args }
            }
            Self::OperationWithValues( op, lhs, rhs) => {
                let lhs = Box::new(lhs.to_node());
                let rhs = Box::new(rhs.to_node());

                match op {
                    0 => { ExprAst::Addition {lhs, rhs}}
                    1 => { ExprAst::Subtraction {lhs, rhs}}
                    2 => { ExprAst::Division {lhs, rhs}}
                    3 => { ExprAst::Multiplication {lhs, rhs}}
                    4 => { ExprAst::Eq {lhs, rhs}}
                    5 => { ExprAst::Neq {lhs, rhs}}
                    6 => { ExprAst::GtEq {lhs, rhs}}
                    7 => { ExprAst::LtEq {lhs, rhs}}
                    8 => { ExprAst::Gt {lhs, rhs}}
                    9 => { ExprAst::Lt {lhs, rhs}}
                    _ => unreachable!()
                }
            }
            _ => unreachable!("Stack item couldnt be converted into a node")
        }
    }
}

fn str_to_op_no(op: &str) -> u8 {
    match op {
        "+" => { 0 }
        "-" => { 1 }
        "/" => { 2 }
        "*" => { 3 }
        "==" => { 4 }
        "!=" => { 5 }
        ">=" => { 6 }
        "<=" => { 7 }
        ">" => { 8 }
        "<" => { 9 }
        _ => { unreachable!() }
    }
}

fn parse_str(string: &str) -> String {
    let mut index = 0;
    let str_len = string.len();
    let str_as_chars = string.chars().collect::<Vec<char>>();
    let mut new_string = String::new();
    let chars_with_escape = vec![('n', '\n'), ('t', '\t'), ('r', '\r'), ('\\', '\\'), ('0', '\0'), ('"', '"' )];
    'outer: while  str_len > index {
        for char in &chars_with_escape {
            if str_as_chars[index] == char.0 {
                if index <= 2 {
                    index += 1;
                    new_string.push(char.0);
                    continue 'outer
                }

                if str_as_chars[index - 1] == '\\' && str_as_chars[index - 2] != '\\' {
                    index += 1;
                    new_string.pop();
                    new_string.push(char.1);
                    continue 'outer
                }
            }
        }
        new_string.push(str_as_chars[index]);
        index += 1;
    }

    new_string
}

fn vec_deque_stack_items_to_number(mut items: VecDeque<StackItems>) -> ExprAst {
    let mut left = items.pop_front().expect("err no items").to_node();
    loop {
        let op = match items.pop_front().expect("err no items"){
            StackItems::Operation(op) => { op },
            _ => unreachable!()
        };
        let right = items.pop_front().expect("err no items");
        left = StackItems::OperationWithValues(
            op,
            Box::new(left.to_stack_item_value()),
            Box::new(right)
        ).to_node();
        if items.len() == 0 { break }
    }
    left
}

fn rule_expr_to_eval_expr(rule: Pair<Rule>) -> ExprAst
{
    match rule.as_rule() {
        Rule::expr => {
            let mut pairs = rule.into_inner().collect::<VecDeque<Pair<Rule>>>();
            let res = rule_expr_to_eval_expr(pairs.pop_back().unwrap());
            res
        }
        Rule::bare_expr => {
            let mut pairs = rule.into_inner().collect::<VecDeque<Pair<Rule>>>();
            if pairs.len() == 1 {
                return rule_expr_to_eval_expr(pairs.pop_back().unwrap())
            }

            let mut items = VecDeque::new();
            for pair in pairs {
                match pair.as_rule() {
                    Rule::eq_ops => {
                        items.push_back(
                            StackItems::Operation(str_to_op_no(pair.as_span().as_str()))
                        )
                    }
                    _ => {
                        let res = rule_expr_to_eval_expr(pair);
                        items.push_back(res.to_stack_item_value())
                    }
                }
            }
            vec_deque_stack_items_to_number(items)
        }
        Rule::sum => {
            let mut pairs = rule.into_inner().collect::<VecDeque<Pair<Rule>>>();
            if pairs.len() == 1 {
                return rule_expr_to_eval_expr(pairs.pop_back().unwrap())
            }

            let mut items = VecDeque::new();
            for pair in pairs {
                match pair.as_rule() {
                    Rule::sum_ops => {
                        items.push_back(
                            StackItems::Operation(str_to_op_no(pair.as_span().as_str()))
                        )
                    }
                    _ => {
                        // let intermediate = pair.into_inner().collect::<Vec<Pair<Rule>>>().pop().unwrap();
                        let res = rule_expr_to_eval_expr(pair);
                        items.push_back(res.to_stack_item_value())
                    }
                }
            }
            vec_deque_stack_items_to_number(items)
        }

        Rule::product => {
            let mut pairs = rule.into_inner().collect::<VecDeque<Pair<Rule>>>();
            if pairs.len() == 1 {
                return rule_expr_to_eval_expr(pairs.pop_back().unwrap())
            }

            let mut items = VecDeque::new();
            for pair in pairs {
                match pair.as_rule() {
                    Rule::prod_ops => {
                        items.push_back(
                            StackItems::Operation(str_to_op_no(pair.as_span().as_str()))
                        )
                    }
                    _ => {
                        // let intermediate = pair.into_inner().collect::<Vec<Pair<Rule>>>().pop().unwrap();
                        let res = rule_expr_to_eval_expr(pair);
                        items.push_back(res.to_stack_item_value())
                    }
                }
            }
            vec_deque_stack_items_to_number(items)
        }
        Rule::term => {
            rule_expr_to_eval_expr(rule.into_inner().collect::<Vec<Pair<Rule>>>().pop().unwrap())
        }
        Rule::integer => {
            ExprAst::Value {
                val: EvalValue::IntegerLiteral {
                    val: rule.as_span().as_str().to_string()
                }
            }
        }
        Rule::string => {
            ExprAst::Value {
                val: EvalValue::Stringliteral {
                    val: {
                        let str = parse_str(rule.as_span().as_str());
                        str[1..str.len() - 1].to_string()
                    }
                }
            }
        }
        Rule::float => {
            ExprAst::Value {
                val: EvalValue::FloatLiteral {
                    val: rule.as_span().as_str().to_string()
                }
            }
        }
        Rule::boolean => {
            ExprAst::Value {
                val: EvalValue::BooleanLiteral {
                    val: rule.as_span().as_str() == "true"
                }
            }
        }
        Rule::identifier => {
            ExprAst::Value {
                val: EvalValue::Reference {
                    val: rule.as_span().as_str().to_string()
                }
            }
        }
        Rule::function_call => {
            let mut pairs = rule.into_inner().collect::<VecDeque<Pair<Rule>>>();
            let name = pairs.pop_front().unwrap().as_span().as_str().to_string();
            ExprAst::FunctionCall {
                name,
                args: pairs.iter().map(|pair| rule_expr_to_eval_expr(pair.clone())).collect()
            }
        }
        Rule::list => {
            let mut pairs = rule.into_inner().collect::<VecDeque<Pair<Rule>>>();
            let mut items = vec![];
            for pair in pairs {
                match pair.as_rule() {
                    _ => {
                        items.push(rule_expr_to_eval_expr(pair))
                    }
                }
            }
            ExprAst::Value {
                val: EvalValue::List {
                    val: items
                }
            }
        }
        _ => unreachable!("{:?}", rule)
    }
}

pub fn parse_expression(rule: Pair<Rule>) -> ExprAst {
    rule_expr_to_eval_expr(rule)
}

pub fn parse_expressions(rules: Pairs<Rule>) -> Vec<ExprAst>{
    let mut nums = vec![];
    for rule in rules {
        match rule.as_rule() {
            Rule::EOI => { break }
            Rule::expr => {
                nums.push(
                    rule_expr_to_eval_expr(rule)
                )
            }
            _ => unreachable!("{:?}", rule)
        }
    }
    nums
}