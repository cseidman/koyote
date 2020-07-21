use super::objects::* ;
use std::convert::TryInto;
use std::thread;
use crate::memory::*;

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
    Heap: HMemory,Ok
    Frames: [Frame;MAX_FRAMES]
}

impl VM {



    /* VM Management */
    pub fn new() -> VM {
        let v = VM {
            Heap: HMemory::new(),
            Frames: [Frame{};MAX_FRAMES],
        };
        return v ;
    }
}

