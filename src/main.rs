#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![macro_use]

extern crate clap ;

use std::fs ;
use std::path::*;
use std::env ;
use std::io::{self,Write};
use std::fmt::Error;
use std::borrow::{BorrowMut, Borrow};
use clap::{Arg, App};

use koyote::compiler::compiler::{Compiler} ;
use koyote::errormgr::{HandleError} ;
use koyote::utils::conversion::* ;

fn main() -> io::Result<()> {
/*
    App::new("Coyote")
        .version("1.0")
        .author("Claude Seidman claude@intellixus.com")
        .arg(Arg::with_name("file")
            .short("f") // allow --file
            .takes_value(true)
            .help("Coyote program to run")
            .required(false)
        .arg(Arg::with_name("debug")
            .short("d")
            .multiple(true)
            .help("Sets the level of debugging information"))
        .get_matches());

    // now add in the argument we want to parse
    let app = app.arg(file_option);

    // extract the matches
    let matches = app.get_matches();


    // Extract the actual name
    let name = matches.value_of("file").expect("No file name provided: ");
*/
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Starting REPL") ;
        return Repl() ;
    }
    return Ok(()) ;
}

pub fn Repl() -> io::Result<()> {

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
