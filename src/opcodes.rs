#[derive(PartialEq)]
pub enum OpCode {
    OP_START ,
    OP_HALT ,
    OP_CONST
}

pub fn OpCode2String(opcode: &OpCode) -> String {
    match opcode {
       OpCode::OP_HALT  => return "OP_HALT".to_string(),
       OpCode::OP_START => return "OP_START".to_string(),
        _ => return "UNKNOWN".to_string()
    };
}
