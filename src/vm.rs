
use std::{collections::VecDeque, vec::Vec};

use crate::{
    compiler, util::KeyedArray, value::Value,
};


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Op {
    LoadConst(usize),
    True,
    False,
    Nil,
    
    Not,
    Negate,
    
    Equal,
    NotEqual,
    GreaterThan,
    GreaterEq,
    LessThan,
    LessEq,

    Add,
    Sub,
    Mul,
    Div,
    
    Return,
}

#[derive(Debug, Clone)]
pub enum StaticMem {
    Str(String),
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
            if let Op::LoadConst(idx) = op {
                write!(f, "[{:04}] - {:?} - {:?}\n", line, op, self.constants[*idx])?;
            } else {
                write!(f, "[{:04}] - {:?}\n", line, op)?;
            }
        }
        Ok(())
    }
}

impl Chunk {
    const SIZE: usize = 128;

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
    stack: VecDeque<Value>,
    chunk: Option<Chunk>,
    line: usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: VecDeque::new(),
            chunk: None,
            line: 0,
        }
    }
    
    pub fn interpret(&mut self, src: &str) -> Result<Value, String> {
        self.chunk = Some(compiler::compile(src)?);
        match self.execute_loaded_chunk() {
            Ok(v)  => Ok(v),
            Err(e) => Err(format!("Runtime error, at line {}: {}", self.line, e))
        }
    }

    fn binary_op(op: Op, lhs: Value, rhs: Value) -> Result<Value, String> {
        match op {
            Op::Add => lhs.add(rhs),
            Op::Sub => lhs.sub(rhs),
            Op::Mul => lhs.mul(rhs),
            Op::Div => lhs.div(rhs),

            Op::Equal     => Ok(Value::Bool(lhs == rhs)),
            Op::NotEqual  => Ok(Value::Bool(lhs != rhs)),
            Op::GreaterThan   => lhs.compare(rhs, ">"), 
            Op::GreaterEq => lhs.compare(rhs, ">="),
            Op::LessThan      => lhs.compare(rhs, "<"),
            Op::LessEq    => lhs.compare(rhs, "<="),
            _ => Err("invalid binary operation.".to_string()),
        }
    }

    fn unary_op(op: Op, v: Value) -> Option<Value> {
        match op {
            Op::Negate => v.unary('-'), 
            Op::Not    => v.unary('!'),
            _ => None,
        }
    }
    
    pub fn load_chunk(&mut self, chunk: Chunk) {
        self.chunk = Some(chunk);
    }
    
    pub fn execute_loaded_chunk(&mut self) -> Result<Value, String> {
        if self.chunk.is_none() { return Err("no chunk has been loaded.".to_string()); }
        
        if let Some(chunk) = &mut self.chunk {
            for (op, line) in &chunk.code {
                self.line = *line;
                match op {
                    // Push
                    Op::LoadConst(idx) => {
                        let value = chunk.constants[*idx].clone();
                        chunk.constants.remove(*idx); // Once a constant is consumed it can be freed (for now?).
                        self.stack.push_back(value);
                    },
                    Op::True => self.stack.push_back(Value::Bool(true)),
                    Op::False => self.stack.push_back(Value::Bool(false)),
                    Op::Nil => self.stack.push_back(Value::Nil),

                    // Pop
                    Op::Add     | Op::Sub       | Op::Mul | Op::Div |
                    Op::Equal   | Op::NotEqual  |
                    Op::GreaterThan | Op::GreaterEq |
                    Op::LessThan    | Op::LessEq => {
                        let rhs = self.stack.pop_back().expect("Expected item on the stack.");
                        let lhs = self.stack.pop_back().expect("Expected item on the stack.");
                        self.stack.push_back(Self::binary_op(*op, lhs, rhs)?);
                    }
                    Op::Negate | Op::Not => {
                        let v = self.stack.pop_back().expect("Expected item on the stack.");
                        if let Some(v) = Self::unary_op(*op, v) {
                            self.stack.push_back(v);
                        } else { return Err("type mismatch on unary operation.".to_string()); }
                    }
                    Op::Return => {
                        // Temporary return behaviour
                        return Ok(self.stack.pop_back().unwrap_or(Value::Nil));
                    }
                }
            }
        }

        self.chunk = None;
        
        return Ok(Value::Nil);
    }
}
