
use super::opcodes::* ;
use super::module::* ;
#[derive(Clone)]
pub struct Instruction {
    pub opcode : OpCode,
    pub operands: Vec<[u8;8]>,
    pub operandCount : usize,

    pub comments: String,

    pub bytes: usize,
}

impl Instruction {

    pub fn AddComment(&mut self,comment: String) {
        self.comments = comment
    }

    pub fn AddOperand(&mut self,operand: [u8;8]) {
        self.operands.push(operand);
        self.operandCount+=1 ;
        self.bytes+=8 ; // 8 bytes for the i64
    }
}

