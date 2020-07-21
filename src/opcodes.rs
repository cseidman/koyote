
#[derive(PartialEq)]
#[derive(Clone)]
pub enum OpCode {
    OP_START ,
    OP_HALT ,
    OP_CONST,
    OP_IPUSH
}

use OpCode::* ;

pub fn OpLabel(opcode: &OpCode) -> String {
    match opcode {
       OP_HALT  => return "OP_HALT".to_string(),
       OP_START => return "OP_START".to_string(),
       OP_CONST  => return "OP_CONST".to_string(),
       OP_IPUSH => return "OP_IPUSH".to_string(),
       _ => return "UNKNOWN".to_string()
    };
}


#[cfg(test)]
mod test {
   use crate::opcodes::* ;
   use OpCode::* ;
   #[test]
   pub fn test_OpCode() {
       let v = OP_CONST;
       assert_eq!("OP_CONST",OpLabel(&OP_CONST));
   }
}
