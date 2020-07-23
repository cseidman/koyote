use super::objects::* ;
use std::convert::TryInto;
use std::thread;
use super::memory::*;
use super::opcodes::*;
use crate::constants::ConstantPool;
use std::ops::Deref;
use super::utypes::* ;


const MAX_FRAMES:usize = 1024 ;

#[derive(Copy, Clone)]
struct Frame {
    pub offset: usize
}

// Manages the instructions stream
struct ByteCode {
    Code: Vec<u8> ,
    p: usize
}

impl ByteCode {
    pub fn new(v:&Vec<u8>) -> ByteCode {
        return ByteCode {
            Code: v.to_vec(),
            p: 0
        }
    }

    pub fn Next(&mut self) -> OpCode {
        self.p += 1 ;
        return self.Code[self.p-1] ;
    }

    pub fn GetOperand(&mut self) -> VAL {
        let val = self.Code[self.p..self.p+4].try_into().expect("Incorrect") ;
        self.p += 4 ;
        return val ;
    }

    pub fn GetIntegerOpd(&mut self) -> i32 {
        return i32::from_le_bytes(self.GetOperand()) ;
    }

}

struct VM {
    Heap: HMemory,
    Registers: [u32;1024],
    Constants: ConstantPool
}

impl VM {

    /* VM Management */
    pub fn new() -> VM {
        let v = VM {
            Heap: HMemory::new(),
            Registers: [0;1024],
            Constants: ConstantPool::new(64000)
        };
        return v ;
    }

    // Mainloop
    pub fn ExecStack(&mut self, code: &Vec<u8>) {

        let mut bcode = ByteCode::new(code) ;

        let mut m = Memory::new() ;
        let mut frames = [Frame{offset:0};MAX_FRAMES] ;

        let ln = bcode.Code.len() ;
        loop {

            let c =  bcode.Next()   ;
            println!("OP: {}",OpLabel(c)) ;
            match c {
                OP_HALT => {
                    break ;
                },
                OP_IPUSH => {
                    m.Push(bcode.GetOperand()) ;
                },
                OP_IADD => {
                    let vr = i32::from_le_bytes(m.Pop()) ;
                    let vl = i32::from_le_bytes(m.Pop()) ;
                    let res = vl + vr ;
                    m.Push(res.to_le_bytes()) ;
                },
                OP_SPUSH => {
                    m.Push(bcode.GetOperand()) ;
                },
                OP_IPRINT => {
                    let val = i32::from_le_bytes(m.Pop()) ;
                    println!("{}",val) ;
                },
                _ => {
                    println!("Oops!")
                }
            }

            // End of the line
            if bcode.p >= ln {
                break ;
            }

        }

    }

}

#[cfg(test)]
mod test {
    use crate::vm::* ;
    use crate::opcodes::* ;
    #[test]
    pub fn test_Vm() {

        let mut vm = VM::new() ;

        let n1 = i32::to_le_bytes(7) ;
        let n2 = i32::to_le_bytes(4) ;

        let val = vm.Constants.Add("Hey hey HEY!!").to_le_bytes() ;

        let v = vec![
            OP_IPUSH, n1[0],n1[1], n1[2], n1[3],
            OP_IPUSH, n2[0],n2[1], n2[2], n2[3],
            OP_IADD,
            OP_IPRINT,
            OP_SPUSH, val[0], val[1], val[2], val[3]

        ] ;


        vm.ExecStack(&v) ;
    }
}
