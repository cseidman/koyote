pub type OpCode = u8 ;

pub const OP_START:OpCode       = 0;
pub const OP_HALT:OpCode        = 1;
pub const OP_CONST:OpCode       = 2;
pub const OP_IPUSH:OpCode       = 3;
pub const OP_IADD:OpCode        = 4;
pub const OP_IPRINT:OpCode      = 5;
pub const OP_SPUSH:OpCode       = 6;
pub const OP_SADD:OpCode        = 7;
pub const OP_SETLOCAL:OpCode    = 8;
pub const OP_GETLOCAL:OpCode    = 9;
pub const OP_SET_IHEAP:OpCode   = 10;
pub const OP_GET_IHEAP:OpCode   = 11;
pub const OP_MALLOC:OpCode      = 12;

pub fn OpLabel(opcode: OpCode) -> String {
    match opcode {
       OP_HALT      => return "OP_HALT".to_string(),
       OP_START     => return "OP_START".to_string(),
       OP_CONST     => return "OP_CONST".to_string(),
       OP_IPUSH     => return "OP_IPUSH".to_string(),
       OP_IADD      => return "OP_IADD".to_string(),
       OP_IPRINT    => return "OP_IPRINT".to_string(),
       OP_SPUSH     => return "OP_SPUSH".to_string(),
       OP_SADD      => return "OP_SADD".to_string(),
       OP_SETLOCAL  => return "OP_SETVAR".to_string(),
       OP_GETLOCAL  => return "OP_GETVAR".to_string(),
       OP_SET_IHEAP => return "OP_SET_IHEAP".to_string(),
       OP_GET_IHEAP => return "OP_GET_IHEAP".to_string(),
       OP_MALLOC    => return "OP_MALLOC".to_string(),
       _            => return "UNKNOWN".to_string(),
    };
}

#[cfg(test)]
mod test {
    use crate::opcodes::* ;
    #[test]
    pub fn test_OpCode() {
        let v = OP_CONST;
        assert_eq!("OP_CONST",OpLabel(OP_CONST));
    }
}
