// Memory

use std::convert::TryInto;
use super::macrolib::* ;
use super::objects::* ;
use std::ops::Deref;

const BLOCK_HEAP:usize = 65536 ;
const BLOCK_STACK:usize = 8092 ;
const STACK_SIZE:usize = 1000000 ;

const SVAL_SIZE:usize = 2 ;
const VAL_SIZE:usize = 4 ;
const WVAL_SIZE:usize = 8 ;

type SVAL   = [u8;SVAL_SIZE];
type VAL    = [u8;VAL_SIZE] ;
type WVAL   = [u8;WVAL_SIZE] ;

struct Freeblock {
    Address: usize,
    Size: usize
}

pub struct Memory {
    Stack: [u8;STACK_SIZE],
    Heap: Vec<Box<dyn Obj>>,
    Registers: [u32;1024],

    sp: usize,// Stack pointer
    hp: usize,// Heap pointer
    tmp: usize,// Temporary location

    FreeBlocks: Vec<Freeblock>
}

impl Memory {
    pub fn new() -> Memory {
        return Memory {
            Stack: [0;STACK_SIZE] ,
            Heap: Vec::with_capacity(BLOCK_HEAP*1024) ,
            Registers: [0;1024],
            sp: 0,
            hp: 0,
            tmp: 0,
            FreeBlocks: vec![Freeblock{Address:0,Size:BLOCK_HEAP*1024}]
        };
    }
    // stack operations ********************************
    // 4 byte values
    pub fn Push(&mut self,v: VAL) {
        for i in  0 .. VAL_SIZE-1 {
            self.Stack[self.sp+i] = v[i] ;
        }
        self.sp+=VAL_SIZE ;
    }

    pub fn Pop(&mut self) -> [u8;4] {
        self.sp-=VAL_SIZE ;

        let from = self.sp ;
        let to = self.sp+VAL_SIZE ;

        return self.Stack[from .. to].try_into().expect(format!("Expected a {} byte array from {} to {}",VAL_SIZE,from,to).as_str())

    }

    // 8 byte values
    pub fn WPush(&mut self,v: WVAL) {
        for i in  0 .. WVAL_SIZE-1 {
            self.Stack[self.sp+i] = v[i] ;
        }
        self.sp+=WVAL_SIZE ;
    }

    pub fn WPop(&mut self) -> WVAL {
        self.sp-=WVAL_SIZE ;

        let from = self.sp ;
        let to = self.sp+WVAL_SIZE ;

        return self.Stack[from .. to].try_into().expect(format!("Expected a {} byte array from {} to {}",VAL_SIZE,from,to).as_str())

    }

    pub fn GetFreeSlot(&self) -> usize {
        return 0 ;
    }

    pub fn Put(&mut self, o:Box<dyn Obj>) -> usize {
        if self.GetFreeSlot() == 0 {
            self.Heap.push(o);
            self.hp += 1;
            return self.hp - 1;
        } else {
            return 0 ;
        }
    }

    pub fn Get(&mut self, addr:usize) -> &Box<dyn Obj>{
        return &self.Heap[addr] ;
    }

    // Heap operations *********************************
    /*
    pub fn PutHeap(&mut self, v: VAL) -> usize {
        let oldHp = self.hp ;
        for i in 0..VAL_SIZE {
            self.Heap.push(v[i]) ;
        }

        self.hp+=VAL_SIZE ;
        return oldHp ;
    }

    pub fn GetHeap(&mut self, addr:usize) -> VAL {
        return self.Heap[addr..addr+VAL_SIZE].try_into().expect("Expected 4 bytes from the heap") ;
    }

    pub fn WPutHeap(&mut self, v: WVAL) -> usize {
        let oldHp = self.hp ;
        for i in 0..WVAL_SIZE {
            self.Heap.push(v[i]) ;
        }

        self.hp+=WVAL_SIZE ;
        return oldHp ;
    }

    pub fn WGetHeap(&mut self, addr:usize) -> WVAL {
        return self.Heap[addr..addr+WVAL_SIZE].try_into().expect("Expected 8 bytes from the heap") ;
    }

    // Clears a section of the heap for use
    pub fn ClearHeap(&mut self, addr:usize, length:usize) {

    }
    */

}

// Unit tests ***************************

#[cfg(test)]
mod tests {
    use crate::memory::Memory;
    use crate::objects::ObjInteger;
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
        let somenum:i64 = 53 ;
        let addr = z.Put(ObjInteger::new(somenum));
        let val:&ObjInteger = z.Get(addr).GetType().downcast_ref::<ObjInteger>().unwrap() ;

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