
use std::{collections::VecDeque, vec::Vec};

use crate::{
    util::{
        KeyedArray,
    }
};

type Value = f64;

#[derive(Clone, Copy, Debug)]
pub enum Op {
    LoadConst(usize),
    Add,
    Sub,
    Mul,
    Div,
    Return,
}

#[derive(Clone)]
pub struct Chunk {
    code: Vec<Op>,
    constants: KeyedArray<Value>,
}

impl Chunk {
    const SIZE: usize = 512;

    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: KeyedArray::new(Self::SIZE),
        }
    }
    
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value) 
    }

    pub fn push_constant(&mut self, value: Value) -> &mut Self {
        let idx = self.add_constant(value);

        self.code.push(Op::LoadConst(idx));
        self
    }

    pub fn push_operation(&mut self, op: Op) -> &mut Self {
        self.code.push(op);
        self
    }
}

pub struct VM {
    stack: VecDeque<f64>,
    chunk: Option<Chunk>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: VecDeque::new(),
            chunk: None,
        }
    }
    
    pub fn interpret(&mut self, src: &str) -> Result<(), String> {
        todo!()
    }

    fn binary_op(op: Op, lhs: Value, rhs: Value) -> Value {
        match op {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
            _ => Value::default(),
        }
    }

    fn execute_loaded_chunk(&mut self) {
        if self.chunk.is_none() { return }
        
        if let Some(chunk) = &self.chunk {
            for op in &chunk.code {
                match op {
                    Op::LoadConst(idx) => {
                        let value = chunk.constants[*idx];
                        self.stack.push_back(value);
                    },
                    Op::Add | Op::Sub | Op::Mul | Op::Div => {
                        let rhs = self.stack.pop_back().expect("Expected item in the stack.");
                        let lhs = self.stack.pop_back().expect("Expected item in the stack.");
                        self.stack.push_back(Self::binary_op(*op, lhs, rhs));
                    }
                    Op::Return => {
                        println!("return: {}", self.stack.pop_back().unwrap_or(Value::default()));
                    }
                }
            }
        }

        self.chunk = None;
    }
}
