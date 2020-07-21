// Memory

use std::convert::TryInto;
use std::mem;

use super::macrolib::* ;
use super::objects::* ;
use super::constants::* ;
use super::utypes::* ;

const BLOCK_HEAP:usize = 65536 ;
const BLOCK_STACK:usize = 8092 ;
const STACK_SIZE:usize = 1000000 ;
const STATIC_SIZE:usize = 1024 ;

struct Freeblock {
    Address: usize,
    Size: usize
}

pub struct HMemory {
    Heap: Vec<VAL>,
    hp: usize,// Heap pointer

    FreeBlocks: Vec<Freeblock>
}


pub struct Memory {
    Static: Vec<VAL>,
    Stack: Vec<VAL>,
    Registers: [u32;1024],

    sp: usize,// Stack pointer
    tmp: usize,// Temporary location

}

impl Memory {
    pub fn new() -> Memory {
        return Memory {
            Static: Vec::with_capacity(STATIC_SIZE),
            Stack: Vec::with_capacity(STACK_SIZE) ,
            Registers: [0;1024],
            sp: 0,
            tmp: 0,
        };
    }

    // stack operations ********************************
    // 4 byte values
    pub fn Push(&mut self, v: VAL) {
        self.Stack.push(v);
        self.sp+=1 ;
    }

    pub fn WPush(&mut self, v: WVAL) {

        self.Stack.push([v[0],v[1],v[2],v[3]]);
        self.Stack.push([v[4],v[5],v[6],v[7]]);
        self.sp+=2 ;
    }

    pub fn Pop(&mut self) -> VAL {
        self.sp-=1 ;
        return self.Stack[self.sp];
    }

    fn WPop(&mut self) -> WVAL {
        let b = self.Stack.pop().unwrap();
        let a = self.Stack.pop().unwrap() ;
        return [
            a[0],a[1],a[2],a[3],
            b[0],b[1],b[2],b[3]
        ] ;
    }

    pub fn GetFreeSlot(&self) -> usize {
        return 0 ;
    }

    pub fn Put(&mut self, o:VAL) -> usize {
        if self.GetFreeSlot() == 0 {
            self.Heap.push(o);
            self.hp += 1;
            return self.hp - 1;
        } else {
            return 0 ;
        }
    }

    pub fn Get(&mut self, addr:usize) -> VAL {
        return self.Heap[addr] ;
    }
}

// Unit tests ***************************

#[cfg(test)]
mod tests {
    use crate::memory::Memory;
    use crate::objects::*;
    use crate::constants::*;
    use crate::objects::ObjType::VAL_I32;
    use std::ops::Deref;

    #[test]
    fn test_stack() {
        let mut m = Memory::new() ;
        let somenum:i32 = 53 ;
        m.Push(somenum.to_le_bytes()) ;
        let numb = i32::from_le_bytes(m.Pop()) ;
        assert_eq!(numb,53) ;

        let wnum1:i64 = 24559;
        m.WPush(wnum1.to_le_bytes()) ;
        let wnum2 = i64::from_le_bytes(m.WPop()) ;
        assert_eq!(wnum2,24559) ;

        let fnum1:f64 = 1234.567;
        m.WPush(f64_bytes!(fnum1));
        let fnum2 = bytes_f64!(m.WPop());
        assert_eq!((fnum2*1000.0).round(),1234567.0) ;
    }

    #[test]
    fn test_heap() {
        let mut z = Memory::new() ;
        let mut c = ConstantPool::new(64) ;

        let somenum:i32 = 53 ;

        let addr = z.Put(ObjInteger::new(somenum).to_bytes());
        let val = ObjInteger::from_bytes(z.Get(addr)) ;

        assert_eq!(val.value,53) ;
    }

    /*
    #[test]
    fn test_hp() {
        let mut z = Memory::new() ;
        let somenum:i32 = 53 ;
        let addr = z.PutHeap(somenum.to_le_bytes()) ;
        let numb = i32::from_le_bytes(z.GetHeap(addr)) ;
        assert_eq!(numb,53) ;

        let num:i64 = 12353 ;
        let waddr = z.WPutHeap(num.to_le_bytes()) ;
        let wnumb = i64::from_le_bytes(z.WGetHeap(waddr)) ;
        assert_eq!(wnumb,12353) ;
    }
    */

}