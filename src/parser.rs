use std::collections::VecDeque;
use std::fs;
use std::ops::Add;
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;


#[derive(Debug, Clone, PartialEq)]
pub enum BisayaValue {
    Int(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Identifier(String),
}

impl BisayaValue {
    fn same_type_with(&self, other: &Self) -> bool {
        let type_id;
        match self {
            BisayaValue::Int(_) => { type_id = 1; }
            BisayaValue::Float(_) => { type_id = 2; }
            BisayaValue::String(_) => { type_id = 3; }
            BisayaValue::Boolean(_) => { type_id = 4; }
            BisayaValue::Identifier(_) => { type_id = 5; }
        }

        match other {
            BisayaValue::Int(_) => { type_id == 1 }
            BisayaValue::Float(_) => { type_id == 2 }
            BisayaValue::String(_) => { type_id == 3 }
            BisayaValue::Boolean(_) => { type_id == 4 }
            BisayaValue::Identifier(_) => { type_id == 5 }
        }
    }
}

impl Add for BisayaValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            BisayaValue::Int(left_value) => {
                match rhs {
                    BisayaValue::Int(right_value) => {
                        BisayaValue::Int(left_value + right_value)
                    }
                    _ => { unreachable!("Cannot Add different types") }
                }
            }
            BisayaValue::Float(left_value) => {
                match rhs {
                    BisayaValue::Float(right_value) => {
                        BisayaValue::Float(left_value + right_value)
                    }
                    _ => { unreachable!("Cannot Add different types") }
                }
            }
            BisayaValue::String(left_value) => {
                match rhs {
                    BisayaValue::String(right_value) => {
                        BisayaValue::String(left_value + &*right_value)
                    }
                    _ => { unreachable!("Cannot Add different types") }
                }
            }
            BisayaValue::Boolean(_) => {
                unreachable!("Cannot Add Booleans")
            }
            BisayaValue::Identifier(_) => {
                unreachable!("Cannot Add Identifiers")
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MathOp {
    Add,
    Sub,
    Mul,
    Div,

    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    BinOp {
        left: Box<Expr>,
        op: MathOp,
        right: Box<Expr>
    },
    Constant(BisayaValue)
}

#[derive(Debug, Clone)]
pub enum BisayaNode {
    Program {
        statements: Vec<BisayaNode>
    },
    VariableDeclaration {
        name: String,
        value: Expr
    },
    FunctionCall {
        function_name: String,
        args: Vec<Expr>
    }
}

// IntermediateForOperatorPrecedence
#[derive(Debug, Clone, PartialEq)]
enum IntermediateOP {
    Factor(BisayaValue),
    Parenthesis(Expr),

    Add,
    Sub,
    Mul,
    Div,

    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte
}

impl IntermediateOP {
    fn to_math_op(&self) -> MathOp {
        match self {
            IntermediateOP::Add => { MathOp::Add }
            IntermediateOP::Sub => { MathOp::Sub }
            IntermediateOP::Mul => { MathOp::Mul }
            IntermediateOP::Div => { MathOp::Div }
            IntermediateOP::Eq => { MathOp::Eq  }
            IntermediateOP::Neq => { MathOp::Neq }
            IntermediateOP::Lt => { MathOp::Lt  }
            IntermediateOP::Lte => { MathOp::Lte }
            IntermediateOP::Gt => { MathOp::Gt  }
            IntermediateOP::Gte => { MathOp::Gte }
            _ => { unreachable!() }
        }
    }
}

struct ExpressionParser {
    tokens: VecDeque<IntermediateOP>,
    current_token: IntermediateOP
}

impl ExpressionParser {
    fn new(mut tokens: VecDeque<IntermediateOP>) -> Self {
        let current_token = tokens.pop_front().unwrap();
        Self {
            tokens,
            current_token
        }
    }
    fn parse(tokens: VecDeque<IntermediateOP>) -> Expr {
        Self::new(tokens).run()
    }
    fn run(&mut self) -> Expr {
        let res = self.expr();
        match res {
            Ok(res) => { res }
            Err(_) => { unimplemented!() }
        }
    }
    fn advance(&mut self) -> Result<(), ()> {
        self.current_token = match self.tokens.pop_front() {
            None => {
                return Err(())
            }
            Some(tok) => {
                tok
            }
        };

        return Ok(())
    }

    fn factor(&mut self) -> Result<Expr, ()> {
        if let IntermediateOP::Factor(bv) = self.current_token.clone() {
            if self.tokens.len() == 0 {
                // todo: bugs are likely here

                return Ok(Expr::Constant(bv))
            }
            self.advance()?;
            return Ok(Expr::Constant(bv))
        } else if let IntermediateOP::Parenthesis(expr) = self.current_token.clone(){
            if self.tokens.len() == 0 {
                // todo: bugs are likely here

                return Ok(expr)
            }

            self.advance()?;
            return Ok(expr)
        }
        unreachable!()
    }

    fn term(&mut self) -> Result<Expr, ()> {
        let mut left = self.factor()?;

        while self.current_token == IntermediateOP::Mul
            || self.current_token == IntermediateOP::Div
        {
            let op_tok = self.current_token.clone();
            self.advance()?;
            let right = self.factor()?;
            left = Expr::BinOp {
                left: Box::new(left),
                op: op_tok.to_math_op(),
                right: Box::new(right)
            };
        };

        return Ok(left)
    }

    fn expr(&mut self) -> Result<Expr, ()> {
        let mut left = self.term()?;

        while self.current_token == IntermediateOP::Add
            || self.current_token == IntermediateOP::Sub
        {
            let op_tok = self.current_token.clone();
            match self.advance() {
                Ok(res) => { res }
                Err(_) => { unimplemented!() /* err*/ }
            }
            let right = match self.term(){
                Ok(res) => { res }
                Err(_) => {
                    unimplemented!() /* err*/
                }
            };
            left = Expr::BinOp {
                left: Box::new(left),
                op: op_tok.to_math_op(),
                right: Box::new(right)
            };
        };

        return Ok(left)
    }
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct BisayaParser;

impl BisayaParser {
    fn parse_value_str(pair: Pair<Rule>) -> String {
        match pair.as_rule() {
            Rule::identifier => {
                pair.as_span().as_str().to_string()
            }
            _ => {
                unreachable!()
            }
        }
    }
    fn parse_expr(pair: Pair<Rule>) -> Expr {
        let mut intermediate = VecDeque::new();
        for individual in pair.into_inner(){
            intermediate.push_back(
                match individual.as_rule() {
                    Rule::add => { IntermediateOP::Add }
                    Rule::sub => { IntermediateOP::Sub }
                    Rule::mul => { IntermediateOP::Mul }
                    Rule::div => { IntermediateOP::Div }
                    Rule::factor => {
                        let value = individual.into_inner().next().unwrap();
                        match value.as_rule() {
                            Rule::num => {
                                let num_str = value.as_span().as_str();
                                if num_str.contains("."){
                                    IntermediateOP::Factor(
                                        BisayaValue::Float(num_str.parse::<f32>().unwrap())
                                    )
                                } else {
                                    IntermediateOP::Factor(
                                        BisayaValue::Int(num_str.parse::<i32>().unwrap())
                                    )
                                }

                            }
                            Rule::string_literal => {
                                let mut string_value = value.as_span().as_str().to_string();
                                string_value.pop(); string_value.remove(0);

                                IntermediateOP::Factor(
                                    BisayaValue::String(string_value)
                                )
                            }
                            Rule::identifier => {
                                let mut string_value = value.as_span().as_str().to_string();

                                IntermediateOP::Factor(
                                    BisayaValue::Identifier(string_value)
                                )
                            }
                            _ => unimplemented!()
                        }
                    }
                    Rule::expr => {
                        IntermediateOP::Parenthesis(Self::parse_expr(individual))
                    }
                    _ => {
                        println!("{}", individual);
                        unreachable!()
                    }
                }
            )

        }
        return ExpressionParser::parse(intermediate)
    }
    fn parse_value(pair: Pair<Rule>) -> Option<BisayaNode> {
        match pair.as_rule() {
            Rule::variable_declaration => {
                let mut parts: VecDeque<Pair<Rule>> = pair.into_inner().collect();
                let name = Self::parse_value_str(parts.pop_front().unwrap());
                let value = Self::parse_expr(parts.pop_front().unwrap());
                Some(
                    BisayaNode::VariableDeclaration
                    {
                        name,
                        value
                    }
                )
            }
            Rule::function_call => {
                let mut pairs: VecDeque<Pair<Rule>> = pair.into_inner().collect();
                let name = Self::parse_value_str(pairs.pop_front().unwrap());
                let mut arguments = vec![];
                for arg in pairs {
                    arguments.push(
                        Self::parse_expr(arg)
                    )
                }
                Some(
                    BisayaNode::FunctionCall {
                        function_name: name,
                        args: arguments,
                    }
                )
            }
            Rule::EOI => { None }
            _ => {
                println!("-> {}", pair);
                unreachable!()
            }
        }
    }
    pub fn parse_source(source: &str) -> Result<BisayaNode, Error<Rule>>{
        let file = Self::parse(Rule::program, source)?;
        let mut remaining = vec![];
        for part in file {
            match Self::parse_value(part) {
                None => {
                    break
                }
                Some(val) => {
                    remaining.push(val)
                }
            }
        }
        Ok(
            BisayaNode::Program {
                statements: remaining
            }
        )

    }
    pub fn parse_file(file: &str) -> Result<BisayaNode, Error<Rule>>{
        let file_data = fs::read_to_string(file).unwrap();
        Self::parse_source(&*file_data)

    }
}