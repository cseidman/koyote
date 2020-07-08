use super::instructions::* ;
use super::opcodes::* ;
use super::objects::* ;

#[derive(Clone)]
pub struct Module {
    pub name: String,
    pub isLoaded: bool,

    pub instructions: Vec<Instruction>,
    pub iCount: usize,

    pub constants: Vec<Box<dyn Obj>>,
    pub iConst: u16
}

impl Module {

    pub fn new(name: String) -> Self {
        return  Module {
            name,
            isLoaded: false,
            instructions: Vec::<Instruction>::new(),
            iCount: 0,
            constants: Vec::<dyn Obj>::new(),
            iConst: 0
        };
    }

    pub fn AddConstant(&mut self, o: Box<dyn Obj>) -> u16 {
        self.constants.push(o) ;
        self.iConst+=1 ;
        return self.iConst-1 ;
    }

    pub fn AddInstruction(&mut self, opcode: OpCode, operands: Vec<u16>) {

        let iLen = operands.len() ;

        let instr = Instruction{
            opcode,
            operandCount: iLen,
            operands,
            comments: ";".to_string(),
            startByte: 0 ,
            endByte: 1 + (iLen * 2)
        };

        self.instructions.push(instr) ;
        self.iCount+=1 ;

    }

    pub fn explain(&self) {
        println!("Module: {}",self.name) ;
        println!("============================") ;
        for i in 0..self.iCount {
            let opc = &self.instructions[i].opcode ;
            print!("{number:>05} | {width:<15} ",number=i,width=OpCode2String(opc)) ;

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


#[cfg(test)]
#[test]
pub fn test_instruction() {
    use OpCode::* ;
    let mut m = Module::new("Test".to_string()) ;
    m.AddInstruction(OP_START,vec![2]) ;
    m.AddInstruction(OP_HALT,vec![]) ;
    m.explain()
}
