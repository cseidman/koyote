
use super::opcodes::* ;
use super::module::* ;
#[derive(Clone)]
pub struct Instruction {
    pub opcode : OpCode,
    pub operands: Vec<u16>,
    pub operandCount : usize,

    pub comments: String,

    pub startByte : usize,
    pub endByte: usize,
}

impl Instruction {

    pub fn AddComment(&mut self,comment: String) {
        self.comments = comment
    }
}

