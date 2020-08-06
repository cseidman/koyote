#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![macro_use]

pub mod utils {
    pub mod conversion;
}
pub mod macrolib;
pub mod memory {
    pub mod ram;
}
pub mod parser;
pub mod scanner;
pub mod tokens;
pub mod errormgr;
pub mod compiler;
pub mod module ;
pub mod opcodes ;
pub mod instructions ;
pub mod rules;
pub mod objects {
    pub mod datatypes ;
}
pub mod vm {
    pub mod vm ;
}

pub mod constants;
pub const SVAL_SIZE:usize = 2 ;
pub const VAL_SIZE:usize = 4 ;
pub const WVAL_SIZE:usize = 8 ;

pub type SVAL   = [u8;SVAL_SIZE];
pub type VAL    = [u8;VAL_SIZE] ;
pub type WVAL   = [u8;WVAL_SIZE] ;
