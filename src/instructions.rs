
use super::opcodes::* ;
use super::module::* ;
#[derive(Clone)]
pub struct Instruction {
    pub opcode : OpCode,
    pub operands: Vec<u16>,
    pub operandCount : usize,

    pub comments: String,

    pub bytes: usize,
}

impl Instruction {

    pub fn AddComment(&mut self,comment: String) {
        self.comments = comment
    }

    pub fn AddOperand(&mut self,operand: u16) {
        self.operands.push(operand);
        self.operandCount+=1 ;
        self.bytes+=2 ; // 2 bytes for the u16
    }
}

