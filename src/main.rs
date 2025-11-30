use std::{env, fs, io::{self, Write}};

use rlox::vm::VM;

fn repl(vm: &mut VM) -> io::Result<()> {
    let mut line = String::new();
    loop {
        print!("//rlox> ");
        io::stdout().flush().unwrap();
        line.clear();

        match io::stdin().read_line(&mut line) {
            Ok(0) => {
                println!();
                return Ok(());
            }
            Ok(_) => {
                vm.interpret(line.as_str()).unwrap();
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

fn run_file(vm: &mut VM, path: &str) {
    let content = fs::read_to_string(path).unwrap();
    vm.interpret(content.as_str()).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut vm = VM::new();

    if args.len() == 1 { repl(&mut vm).unwrap(); }
    else if args.len() == 2 {
        run_file(&mut vm, args[1].as_str());
    }
}
