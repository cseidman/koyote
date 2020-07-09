use super::instructions::* ;
use super::opcodes::* ;
use super::objects::* ;


pub struct Module {
    pub name: String,
    pub isLoaded: bool,

    pub instructions: Vec<Instruction>,
    pub iCount: usize,

    pub constants: Vec<Box<dyn Obj>>,
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
            constants: Vec::<Box<dyn Obj>>::new(),
            iConst: 0,
            bytes: 0
        };
    }

    // Add the given object to the current module's constant pool
    // and return the index number of the new constant
    pub fn NewConstant(&mut self, o: Box<dyn Obj>) -> u16 {
        self.constants.push(o) ;
        self.iConst+=1 ;
        return self.iConst-1 ;
    }

    // Once an instruction has added, we can add operands to the same instruction
    pub fn AddOperand(&mut self, index: u16) {
        self.instructions[self.iCount].AddOperand(index) ;
        self.bytes+=2 ; // Add 2 to the total bytes
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
            let opc = &self.instructions[i].opcode ;
            print!("{number:>05} | {width:<15} ",number=i,width=OpLabel(opc)) ;

            if self.instructions[i].operandCount == 0 {
                print!("|") ;
            }

            for j in 0..self.instructions[i].operandCount {
               print!("| {}",self.instructions[i].operands[j]) ;
            }
            println!()

        }
        println!()
    }
}

