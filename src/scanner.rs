use phf::phf_map;

pub struct Scanner<'a> {
    source: &'a str,
    pub line:   usize,
    start:  usize,
    end:    usize,
    pub err:    String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    LParen, RParen,
    LBrace, RBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,
    Bang, BangEqual,
    Equal, Equate,
    GreaterThan, GreaterEq,
    LessThan, LessEq,
    
    // Literals
    Identifier, Str, Number,

    // Keywords
    And, Struct, Else, False,
    For, Fn, If, Nil, Or,
    Print, Return, Super, StructSelf,
    True, Let,
    
    Eof,
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub t_type: TokenType,
    pub slice: &'a str,
    pub line:  usize,
}

impl<'a> Token<'a> {
    fn new(t: TokenType, slice: &'a str, line: usize) -> Self {
        Self {
            t_type: t,
            slice,
            line,
        }
    }
}

impl<'a> Scanner<'a> {
    const KEYWORDS: phf::Map<&'static str, TokenType> = phf_map!  {
        "and"    => TokenType::And,
        "struct" => TokenType::Struct,
        "else"   => TokenType::Else,
        "false"  => TokenType::False,
        "for"    => TokenType::For,
        "fn"     => TokenType::Fn,
        "if"     => TokenType::If,
        "nil"   => TokenType::Nil,
        "or"     => TokenType::Or,
        "print"  => TokenType::Print,
        "return" => TokenType::Return,
        "super"  => TokenType::Super,
        "self"   => TokenType::StructSelf,
        "true"   => TokenType::True,
        "let"    => TokenType::Let,
    };

    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            line:  1,

            start: 0,
            end:   1,
            err:   String::new(),
        } 
    }
    
    // Gets current slices encompassed by start - end
    pub fn get_slice(&self) -> Option<&'a str> {
        let (start, _) = self.source.char_indices().nth(self.start)?;
        let (end, _)   = self.source.char_indices().nth(self.end)
            .unwrap_or((self.source.len(), '\0'));
         
        self.source.get(start..end)
    }

    fn get_current(&self) -> Option<char> {
        self.source.chars().nth(self.end - 1)
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.end)
    }

    fn consume(&mut self) {
        self.end += 1;
    }

    // Peeks next char, if match consumes it
    fn match_and_consume(&mut self, c: char) -> bool {
        if let Some(peek) = self.peek() && peek == c {
            self.consume();
            true
        } else { false }
    }

    // Peeks next char and consumes if condition is true
    fn consume_if(&mut self, f: impl Fn(char) -> bool) -> bool {
        if let Some(peek) = self.peek() && f(peek) {
            self.consume();
            true
        } else { false }
    }

    fn consume_till(&mut self, f: impl Fn(char) -> bool) -> bool {
        loop {
            if let Some(peek) = self.peek() {
                if f(peek) { return true }
                self.consume();
            } else { return false }
        }
    }

    fn consume_till_mut(&mut self, f: &mut impl FnMut(char) -> bool) -> bool {
        loop {
            if let Some(peek) = self.peek() {
                if f(peek) { return true }
                self.consume();
            } else { return false }
        }
    }

    fn next_range(&mut self) {
        self.start = self.end;
        self.end  += 1;
    }

    fn emit_token(&mut self, t: TokenType) -> Option<Token<'a>> {
        if let Some(slice) = self.get_slice() {
            self.next_range();
            Some(Token::new(t,
                slice,
                self.line
            ))
        } else { None }
    }

    pub fn scan_token(&mut self) -> Option<Token<'a>> {
        // Skip whitespace
        {
            let mut line_change: usize = 0;
            if self.get_current().unwrap_or('\0').is_whitespace() {
                self.consume_till_mut(&mut |c| {
                    if c == '\n' {
                        line_change += 1;
                    }
                    !c.is_whitespace()
                });
                self.next_range();
            }
            self.line += line_change;
        }
        // Skip comments
        if self.get_current().unwrap_or('\0') == '#' {
            self.consume_till(|c| {
                c == '\n'
            });
            self.next_range();
        }

        match self.get_current() {
            None => Some(Token::new(TokenType::Eof, "eof", 0)),
            Some(curr) => match curr {
                '(' => self.emit_token(TokenType::LParen),
                ')' => self.emit_token(TokenType::RParen),
                '{' => self.emit_token(TokenType::LBrace),
                '}' => self.emit_token(TokenType::RBrace),
                ',' => self.emit_token(TokenType::Comma),
                // Check if next char is a digit, to account for float syntax '.5'
                '.' => if self.consume_if(|c| c.is_digit(10)) { self.consume_till(|c| !c.is_digit(10)); self.emit_token(TokenType::Number)}
                       else { self.emit_token(TokenType::Dot) }
                ';' => self.emit_token(TokenType::Semicolon),
                '+' => self.emit_token(TokenType::Plus),
                '-' => self.emit_token(TokenType::Minus),
                '*' => self.emit_token(TokenType::Star),
                '/' => self.emit_token(TokenType::Slash),
                
                '!' => if self.match_and_consume('=') { self.emit_token(TokenType::BangEqual)   }
                       else {                           self.emit_token(TokenType::Bang)        }
                '=' => if self.match_and_consume('=') { self.emit_token(TokenType::Equate)      }    
                       else {                           self.emit_token(TokenType::Equal)       }
                '>' => if self.match_and_consume('=') { self.emit_token(TokenType::GreaterEq)   }
                       else {                           self.emit_token(TokenType::GreaterThan) }
                '<' => if self.match_and_consume('=') { self.emit_token(TokenType::LessEq)      }    
                       else {                           self.emit_token(TokenType::LessThan)    }
                
                '"' => {
                    let mut change: usize = 0;
                    let result = self.consume_till_mut(&mut |c| {
                        if c == '\n' { change += 1; }
                        c == '"'
                    });
                    self.line += change;

                    if !result {
                        self.err = "unterminated string".to_string();
                        None 
                    } else { 
                        self.consume(); // Consume terminating quote
                        self.emit_token(TokenType::Str) 
                    }
                }

                _ => {
                    if curr.is_digit(10) {
                        let mut has_dot = false;
                        self.consume_till_mut(&mut |c| {
                            let check = !c.is_digit(10) && (c != '.' || has_dot) && c != '_';
                            if c == '.' { has_dot = true; }
                            check
                        });
                        self.emit_token(TokenType::Number)
                    } else if curr.is_alphabetic() {
                        self.consume_till(|c|{ !c.is_alphabetic() });

                        if let Some(word) = Self::KEYWORDS.get(self.get_slice().unwrap_or("")) {
                            self.emit_token(*word)
                        } else { self.emit_token(TokenType::Identifier) }
                    } else {
                        self.err = "unknown character".to_string();
                        None
                    }
                }
            }
        }
    }
}
