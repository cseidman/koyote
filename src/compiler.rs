#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use super::scanner::*;
use super::parser::*;

pub struct Compiler {
    scanner: Scanner ,
    parser: CodeParser,
}