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
const MEMORY_SLOTS:usize = 10240000 ;

struct Freeblock {
    Address: usize,
    Size: usize
}

/*
Extent has a total of 65536 of memory
Extent: 8 x 8k Pages
Page: 4b x 2000 DWords

Extent (bytes):
1:        Number of pages used (0-8)
2-3:      Number of DWords used in page 1
4-5:      Number of DWords used in page 2
..
16-17:    Number of DWords used in page 8

*/
pub struct Extent {
    Block: [u8;64000]
}

impl Extent {
    pub fn new() -> Extent {
        return Extent {
            Block: [0;64000]
        };
    }
    // 4 bytes * 16000
    // 1536 bytes remaining
    // Bytes:
    // 1-2: Pages in use (0-8000)
    pub fn AllocatePage(&mut self) -> usize {
        for i in 1..8000 {
            let offset = 1536 + i ;
            if self.Block[offset] == 0 {
                self.Block[offset] = 1; // Set to allocated
                return i ;
            }
        }
        return 0 ;
    }

}


pub struct HMemory {
    Heap: Vec<VAL>,
    Extents: Vec<Extent>,
    hp: usize,// Heap pointer
}

impl HMemory {

    pub fn new() -> HMemory {
        return HMemory {
            Heap: Vec::with_capacity(MEMORY_SLOTS) ,
            hp: 0
        }
    }

    pub fn GetFreeSlot(&self) -> usize {
        return 0 ;
    }
    pub fn GetFreeSlots(&self, count: usize) -> &[Val]{

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

pub struct Memory {
    Static: Vec<VAL>,
    Stack: Box<[VAL]>,

    pub sp: usize, // Stack pointer
    tmp: usize, // Temporary location

}

impl Memory {
    pub fn new() -> Memory {
        return Memory {
            Static: Vec::with_capacity(STATIC_SIZE),
            Stack: vec![[0,0,0,0];STACK_SIZE].into_boxed_slice(),
            sp: 0,
            tmp: 0,
        };
    }


    // stack operations ********************************
    // 4 byte values
    pub fn Push(&mut self, v: VAL) {
        self.Stack[self.sp] = v;
        self.sp+=1 ;
    }

    pub fn WPush(&mut self, v: WVAL) {

        let a:[u8;4] = v[0..4].try_into().expect("Not a 4 byte segment");
        let b:[u8;4] = v[4..8].try_into().expect("Not a 4 byte segment");

        self.Stack[self.sp] = a ;
        self.sp+=1 ;

        self.Stack[self.sp] = b ;
        self.sp+=1 ;
    }

    pub fn Pop(&mut self) -> VAL {
        self.sp-=1 ;
        return self.Stack[self.sp];
    }

    pub fn WPop(&mut self) -> WVAL {
        self.sp-=1;
        let b = self.Stack[self.sp];

        self.sp-=1;
        let a = self.Stack[self.sp] ;

        return [
            a[0],a[1],a[2],a[3],
            b[0],b[1],b[2],b[3]
        ] ;
    }

}

// Unit tests ***************************

#[cfg(test)]
mod tests {
    use crate::memory::*;
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
        let f = m.WPop() ;
        let wnum2 = i64::from_le_bytes(f) ;
        assert_eq!(wnum2,24559) ;

        let fnum1:f64 = 1234.567;
        m.WPush(f64_bytes!(fnum1));
        let fnum2 = bytes_f64!(m.WPop());
        assert_eq!((fnum2*1000.0).round(),1234567.0) ;

    }

    #[test]
    fn test_heap() {
        let mut z = HMemory::new() ;
        let mut c = ConstantPool::new(64) ;

        let somenum:i32 = 53 ;

        let addr = z.Put(ObjInteger::new(somenum).to_bytes());
        let val = ObjInteger::from_bytes(z.Get(addr)) ;

        assert_eq!(val.value,53) ;
    }



}