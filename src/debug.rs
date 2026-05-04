use crate::Chunk;
use crate::OpCode;

use crate::value::print_value;

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    return offset + 1;
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = &chunk.code[offset + 1];
    print!("{} {:?}", name, constant);
    let pointer = std::ptr::addr_of!(constant);
    print_value(&chunk.constants[pointer as usize]);
    println!("");
    return offset + 2;
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    let instruction = &chunk.code[offset];
    match instruction {
        OpCode::OpReturn => return simple_instruction("OpReturn", offset),
        OpCode::OpAdd => return simple_instruction("OpAdd", offset),
        OpCode::OpConstant => return constant_instruction("OpConstant", chunk, offset),
    }
}

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("==={}===", name);
    let mut offset = 0;

    for i in offset..chunk.code.len() {
        offset = disassemble_instruction(chunk, i);
    }
}
