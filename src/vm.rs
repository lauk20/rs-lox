use crate::chunk::{Chunk, OpCode};
use crate::value::{Value, ValueArray};

#[derive(Default)]
pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

#[allow(dead_code)]
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
                print!("          ");
                for val in &self.stack {
                    print!("[ ");
                    ValueArray::print_value(val);
                    print!(" ]");
                }
                println!();
                self.chunk.disassemble_instruction(self.ip.clone());
            }
            let byte = self.read_byte();
            let opcode: OpCode = match byte.try_into() {
                Ok(op) => op,
                Err(_) => return InterpretResult::InterpretCompilerError,
            };

            match opcode {
                OpCode::OpReturn => {
                    ValueArray::print_value(&self.stack.pop().unwrap());
                    println!();
                    break InterpretResult::InterpretOk;
                }
                OpCode::OpAdd => {
                    self.binary_op(|a, b| a + b);
                }
                OpCode::OpSubtract => {
                    self.binary_op(|a, b| a - b);
                }
                OpCode::OpMultiply => {
                    self.binary_op(|a, b| a * b);
                }
                OpCode::OpDivide => {
                    self.binary_op(|a, b| a / b);
                }
                OpCode::OpNegate => {
                    let top_value = self.pop_stack();
                    self.push_stack(-top_value);
                }
                OpCode::OpConstant => {
                    let index = self.read_byte();
                    let constant = self.chunk.get_constant(index);
                    self.push_stack(constant);
                    // ValueArray::print_value(&constant);
                    // println!();
                }
            }
        }
    }

    fn binary_op<F>(&mut self, op: F) 
    where 
        F: Fn(f64, f64) -> f64,
    {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(op(a, b));
    }

    fn read_byte(&mut self) -> usize {
        let val = self.chunk.get(self.ip);
        self.ip += 1;
        val
    }

    #[allow(dead_code)]
    fn reset_stack(&mut self) {
        self.stack.clear();
    }

    fn push_stack(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop_stack(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
}