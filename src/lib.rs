pub mod scanner;
pub mod compiler;
pub mod vm;
pub mod value;
pub mod util;


#[cfg(test)]
mod tests {
    use crate::{compiler, scanner::{Scanner, TokenType}, value::Value, vm::{Chunk, Op, VM}};
    
    #[test]
    fn vm() {
        let mut chunk = Chunk::new();
        chunk
            .push_constant(Value::Number(5.0), 0)
            .push_constant(Value::Number(5.0), 0)
            .push_operation(Op::Add, 0)
            .push_constant(Value::Number(4.0), 0)
            .push_operation(Op::Sub, 0)
            .push_constant(Value::Number(3.0), 0)
            .push_operation(Op::Mul, 0)
            .push_constant(Value::Number(2.0), 0)
            .push_operation(Op::Div, 0)
            .push_operation(Op::Negate, 0)
            .push_operation(Op::Return, 0);
        let mut vm = VM::new();
        vm.load_chunk(chunk);
        assert_eq!(vm.execute_loaded_chunk(), Ok(Value::Number(-9.0)));
    }

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
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Eof);

        // Test number tokens
        let src = "5 5.0 5. .5";
        let mut scanner = Scanner::new(src);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Number);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Number);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Number);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Number);
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Eof);

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
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Eof);

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
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Eof);

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
        assert_eq!(scanner.scan_token().unwrap().t_type, TokenType::Eof);
        assert_eq!(scanner.line, 4);
    }

    #[test]
    fn compiler() {
        // Test math expression
        let src = "-(5 + 4) * 2 / 2;";
        let chunk = compiler::compile(src).unwrap();
        
        let expected = [
            Op::LoadConst(0),
            Op::LoadConst(1),
            Op::Add,
            Op::Negate,
            Op::LoadConst(2),
            Op::Mul,
            Op::LoadConst(3),
            Op::Div,
            Op::Pop,
        ];

        for (i, (op, _)) in chunk.code.iter().enumerate() {
            assert_eq!(*op, expected[i]);  
        }

        // Test boolean expressions
        let src = "true and false or false and false;";
        let chunk = compiler::compile(src).unwrap();

        let expected = [
            Op::True,
            Op::False,
            Op::And,
            Op::False,
            Op::False,
            Op::And,
            Op::Or,
            Op::Pop,    
        ];

        for (i, (op, _)) in chunk.code.iter().enumerate() {
            assert_eq!(*op, expected[i]);  
        }

        // Test strings
        let src = "\"Hello, \" + \"World\";";
        let chunk = compiler::compile(src).unwrap();

        let expected = [
            Op::LoadConst(0),
            Op::LoadConst(1),
            Op::Add,
            Op::Pop,    
        ];

        for (i, (op, _)) in chunk.code.iter().enumerate() {
            assert_eq!(*op, expected[i]);  
        }

        // Test comparison
        let src = "5 == 5 and 5 != 4 and 5 > 4 and 4 < 5 and 5 >= 4 and 4 <= 5;";
        let chunk = compiler::compile(src).unwrap();

        let expected = [
            Op::LoadConst(0),
            Op::LoadConst(1),
            Op::Equal,
            Op::LoadConst(2),
            Op::LoadConst(3),
            Op::NotEqual,
            Op::And,
            Op::LoadConst(4),
            Op::LoadConst(5),
            Op::GreaterThan,
            Op::And,
            Op::LoadConst(6),
            Op::LoadConst(7),
            Op::LessThan,
            Op::And,
            Op::LoadConst(8),
            Op::LoadConst(9),
            Op::GreaterEq,
            Op::And,
            Op::LoadConst(10),
            Op::LoadConst(11),
            Op::LessEq,
            Op::And,
            Op::Pop,    
        ];

        for (i, (op, _)) in chunk.code.iter().enumerate() {
            assert_eq!(*op, expected[i]);  
        }
    }
}
