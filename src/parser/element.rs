#![allow(dead_code)]

use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Element {
    pub value: Value,
    pub value_type: ValueType,
    pub childlen: Vec<Element>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Value {
    Identifier(String),     // start with alphabetic char; variable or
    Integer(i64),           // 1234
    String(String),
    Operator(String,i32),   // infix notation, whitch required left and right element (operator string, priority)
    Formula(String),
    Scope(Box<Value>),
    Brackets(String),
}

lazy_static! {
    pub static ref OPERATORS: HashMap<String, Value> = {
        let mut m = HashMap::new();
        m.insert("*".to_string(), Value::Operator("*".to_string(), 20));
        m.insert("/".to_string(), Value::Operator("/".to_string(), 20));
        m.insert("+".to_string(), Value::Operator("+".to_string(), 10));
        m.insert("-".to_string(), Value::Operator("-".to_string(), 10));
        m
    };
}

pub fn get_operator(ope: &String) -> Value {
    return OPERATORS[ope].clone()
}

pub fn get_end_bracket(bra: &str) -> Option<String> {
    match bra {
        "(" => Some(")".to_string()),
        "{" => Some("}".to_string()),
        _ => None,
    }
}

#[derive(Debug)]
pub enum ValueType {
    Inference,
}
