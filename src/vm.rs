use super::objects::* ;
use std::convert::TryInto;

const MEMORY_SLOTS:usize = 10240000 ;
const STACK:usize = 256 ; // Stack pointer starts here
const HEAP:usize = 10000000 ; // Heap pointer starts here(and works backwards
const TMP:usize = 10000001 ; // Heap pointer starts here
const MAX_FRAMES:usize = 1024 ;
const WORD_SIZE:usize = 4 ;


#[derive(Copy, Clone)]
struct Frame {

}


struct VM {
    // Memory
    Memory: [u8;MEMORY_SLOTS*WORD_SIZE],

    sp:  usize,  // Stack pointer
    hp:  usize,  // Heap pointer
    tmp: usize,  // Temporary location
    mp:  usize,  // Memory pointer

    Frames: [Frame;MAX_FRAMES]
}

impl VM {
    /* Memory management */

    // Find a particular object at a given location
    fn get_int(&mut self, location: usize) -> u32 {
        return u32::from_le_bytes(self.Memory[location ..location+WORD_SIZE].try_into().expect("Incorrect length")) ;
    }
    // Write integer ti
    fn put_int(&mut self, location:usize, obj:u32) {
        let s:[u8;4] = obj.to_le_bytes() ;
        self.Memory[location..location+4].copy_from_slice(&s) ;
    }

    /* VM Management */
    pub fn new() -> VM {
        let v = VM {
            Memory: [0;MEMORY_SLOTS*WORD_SIZE],
            Frames: [Frame{};MAX_FRAMES],
            sp: STACK,
            hp: HEAP,
            tmp: TMP,
            mp: 0
        };
        return v ;
    }
}

