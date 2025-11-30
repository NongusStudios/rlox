
use std::{collections::VecDeque, vec::Vec};

use crate::{
    compiler, util::KeyedArray
};

pub type Value = f64;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Op {
    LoadConst(usize),
    Negate,
    Add,
    Sub,
    Mul,
    Div,
    Return,
}

#[derive(Clone)]
pub struct Chunk {
    pub code: Vec<(Op, usize)>,
    constants: KeyedArray<Value>,
}

impl std::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n")?;
        for (op, line) in &self.code {
            write!(f, "[{:04}] - {:?}\n", line, op)?;
        }
        Ok(())
    }
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

    pub fn push_constant(&mut self, value: Value, line: usize) -> &mut Self {
        let idx = self.add_constant(value);

        self.code.push((Op::LoadConst(idx), line));
        self
    }

    pub fn push_operation(&mut self, op: Op, line: usize) -> &mut Self {
        self.code.push((op, line));
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
        self.chunk = Some(compiler::compile(src)?);
        self.execute_loaded_chunk();
        Ok(())
    }

    fn binary_op(op: Op, lhs: Value, rhs: Value) -> Option<Value> {
        match op {
            Op::Add => Some(lhs + rhs),
            Op::Sub => Some(lhs - rhs),
            Op::Mul => Some(lhs * rhs),
            Op::Div => Some(lhs / rhs),
            _ => None,
        }
    }

    fn unary_op(op: Op, v: Value) -> Option<Value> {
        match op {
            Op::Negate => Some(-v), 
            _ => None,
        }
    }

    fn execute_loaded_chunk(&mut self) {
        if self.chunk.is_none() { return }
        
        if let Some(chunk) = &self.chunk {
            for (op, _line) in &chunk.code {
                match op {
                    Op::LoadConst(idx) => {
                        let value = chunk.constants[*idx];
                        self.stack.push_back(value);
                    },
                    Op::Add | Op::Sub | Op::Mul | Op::Div => {
                        let rhs = self.stack.pop_back().expect("Expected item on the stack.");
                        let lhs = self.stack.pop_back().expect("Expected item on the stack.");
                        self.stack.push_back(Self::binary_op(*op, lhs, rhs).unwrap());
                    }
                    Op::Negate => {
                        let v = self.stack.pop_back().expect("Expecte item on the stack.");
                        self.stack.push_back(Self::unary_op(*op, v).unwrap());
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
