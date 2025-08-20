mod chunk;
mod value;
mod vm;

use chunk::{Chunk, OpCode};


fn main() {
    let mut vm = vm::VM::new();
    let mut chunk = Chunk::new();
    
    let mut constant = chunk.add_constant(1.2);
    chunk.write_chunk(OpCode::OpConstant as usize, 123);
    chunk.write_chunk(constant, 123);

    constant = chunk.add_constant(3.4);
    chunk.write_chunk(OpCode::OpConstant as usize, 123);
    chunk.write_chunk(constant, 123);

    chunk.write_chunk(OpCode::OpAdd as usize, 123);

    constant = chunk.add_constant(5.6);
    chunk.write_chunk(OpCode::OpConstant as usize, 123);
    chunk.write_chunk(constant, 123);

    chunk.write_chunk(OpCode::OpDivide as usize, 123);

    chunk.write_chunk(OpCode::OpNegate as usize, 123);

    chunk.write_chunk(OpCode::OpReturn as usize, 123);
    
    // chunk.disassemble_chunk("test chunk");

    vm.init(&chunk);
    vm.interpret(&chunk);
}
