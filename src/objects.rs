use std::io::Bytes;

#[derive(Copy)]
enum ObjType {
    VAL_STRING,
    VAL_INTEGER,
    VAL_FLOAT,
    VAL_BOOL
}

use ObjType::* ;

pub trait Obj: Clone + Sized {
    fn ShowValue(&self) -> String ;
    fn ToBytes(&self) -> Vec<u8> ;
}

// ObjString ----------------
#[derive(Clone)]
pub struct ObjString {
    pub objtype: ObjType,
    pub value: String
}

impl Obj for ObjString {
    fn ShowValue(&self) -> String {
        return self.value.clone() ;
    }
    fn ToBytes(&self) -> Vec<u8> {
        return self.value.as_bytes().to_vec() ;
    }
}

// ObjInteger ----------------
#[derive(Clone)]
pub struct ObjInteger {
    pub objtype: ObjType,
    pub value: i64
}

impl Obj for ObjInteger {
    fn ShowValue(&self) -> String {
        return format!("{}",self.value) ;
    }
    fn ToBytes(&self) -> Vec<u8> {
        return self.value.to_le_bytes().to_vec();
    }
}