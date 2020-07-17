use std::io::Bytes;

pub enum ObjType {
    VAL_STRING,
    VAL_INTEGER,
    VAL_FLOAT,
    VAL_BOOL
}

use ObjType::* ;

pub trait Obj<T>  {
    fn ShowValue(&self) -> String ;
    fn ToBytes(&self) -> Vec<u8> ;
}

// ObjString ----------------
pub struct ObjString {
    pub objtype: ObjType,
    pub value: String
}

impl Obj<String> for ObjString {
    fn ShowValue(&self) -> String {
        let s = self.value.clone() ;
        return s ;
    }
    fn ToBytes(&self) -> Vec<u8> {
        return self.value.as_bytes().to_vec() ;
    }

}

impl ObjString {
    pub fn new(s:String) -> Box<ObjString> {
        return Box::new(ObjString{
            objtype: VAL_STRING,
            value: s
        }) ;
    }
}

// ObjInteger ----------------

pub struct ObjInteger {
    pub objtype: ObjType,
    pub value: i64
}

impl ObjInteger {
    pub fn new(i:i64) -> Box<ObjInteger> {
        return Box::new(ObjInteger{
            objtype: VAL_INTEGER,
            value: i
        }) ;
    }
}

impl Obj for ObjInteger {
    fn ShowValue(&self) -> String {
        let val = self.value ;
        return format!("{}",val) ;
    }
    fn ToBytes(&self) -> Vec<u8> {
        return self.value.to_le_bytes().to_vec();
    }

}

// ObjFloat ----------------
pub struct ObjFloat {
    pub objtype: ObjType,
    pub value: f64
}
impl ObjFloat {
    pub fn new(i:f64) -> Box<ObjFloat> {
        return Box::new(ObjFloat{
            objtype: VAL_FLOAT,
            value: i
        }) ;
    }
}

impl Obj for ObjFloat {
    fn ShowValue(&self) -> String {
        let val = self.value ;
        return format!("{}",val) ;
    }
    fn ToBytes(&self) -> Vec<u8> {
        return self.value.to_le_bytes().to_vec();
    }

}