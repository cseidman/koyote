
pub type OpCode = u8 ;

pub const OP_START:OpCode   = 0;
pub const OP_HALT:OpCode    = 1;
pub const OP_CONST:OpCode   = 2;
pub const OP_IPUSH:OpCode   = 3;
pub const OP_IADD:OpCode    = 4;
pub const OP_IPRINT:OpCode  = 5;
pub const OP_SPUSH:OpCode   = 6;
pub const OP_SADD:OpCode    = 7;

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
