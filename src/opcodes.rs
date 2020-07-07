#[derive(PartialEq)]
#[derive(Clone)]
pub enum OpCode {
    OP_START ,
    OP_HALT ,
    OP_CONST(u16),
    OP_PUSH(i32)
}

pub fn OpCode2String(opcode: &OpCode) -> String {
    match opcode {
       OpCode::OP_HALT  => return "OP_HALT".to_string(),
       OpCode::OP_START => return "OP_START".to_string(),
        _ => return "UNKNOWN".to_string()
    };
}
