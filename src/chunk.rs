use crate::value::{ValueArray, Value};

#[derive(Debug)]
#[repr(usize)]
pub enum OpCode {
    OpConstant,
    OpReturn,
}

impl TryFrom<usize> for OpCode {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            x if x == OpCode::OpConstant as usize => Ok(OpCode::OpConstant),
            x if x == OpCode::OpReturn as usize => Ok(OpCode::OpReturn),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
pub struct Chunk {
    code: Vec<usize>,
    constants: ValueArray,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk::default()
    }

    pub fn write_chunk(&mut self, byte: usize, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {name} ==");
        let mut offset: usize = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset as usize) as usize;
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{}", format!("{:04} ", offset));

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("  |  ");
        } else {
            print!("{} ", format!("{:04}", self.lines[offset]));
        }

        let byte = self.code[offset];
        let instruction = OpCode::try_from(byte)
            .unwrap_or_else(|_| {
                println!("Unknown opcode {}", byte);
                panic!("Unknown opcode")
            });
        match instruction {
            OpCode::OpReturn => Self::simple_instruction("OpReturn", offset),
            OpCode::OpConstant => Self::constant_instruction("OpConstant", self, offset),
        }
    }

    fn simple_instruction(name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }

    fn constant_instruction(name: &str, chunk: &Self, offset: usize) -> usize {
        let constant_index = chunk.code[offset + 1];
        print!("{} ", format!("{} {:?}", name, constant_index));
        ValueArray::print_value(&chunk.constants.get(constant_index));
        println!();
        offset + 2
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.add_constant(value);
        self.constants.get_count() - 1
    }
}