use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Number(f64),
    Bool(bool),
    // Holds index to string in chunk memory.
    Str(Rc<String>),
}

impl Value {
    pub fn from_str(s: &str) -> Self {
        Self::Str(Rc::new(s.to_string()))
    }

    pub fn add(self, rhs: Value) -> Result<Value, String> {
        let value = match &self {
            Self::Nil => Some(Self::Nil),
            Self::Number(l) => if let Self::Number(r) = rhs {
                Some(Self::Number(l + r))
            } else { None }
            Self::Str(l) => if let Self::Str(r) = &rhs {
                Some(Self::Str(Rc::new(l.as_ref().clone() + r.as_ref())))
            } else { None }
            _ => None,
        };

        if let Some(v) = value {
            Ok(v)
        } else { Err("type mismatch or invalid '+' operation.".to_string()) }
    }

    pub fn sub(self, rhs: Value) -> Result<Value, String> {
        let value = match self {
            Self::Nil => Some(Self::Nil),
            Self::Number(l) => if let Self::Number(r) = rhs {
                Some(Self::Number(l - r))
            } else { None }
            _ => None, 
        };

        if let Some(v) = value {
            Ok(v)
        } else { Err("type mismatch or invalid '-' operation.".to_string()) }
    }

    pub fn mul(self, rhs: Value) -> Result<Value, String> {
        let value = match self {
            Self::Nil => Some(Self::Nil),
            Self::Number(l) => if let Self::Number(r) = rhs {
                Some(Self::Number(l * r))
            } else { None }
            _ => None, 
        };

        if let Some(v) = value {
            Ok(v)
        } else { Err("type mismatch or invalid '*' operation.".to_string()) }
    }

    pub fn div(self, rhs: Value) -> Result<Value, String> {
        let value = match self {
            Self::Nil => Some(Self::Nil),
            Self::Number(l) => if let Self::Number(r) = rhs {
                Some(Self::Number(l / r))
            } else { None }
            _ => None, 
        };

        if let Some(v) = value {
            Ok(v)
        } else { Err("type mismatch or invalid '/' operation.".to_string()) }
    }

    pub fn unary(self, op: char) -> Option<Value> {
        match self {
            Self::Bool(b) => match op {
                '!' => Some(Self::Bool(!b)),
                _   => None,
            }
            Self::Number(n) => match op {
                '-' => Some(Self::Number(-n)),
                _   => None,
            }
            _ => None,
        }
    }

    pub fn compare(self, rhs: Value, op: &str) -> Result<Value, String> {
        let value = match self {
            Self::Number(l) => if let Self::Number(r) = rhs {
                Some(Self::Bool(match op {
                    "<" =>  l <  r,
                    "<=" => l <= r,
                    ">" =>  l >  r,
                    ">=" => l >= r,
                    _ => false,
                }))
            } else { None }
            _ => None
        };
        
        if let Some(v) = value {
            Ok(v)
        } else { Err(format!("only numerical types are comparable, near {}", op)) }
    }

    pub fn and(self, rhs: Value) -> Result<Value, String> {
        let value = match self {
            Self::Bool(l) => if let Self::Bool(r) = rhs {
                Some(Self::Bool(l && r))
            } else { None }
            _ => None,
        };
        
        if let Some(v) = value {
            Ok(v)
        } else { Err("only boolean values can be used for 'and' operation.".to_string())}
    }

    pub fn or(self, rhs: Value) -> Result<Value, String> {
        let value = match self {
            Self::Bool(l) => if let Self::Bool(r) = rhs {
                Some(Self::Bool(l || r))
            } else { None }
            _ => None,
        };
        
        if let Some(v) = value {
            Ok(v)
        } else { Err("only boolean values can be used for 'and' operation.".to_string())}
    }

    pub fn print(self) {
        match self {
            Self::Bool(b) => print!("{}", b),
            Self::Number(n) => print!("{}", n),
            Self::Str(s) => print!("{}", s),
            Self::Nil => print!("nil"),
        }
    }
}
