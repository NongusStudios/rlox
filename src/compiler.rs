use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    scanner::{
        Scanner, Token, TokenType
    }, vm::{Chunk, Op, Value}
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
enum Precedence {
    None,
    Assignment, // =
    Or, // or
    And, // and
    Equality, // == !=
    Comparison, // < > <= >=
    Term, // + -
    Factor, // * /
    Unary, // ! -
    Call, // . ()
    Primary
}

fn next_precedence(p: Precedence) -> Option<Precedence> {
    let iter = Precedence::iter();
    iter.skip_while(|&pr| pr != p).nth(1)
}

pub struct Compiler<'a> {
    chunk:    &'a mut Chunk,
    scanner:  Scanner<'a>,
    previous: Option<Token<'a>>,
    current:  Option<Token<'a>>,
}

type ParseFn<'a> = fn(&mut Compiler<'a>) -> Result<(), String>;
struct ParseRule<'a> {
    prefix: Option<ParseFn<'a>>,
    infix:  Option<ParseFn<'a>>,
    precedence: Precedence,
} impl<'a> ParseRule<'a> {
    fn new(prefix: Option<ParseFn<'a>>, infix: Option<ParseFn<'a>>, p: Precedence) -> Self {
        Self {
            prefix,
            infix,
            precedence: p,
        }
    }

    fn empty() -> Self {
        Self {
            prefix: None,
            infix:  None,
            precedence: Precedence::None,
        }
    }

    fn is_empty(&self) -> bool {
        self.prefix.is_none() && self.infix.is_none()
    }

    // Get parse rule based on the token type
    fn get(t: TokenType) -> Self {
        match t {
            TokenType::LParen => ParseRule::new(Some(Compiler::grouping), None,                   Precedence::None),
            TokenType::Minus  => ParseRule::new(Some(Compiler::unary),    Some(Compiler::binary), Precedence::Term),
            TokenType::Plus   => ParseRule::new(None,                     Some(Compiler::binary), Precedence::Term),
            TokenType::Slash  => ParseRule::new(None,                     Some(Compiler::binary), Precedence::Factor),
            TokenType::Star   => ParseRule::new(None,                     Some(Compiler::binary), Precedence::Factor),
            TokenType::Number => ParseRule::new(Some(Compiler::number),   None,                   Precedence::None),
            
            _ => ParseRule::empty(),
        }
    }
}

impl<'a> Compiler<'a> {
    pub fn new(chunk: &'a mut Chunk, source: &'a str) -> Self {
        Self {
            chunk,
            scanner: Scanner::new(source),
            previous: None,
            current: None,
        }
    }
    
    fn emit_constant(&mut self, value: Value) {
        self.chunk.push_constant(value, self.scanner.line);
    }

    fn emit_op(&mut self, op: Op) {
        self.chunk.push_operation(op, self.scanner.line);
    } 

    fn consume(&mut self) -> Result<(), String> {
        self.previous = self.current;
        if let Some(token) = self.scanner.scan_token() {
            self.current = Some(token);
            Ok(())
        } else {
            Err(self.scanner.err.clone())
        }
    }

    fn match_and_consume(&mut self, t: TokenType) -> Result<bool, String> {
        if self.current.unwrap().t_type == t {
            if let Err(err) = self.consume() {
                Err(err)
            } else { Ok(true) }
        } else { Ok(false) }
    }
    
    fn parse_precedence(&mut self, p: Precedence) -> Result<(), String> {
        self.consume()?;
        // Expect first token to have a prefix expression, if not compiler error.
        let rule = ParseRule::get(self.previous.unwrap().t_type);
        if let Some(prefix) = rule.prefix {
            // Run found prefix expression.
            prefix(self)?;
            
            // Keep compiling tokens until a higher precedence is reached.
            while p <= ParseRule::get(self.current.unwrap().t_type).precedence {
                self.consume()?;
                let rule = ParseRule::get(self.previous.unwrap().t_type);
                if let Some(infix) = rule.infix {
                    infix(self)?;
                } else { break; }
            }

            Ok(())
        } else {
            Err("expected expression.".to_string())
        }
    }

    fn expression(&mut self) -> Result<(), String> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn number(&mut self) -> Result<(), String> {
        if let Ok(value) = self.previous.unwrap().slice.parse() {
            self.emit_constant(value);
            Ok(())
        } else {
            Err("expected a number literal.".to_string())
        }
    }

    fn unary(&mut self) -> Result<(), String> {
        let op_type = self.previous.unwrap().t_type;
        
        self.parse_precedence(Precedence::Unary)?;

        match op_type {
            TokenType::Minus => { self.emit_op(Op::Negate); Ok(()) }
            _ => Err("invalid unary operation.".to_string())
        }
    }
    
    fn binary(&mut self) -> Result<(), String> {
        let op_type = self.previous.unwrap().t_type;
        
        let rule = ParseRule::get(op_type);
        self.parse_precedence(
            next_precedence(rule.precedence)
                .unwrap_or(Precedence::iter().last().unwrap())
        )?;
        
        match op_type {
            TokenType::Plus => self.emit_op(Op::Add),
            TokenType::Minus => self.emit_op(Op::Sub),
            TokenType::Star => self.emit_op(Op::Mul),
            TokenType::Slash => self.emit_op(Op::Div),
            _ => return Err("invalid operand for binary expression.".to_string()),
        }

        Ok(())
    }

    fn grouping(&mut self) -> Result<(), String> {
        if let Err(e) = self.expression() {
            return Err(e);
        }

        match self.match_and_consume(TokenType::RParen) {
            Ok(result) => if result { Ok(()) }
                          else      { Err("expected ')' after expression.".to_string()) }
            Err(e) => Err(e),
        }
    }
}

pub fn compile(source: &str) -> Result<Chunk, String> {
    let mut chunk = Chunk::new();
    let mut compiler  = Compiler::new(&mut chunk, source);
   
    // Pump the compiler.
    if let Err(e) = compiler.consume() {
        return Err(format!("Error at line {}: {}", compiler.scanner.line, e));
    }
    
    // Compile the source.
    if let Err(e) = compiler.expression() {
        return Err(format!("Error at line {}: {}", compiler.scanner.line, e));
    }

    // Make sure Eof has been reached.
    match compiler.match_and_consume(TokenType::Eof) {
        Ok(result) => if !result { return Err("expected end of expression.".to_string()); }
        Err(e)     => return Err(format!("Error ran out of tokens: {}", e)),
    }
    
    compiler.emit_op(Op::Return);

    // Emit compiled chunk
    Ok(dbg!(chunk))
}
