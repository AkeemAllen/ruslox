use crate::Chunk;

use crate::value::{Value, write_value_array};

#[derive(Debug)]
pub enum OpCode {
    OpReturn,
    OpConstant,
    OpAdd,
}

pub fn write_chunk(chunk: &mut Chunk, op: OpCode) {
    chunk.code.push(op);
}

pub fn free_chunk(chunk: &mut Chunk) {
    chunk.code.clear();
}

pub fn add_constant(chunk: &mut Chunk, value: Value) -> usize {
    write_value_array(&mut chunk.constants, value);
    return chunk.constants.len() - 1;
}
