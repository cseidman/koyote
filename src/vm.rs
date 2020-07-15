use super::objects::* ;

const MEMORY_SLOTS:usize = 10240000 ;
const STACK:usize = 256 ; // Stack pointer starts here
const HEAP:usize = 10000000 ; // Heap pointer starts here(and works backwards
const TMP:usize = 10000001 ; // Heap pointer starts here
const MAX_FRAMES:usize = 1024 ;



struct Frame {

}

struct VM {
    // Memory
    Memory: [Box<Obj>;MEMORY_SLOTS],

    sp: usize, // Stack pointer
    hp: usize,  // Heap pointer
    tmp: usize, // Temporary location

    Frames: [Frame;MAX_FRAMES]
}

impl VM {
    pub fn new() -> VM {

        let v = VM {
            Memory: [Box::new(Obj);MEMORY_SLOTS],
            Frames: [Frame{};MAX_FRAMES],
            sp: STACK,
            hp: HEAP,
            tmp: TMP
        };
        return v ;
    }
}

