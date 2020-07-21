
use super::opcodes::* ;
use super::module::* ;
#[derive(Clone)]
pub struct Instruction {
    pub opcode : OpCode,
    pub operands: Vec<[u8;4]>,
    pub operandCount : usize,

    pub comments: String,

    pub bytes: usize,
}

impl Instruction {

    pub fn AddComment(&mut self,comment: String) {
        self.comments = comment
    }

    pub fn AddOperand(&mut self,operand: [u8;4]) {
        self.operands.push(operand);
        self.operandCount+=1 ;
        self.bytes+=2 ; // 2 bytes for the u16
    }
}

