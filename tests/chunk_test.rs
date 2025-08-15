#[cfg(test)]
mod tests {
    use rs_lox::chunk::{OpCode, Chunk};

    #[test]
    fn test_chunk() {
        let mut chunk = Chunk::new();
        let constant = chunk.add_constant(1.2);
        chunk.write_chunk(OpCode::OpConstant as usize, 123);
        chunk.write_chunk(constant, 123);
        chunk.write_chunk(OpCode::OpReturn as usize, 123);
        chunk.disassemble_chunk("test chunk");
    }
}