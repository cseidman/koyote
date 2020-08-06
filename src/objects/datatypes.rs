use std::io::Bytes;
use std::any::Any;

pub enum ObjType {
    VAL_STRING,
    VAL_I32,
    VAL_I64,
    VAL_F32,
    VAL_F64,
    VAL_BOOL
}

// This we use for stack operations
pub union Value {
    ival: i64,
    fval: f64,
    byteval: [u8;8],
    boolval: u8,
}

pub enum ObjVal {
    NULL,
    STR(Box<String>),
    INT(i64),
    FLOAT(f64),
    BOOL(bool),
    ARRAY
}

impl ObjVal {
    pub fn GetString(&self) -> &String {
        return match self {
            ObjVal::STR(val) => val,
            _ => panic!("Not a String")
        }
    }
    pub fn GetInt(&self) -> i64 {
        return match self {
            ObjVal::INT(val) => *val ,
            _ => panic!("Not an integer")
        }
    }

    pub fn GetFloat(&self) -> &f64 {
        return match self {
            ObjVal::FLOAT(val) => val ,
            _ => panic!("Not an float")
        }
    }

    pub fn GetBool(&self) -> &bool {
        return match self {
            ObjVal::BOOL(val) => val ,
            _ => panic!("Not an boolean")
        }
    }

}

