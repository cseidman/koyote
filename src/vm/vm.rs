use std::convert::TryInto;
use std::thread::*;
use std::sync::{Arc, Mutex};
use std::ops::Deref;

use crate::memory::ram::*;
use crate::opcodes::*;
use crate::objects::datatypes::* ;
use crate::constants::ConstantPool;
use crate::{WVAL};

const MAX_FRAMES:usize = 1024 ;

//#[derive(Copy, Clone)]
struct Frame<'a> {
    pub locals: &'a mut [WVAL],
    pub fsp: usize
}

impl Frame<'_> {
    pub fn new(m: &mut Memory,slotcount: usize) -> Frame {
        return Frame {
            locals: &mut m.Stack[m.sp..m.sp+slotcount]  ,
            fsp: 0
        }
    }

    pub fn Push(&mut self, v:WVAL) {
        self.locals[self.fsp] = v ;
        self.fsp+=1 ;
    }

    pub fn Pop(&mut self) -> WVAL {
        self.fsp -= 1 ;
        return self.locals[self.fsp] ;
    }
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

    pub fn GetOperand(&mut self) -> [u8;8] {
        let val = self.Code[self.p..self.p+8].try_into().expect("Expected 8 byte array") ;
        self.p += 8 ;
        return val ;
    }

    pub fn GetIOperand(&mut self) -> i64 {
        let val = i64::from_le_bytes(self.Code[self.p..self.p+8].try_into().expect("Expected 8 byte array to convert to I64")) ;
        self.p += 8 ;
        return val ;
    }

    pub fn GetFOperand(&mut self) -> f64 {
        let val = f64::from_le_bytes(self.Code[self.p..self.p+8].try_into().expect("Expected 8 byte array to convert to F64")) ;
        self.p += 8 ;
        return val ;
    }
}

pub struct App {
    heap: Arc<Mutex<Vec<ObjVal>>>,
    vm: VM
}

impl App {

    pub fn new() -> App {
        return App {
            heap: Arc::new(Mutex::new(Vec::with_capacity(128000))),
            vm: VM::new()
        }
    }

    pub fn LaunchVM(&mut self, code:Arc<Mutex<Vec<u8>>>) {

       let c = Arc::clone(&code);
       let h = Arc::clone(&self.heap);

       let compute = spawn(move || {
           let c2 = &*c.lock().unwrap();
           let h2 = &mut *h.lock().unwrap() ;
           ExecStack(h2, c2);
        });

        compute.join() ;
    }
}

struct VM {
    Registers: [u32;1024],
    Constants: ConstantPool,
}

impl VM {

    /* VM Management */
    pub fn new() -> VM {
        let v = VM {
            Registers: [0;1024],
            Constants: ConstantPool::new(64000)
        };
        return v ;
    }

}

// Mainloop
pub fn ExecStack(heap: &mut Vec<ObjVal>,code: &Vec<u8>) {

    let mut vm = VM::new() ;
    let mut fstack = Vec::with_capacity(1024) ;
    let mut m = Memory::new() ;

    let mut b = ByteCode::new(&code) ;
    let mut f = Frame::new(&mut m, 16) ;
    fstack.push(&f) ;

    let ln = b.Code.len() ;
    loop {

        let c =  b.Next()   ;
        println!("OP: {}",OpLabel(c)) ;
        match c {
            OP_HALT => {
                break ;
            },
            OP_IPUSH => {
                f.Push(b.GetOperand()) ;
            },
            OP_IADD => {
                let vr = i64::from_le_bytes(f.Pop()) ;
                let vl = i64::from_le_bytes(f.Pop()) ;
                let res = vl + vr ;
                f.Push(res.to_le_bytes()) ;
            },
            OP_SPUSH => {
                f.Push(b.GetOperand()) ;
            },
            OP_SETLOCAL => {
                let addr = b.GetIOperand() as usize ;
                f.locals[addr] = f.Pop() ;
            },
            OP_GETLOCAL => {
                let addr = b.GetIOperand() as usize ;
                f.Push(f.locals[addr])
            } ,
            OP_MALLOC => {
                heap.push(ObjVal::NULL);
                let addr = heap.len() as i64;
                f.Push(addr.to_le_bytes())
            },
            OP_SET_IHEAP => {
                let o = ObjVal::INT(i64::from_le_bytes(f.Pop())) ;
                let loc = i64::from_le_bytes(f.Pop()) as usize ;
                heap[loc] = o ;
            },
            OP_GET_IHEAP => {
                let loc = i64::from_le_bytes(f.Pop()) as usize;
                f.Push(heap[loc].GetInt().to_le_bytes());
            },

            OP_IPRINT => {
                let val = i64::from_le_bytes(f.Pop()) ;
                println!("{}",val) ;
            },
            _ => {
                println!("Oops!")
            }
        }

        // End of the line
        if b.p >= ln {
            break ;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::vm::vm::* ;
    use crate::opcodes::* ;
    use std::thread ;
    use std::sync::{Arc, Mutex};

    fn PushNum(v: &mut Vec<u8>, num: i64) {
        let ar:[u8;8] = i64::to_le_bytes(num) ;
        for i in 0..8 {
            v.push(ar[i])
        }
    }

    fn PushInstr(v: &mut Vec<u8>, i: OpCode) {
        v.push(i) ;
    }

    fn PushCmd(v: &mut Vec<u8>, i: OpCode, num: i64) {
        PushInstr(v, i);
        PushNum(v, num) ;
    }

    #[test]
    pub fn test_Vm() {

        let mut ins = Vec::new() ;

        let mut app = App::new() ;

        let val = app.vm.Constants.Add("Hey hey HEY!!").to_le_bytes()  ;
        PushCmd(&mut ins, OP_IPUSH, 7) ;
        PushCmd(&mut ins, OP_IPUSH, 4) ;
        PushInstr(&mut ins, OP_IADD) ;
        PushInstr(&mut ins, OP_IPRINT) ;

        PushCmd(&mut ins, OP_IPUSH, 15) ;
        PushCmd(&mut ins, OP_SETLOCAL, 0) ;
        PushCmd(&mut ins, OP_IPUSH, 25) ;
        PushCmd(&mut ins, OP_SETLOCAL, 1) ;
        PushCmd(&mut ins, OP_GETLOCAL, 0) ;
        PushInstr(&mut ins, OP_IPRINT) ;

        PushCmd(&mut ins, OP_IPUSH, 6) ;
        PushCmd(&mut ins, OP_IPUSH, 4) ;
        PushInstr(&mut ins, OP_IADD) ;
        PushInstr(&mut ins, OP_IPRINT) ;

        let mut v = Arc::new(Mutex::new(ins)) ;

        app.LaunchVM(v) ;

    }
}
