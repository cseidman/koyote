#[derive(PartialEq)]
#[derive(Clone)]
pub enum OpCode {
    OP_START ,
    OP_HALT ,
    OP_CONST,
    OP_PUSH
}

use OpCode::* ;

pub fn OpLabel(opcode: &OpCode) -> String {
    match opcode {
       OP_HALT  => return "OP_HALT".to_string(),
       OP_START => return "OP_START".to_string(),
       OP_CONST  => return "OP_CONST".to_string(),
       OP_PUSH => return "OP_PUSH".to_string(),
        //_ => return "UNKNOWN".to_string()
    };
}
