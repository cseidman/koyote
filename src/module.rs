
use crate::instructions::{Instruction};
use crate::opcodes::* ;
use crate::objects::datatypes::* ;
use crate::tokens::{Token,TokenType};

use ObjType::* ;
use TokenType::* ;
use crate::{VAL, WVAL} ;

pub struct Module {
    pub name: String,
    pub isLoaded: bool,

    pub instructions: Vec<Instruction>,
    pub iCount: usize,

    pub constants: Vec<ObjVal>,
    pub iConst: u16,
    pub bytes: usize
}

impl Module {

    pub fn new(name: String) -> Self {
        return  Module {
            name,
            isLoaded: false,
            instructions: Vec::<Instruction>::new(),
            iCount: 0,
            constants: Vec::<ObjVal>::new(),
            iConst: 0,
            bytes: 0
        };
    }

    pub fn ClearInstructions(&mut self) {
        self.instructions = Vec::<Instruction>::new() ;
        self.iCount = 0 ;
        self.bytes = 0 ;
    }

    // Add the given object to the current module's constant pool
    // and return the index number of the new constant
    pub fn NewConstant(&mut self, o: ObjVal) -> u16 {
        self.constants.push(o) ;
        self.iConst+=1 ;
        return self.iConst-1 ;
    }

    // Once an instruction has added, we can add operands to the same instruction
    pub fn AddOperand(&mut self, operand: WVAL) {
        self.instructions[self.iCount-1].AddOperand(operand) ;
        self.bytes+=8 ; // Add 2 to the total bytes
    }

    pub fn GetByteCode(&mut self) -> Vec<u8> {
        let mut b = Vec::<u8>::new()  ;
        for i in 0 .. self.instructions.len() {
            b.push(self.instructions[i].opcode as u8) ;
            for j in 0..self.instructions[i].operandCount {
                b.push(self.instructions[i].operands[j][0]);
                b.push(self.instructions[i].operands[j][1]);
                b.push(self.instructions[i].operands[j][2]);
                b.push(self.instructions[i].operands[j][3]);
                b.push(self.instructions[i].operands[j][4]);
                b.push(self.instructions[i].operands[j][5]);
                b.push(self.instructions[i].operands[j][6]);
                b.push(self.instructions[i].operands[j][7]);
            }
        }
        self.ClearInstructions() ;
        return b ;
    }

    pub fn AddInstruction(&mut self, opcode: OpCode) {

        let instr = Instruction{
            opcode,
            operandCount: 0,
            operands: Vec::new(),
            comments: ";".to_string(),
            bytes: 1
        };

        self.instructions.push(instr) ;
        self.iCount+=1 ;
        self.bytes+=1 ;

    }

    pub fn explain(&self) {
        println!("Module: {}",self.name) ;
        println!("============================") ;
        for i in 0..self.iCount {
            let opc = self.instructions[i].opcode ;
            print!("{number:>05} | {width:<15} ",number=i,width=OpLabel(opc)) ;

            if self.instructions[i].operandCount == 0 {
                print!("|") ;
            }

            for j in 0..self.instructions[i].operandCount {
               print!("| {:?}",self.instructions[i].operands[j]) ;
            }
            println!()

        }
        println!()
    }
}

