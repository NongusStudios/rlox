use crate::{
    scanner::{
        Scanner, Token, TokenType
    }, vm::Chunk
};

pub struct Compiler<'a> {
    chunk: Option<Chunk>,
    scanner: Option<Scanner<'a>>,
    previous: Option<Token<'a>>,
    current:  Option<Token<'a>>,
}

impl<'a> Compiler<'a> {
    pub fn new() -> Self {
        Self {
            chunk: None,
            scanner: None,
            previous: None,
            current: None,
        }
    }

    fn advance(&mut self) -> Result<(), String> {
        let scanner = self.scanner.as_mut().unwrap();

        self.previous = self.current;
        let token = scanner.scan_token();
        if let Some(t) = token {
            self.current = Some(t);
            Ok(())
        } else {
            Err(format!("Error at line {}: {}", scanner.line, scanner.err))
        }
    }

    pub fn compile(&mut self, source: &'a str) -> Result<Chunk, String> {
        self.scanner = Some(Scanner::new(source));
        self.chunk = Some(Chunk::new());

        self.scanner = None;
        let chunk = self.chunk.take().expect("How'd the chunk disappear, something really bad happend ig?");
        Ok(chunk)
    }
}
