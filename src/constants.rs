pub struct ConstantPool {
    constants: Vec<String> ,
    cp: i32
}

impl ConstantPool {
    pub fn new(capacity:usize) -> ConstantPool {
        return ConstantPool {
            constants: Vec::with_capacity(capacity) ,
            cp: 0
        }
    }
    pub fn Add(&mut self,s: &str) -> i32 {
        self.constants.push(s.to_string()) ;
        self.cp+=1 ;
        return self.cp -1 ;
    }

    pub fn Get(&self,addr: i32) -> String  {
        return self.constants[addr as usize].clone() ;
    }
}

#[cfg(test)]
mod test {
    use crate::constants::ConstantPool;

    #[test]
    pub fn test_Constants() {
        // Init
        let mut c = ConstantPool::new(64) ;
        // Add
        let sconst = "This is a test of the constant pool system" ;
        let key = c.Add(sconst) ;
        // Get
        let val = c.Get(key) ;
        assert_eq!(sconst,val) ;
    }
}
