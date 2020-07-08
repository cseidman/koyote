#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(dead_code)]
mod utils;
mod parser;
mod scanner ;
mod tokens ;
mod errormgr ;
mod compiler ;
mod module ;
mod opcodes ;
mod instructions ;
mod rules;
mod objects;

use errormgr::* ;

use std::fs ;
use std::path::*;
use std::env ;
use std::io::{self,Write};
use std::fmt::Error;

use crate::compiler::*;
use std::borrow::{BorrowMut, Borrow};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Starting REPL") ;
        return Repl() ;
    }
    return Ok(()) ;
}

pub fn Repl() -> io::Result<()> {

    //let app = App::new() ;

    let mut compiler = Compiler::new("main".to_string()) ;

    loop {
        let mut buffer = String::new();

        print!("> ") ;
        let res = io::Write::flush(&mut io::stdout());
        if res.is_err() {
            HandleError("Unable to write to console");
        }

        io::stdin().read_line(&mut buffer)?;

        if buffer.trim().to_uppercase() == "\\Q".to_string() {
           break ;
        }


        compiler.Compile(&buffer);

    }
    return Ok(()) ;
}


pub fn ReadSource(filePath: &str) -> String {
    if !Path::new(filePath).exists() {
       HandleError(format!("Unable to open {}",filePath).as_str()) ;
    }
    return fs::read_to_string(filePath).expect("Unable to read source file") ;
}
