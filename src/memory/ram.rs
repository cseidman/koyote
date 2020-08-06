// Memory

use std::convert::TryInto;
use std::mem;

use crate::macrolib::* ;
use crate::objects::* ;
use crate::constants::* ;

const BLOCK_HEAP:usize = 65536 ;
const BLOCK_STACK:usize = 8092 ;
const STACK_SIZE:usize = 64000 ;
const STATIC_SIZE:usize = 1024 ;
const MEMORY_SLOTS:usize = 10240000 ;

use crate::{VAL_SIZE,WVAL_SIZE,VAL,WVAL};

pub struct Memory {
    pub Static: Vec<WVAL>,
    pub Stack: [WVAL;STACK_SIZE],

    pub sp: usize, // Stack pointer
    pub tmp: usize, // Temporary location

}

impl Memory {
    pub fn new() -> Memory {
        return Memory {
            Static: Vec::with_capacity(STATIC_SIZE),
            Stack: [[0;8];STACK_SIZE],
            sp: 0,
            tmp: 0,
        };
    }

    // stack operations ********************************

    pub fn Push(&mut self, v: WVAL) {
        self.Stack[self.sp] = v;
        self.sp+=1 ;
    }

    pub fn Pop(&mut self) -> WVAL {
        self.sp-=1 ;
        return self.Stack[self.sp];
    }

}

// Unit tests ***************************

#[cfg(test)]
mod tests {
    use crate::memory::*;
    use crate::objects::*;
    use crate::constants::*;
    use crate::objects::datatypes::ObjType::VAL_I32;
    use std::ops::Deref;

    #[test]
    fn test_stack() {
        let mut m = ram::Memory::new() ;
        let somenum:i64 = 53 ;
        m.Push(i64_bytes!(somenum)) ;
        let numb = m.Pop() ;
        assert_eq!(i64::from_le_bytes(numb),53) ;

        let fnum1:f64 = 1234.567;
        m.Push(f64_bytes!(fnum1));
        let fnum2 = bytes_f64!(m.Pop());
        assert_eq!((fnum2*1000.0).round(),1234567.0) ;

    }

}