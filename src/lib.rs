pub mod scanner;
pub mod compiler;
pub mod vm;
pub mod util;


#[cfg(test)]
mod tests {
    use crate::scanner::{Scanner, TokenType};

    #[test]
    fn scanner() {
        // Test variable definition
        let src = "let x = 5;";
        let mut scanner = Scanner::new(src);

        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Let);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Identifier);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Equal);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Number);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Semicolon);

        // Test number tokens
        let src = "5 5.0 5. .5";
        let mut scanner = Scanner::new(src);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Number);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Number);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Number);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Number);

        // Math 
        let src = "5.0 + 5.0 - 5.0 * 5.0 / 5.0";
        let mut scanner = Scanner::new(src);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Number);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Plus);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Number);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Minus);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Number);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Star);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Number);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Slash);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Number);

        // Function definition
        let src = "fn foo() { print(\"foo\"); }";
        let mut scanner = Scanner::new(src);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Fn);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Identifier);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::LParen);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::RParen);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::LBrace);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Print);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::LParen);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Str);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::RParen);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Semicolon);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::RBrace);

        // stuct def
        let src = "struct Foo {\n
            x = 0;\n
            y = \"\";\n
        }";
        let mut scanner = Scanner::new(src);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Struct);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Identifier);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::LBrace);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Identifier);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Equal);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Number);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Semicolon);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Identifier);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Equal);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Str);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Semicolon);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::RBrace);
        assert_eq!(scanner.line, 4);
    }
}
