pub struct ConstantPool {
    constants: Vec<String> ,
    cp: usize
}

impl ConstantPool {
    pub fn new(capacity:usize) -> ConstantPool {
        return ConstantPool {
            constants: Vec::with_capacity(capacity) ,
            cp: 0
        }
    }
    pub fn Add(&mut self,s: String) -> usize {
        self.constants.push(s) ;
        self.cp+=1 ;
        return self.cp -1 ;
    }

    pub fn Get(&self,addr: usize) -> String  {
        return self.constants[addr].clone() ;
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
        let sconst = "This is a test of the constant pool system".to_string() ;
        let key = c.Add(sconst.clone()) ;
        // Get
        let val = c.Get(key) ;
        assert_eq!(sconst,val) ;
    }
}
