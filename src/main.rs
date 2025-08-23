mod chunk;
mod value;
mod vm;
mod compiler;
mod scanner;

use chunk::{
    Chunk, 
    OpCode
};
use std::env;
use std::fs;
use std::io::{
    self,
    Write
};
use std::process::exit;
use vm::{
    InterpretResult, 
    VM
};

fn repl(vm: &mut VM) {
    print!("> ");
    io::stdout().flush().expect("Failed to flush stdout"); // Explicitly flush
    for line in io::stdin().lines() {
        match line {
            Ok(line) => {
                vm.interpret(line);
                print!("> ");
                io::stdout().flush().expect("Failed to flush stdout"); // Explicitly flush
            }
            Err(error) => eprintln!("Error reading line {}", error)
        }
    }
}

fn run_file(vm: &mut VM, path: &str) {
    let source = fs::read_to_string(path).unwrap();
    let result = vm.interpret(source);

    match result {
        InterpretResult::InterpretCompilerError => exit(65),
        InterpretResult::InterpretRuntimeError => exit(70),
        _ => {},
    }
}

fn main() {
    let mut vm = vm::VM::new();
    
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => repl(&mut vm),
        2 => run_file(&mut vm, &args[1]),
        _ => eprintln!("Usage: rslox [path]\n"),
    }
}
