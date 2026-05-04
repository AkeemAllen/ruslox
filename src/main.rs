mod chunk;
mod debug;
mod value;

use chunk::{OpCode, add_constant, free_chunk, write_chunk};
use debug::disassemble_chunk;
use value::Value;

#[derive(Debug)]
struct Chunk {
    code: Vec<OpCode>,
    constants: Vec<Value>,
}

fn main() {
    let mut chunk: Chunk = Chunk {
        code: vec![],
        constants: vec![],
    };
    write_chunk(&mut chunk, OpCode::OpConstant);
    _ = add_constant(&mut chunk, Value(1.2));

    write_chunk(&mut chunk, OpCode::OpReturn);
    write_chunk(&mut chunk, OpCode::OpAdd);
    write_chunk(&mut chunk, OpCode::OpConstant);
    _ = add_constant(&mut chunk, Value(2.2));

    println!("Chunk: {:?}", chunk);
    //disassemble_chunk(&chunk, "test chunk");
    free_chunk(&mut chunk);
}
