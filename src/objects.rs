use std::io::Bytes;
use std::any::Any;
use super::utypes::* ;

pub enum ObjType {
    VAL_STRING,
    VAL_I32,
    VAL_I64,
    VAL_F32,
    VAL_F64,
    VAL_BOOL
}

use ObjType::* ;

pub trait Obj  {
    fn ShowValue(&self) -> String ;
}

impl Obj for ObjInteger {
    fn ShowValue(&self) -> String {
        let val = self.value ;
        return format!("{}",val) ;
    }
}
// ObjBigInteger ----------------

pub struct ObjBigInteger {
    pub objtype: ObjType,
    pub value: i64
}

// ObjString ----------------
pub struct ObjString {
    pub objtype: ObjType,
    pub value: String
}

impl Obj for ObjString {
    fn ShowValue(&self) -> String {
        let s = self.value.clone() ;
        return s ;
    }

}

impl ObjString {
    pub fn new(s:String) -> ObjString {
        return ObjString {
            objtype: VAL_STRING,
            value: s
        } ;
    }
}

// ObjInteger ----------------

pub struct ObjInteger {
    pub objtype: ObjType,
    pub value: i32
}

impl ObjInteger {
    pub fn new(i:i32) -> ObjInteger {
        return ObjInteger {
            objtype: VAL_I32,
            value: i
        } ;
    }

    pub fn from_bytes(b:[u8;4]) -> ObjInteger {
        return ObjInteger {
            objtype: VAL_I32 ,
            value: i32::from_le_bytes(b)
        }
    }

    pub fn to_bytes(&self) -> [u8;4] {
        return self.value.to_le_bytes() ;
    }

}

impl ObjBigInteger {
    pub fn new(i:i64) -> ObjBigInteger {
        return ObjBigInteger {
            objtype: VAL_I64,
            value: i
        } ;
    }

    pub fn from_bytes(b:[u8;8]) -> ObjBigInteger {
        return ObjBigInteger {
            objtype: VAL_I64 ,
            value: i64::from_le_bytes(b)
        }
    }

    pub fn to_bytes(&self) -> [u8;8] {
        return self.value.to_le_bytes() ;
    }

}

impl Obj for ObjBigInteger {
    fn ShowValue(&self) -> String {
        let val = self.value ;
        return format!("{}",val) ;
    }


}

// ObjFloat ----------------
pub struct ObjFloat {
    pub objtype: ObjType,
    pub value: f32
}
impl ObjFloat {
    pub fn new(i:f32) -> ObjFloat {
        return ObjFloat{
            objtype: VAL_F32,
            value: i
        } ;
    }

    pub fn from_bytes(b:[u8;4]) -> ObjFloat {
        return ObjFloat {
            objtype: VAL_F32 ,
            value: f32::from_le_bytes(b)
        }
    }

    pub fn to_bytes(&self) -> [u8;4] {
        return self.value.to_le_bytes() ;
    }
}

impl Obj for ObjFloat {
    fn ShowValue(&self) -> String {
        let val = self.value ;
        return format!("{}",val) ;
    }

}

// ObjBigFloat
pub struct ObjBigFloat {
    pub objtype: ObjType,
    pub value: f64
}
impl ObjBigFloat{
    pub fn new(i:f64) -> ObjBigFloat {
        return ObjBigFloat {
            objtype: VAL_F64,
            value: i
        } ;
    }

    pub fn from_bytes(b:[u8;8]) -> ObjBigFloat {
        return ObjBigFloat {
            objtype: VAL_F64 ,
            value: f64::from_le_bytes(b)
        }
    }

    pub fn to_bytes(&self) -> [u8;8] {
        return self.value.to_le_bytes() ;
    }
}

impl Obj for ObjBigFloat {
    fn ShowValue(&self) -> String {
        let val = self.value ;
        return format!("{}",val) ;
    }

}

// ObjBool
pub struct ObjBool {
    pub objtype: ObjType,
    pub value: bool
}

impl ObjBool{
    pub fn new(i:bool) -> ObjBool {
        return ObjBool {
            objtype: VAL_F64,
            value: i
        } ;
    }

    pub fn from_bytes(b:VAL) -> ObjBool {
        return ObjBool {
            objtype: VAL_BOOL ,
            value: i32::from_le_bytes(b) != 0
        }
    }

    pub fn to_bytes(&self) -> VAL {
        return (self.value as i32).to_le_bytes() ;
    }
}

impl Obj for ObjBool {
    fn ShowValue(&self) -> String {
        let val = self.value ;
        return format!("{}",val) ;
    }

}