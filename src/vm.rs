use crate::chunk::{Chunk, OpCode};
use crate::value::{ValueArray};

#[derive(Default)]
pub struct VM {
    chunk: Chunk,
    ip: usize,
}

pub enum InterpretResult {
    InterpretOk,
    InterpretCompilerError,
    InterpretRuntimeError,
}

impl VM {
    pub fn new() -> VM {
        VM::default()
    }

    pub fn init(&mut self, chunk: &Chunk) {
        self.chunk = chunk.clone();
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        self.chunk = chunk.clone();
        self.ip = 0;
        self.run()
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            #[cfg(debug_assertions)]
            {
                self.chunk.disassemble_instruction(self.ip.clone());
            }
            let byte = self.read_byte();
            let opcode: OpCode = match byte.try_into() {
                Ok(op) => op,
                Err(_) => return InterpretResult::InterpretCompilerError,
            };

            match opcode {
                OpCode::OpReturn => {
                    break InterpretResult::InterpretOk;
                }
                OpCode::OpConstant => {
                    let index = self.read_byte();
                    let constant = self.chunk.get_constant(index);
                    ValueArray::print_value(&constant);
                    println!();
                }
            }
        }
    }

    fn read_byte(&mut self) -> usize {
        let val = self.chunk.get(self.ip);
        self.ip += 1;
        val
    }
}