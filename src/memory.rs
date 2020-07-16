// Memory

const BLOCK_HEAP:usize = 65536 ;
const BLOCK_STACK:usize = 8092 ;

pub struct Memory {
    Stack: Vec<u8>,
    Heap: Vec<u8>,
    Registers: [u32;1024],

    sp: usize,// Stack pointer
    hp: usize,// Heap pointer
    tmp: usize,// Temporary location

}

impl Memory {
    pub fn new() -> Memory {
        return Memory {
            Stack: Vec::with_capacity(BLOCK_STACK*1024) ,
            Heap: Vec::with_capacity(BLOCK_HEAP*1024) ,
            Registers: [0;1024],
            sp: 0,
            hp: 0,
            tmp: 0
        };
    }

    pub fn Push<T>(obj:T) {

    }

}



// Unit tests ***************************

#[cfg(test)]
mod tests {
    use crate::memory::Memory;
    #[test]
    fn test_mem() {
        let mut m = Memory::new() ;

    }
}