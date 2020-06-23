#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use super::scanner::* ;
use super::tokens::TokenType ;
use super::tokens::Token;


pub struct CodeParser {
    current: Token,
    previous: Token,
    hadError: bool,
    inPanicMode: bool
}

impl CodeParser {

    pub fn new() -> CodeParser {
        use super::tokens::TokenType::* ;
        return CodeParser{
            current: Token{ tokenType: T_START, line: 0, name: "T_START".to_string(), label: "T_START".to_string() },
            previous: Token{ tokenType: T_START, line: 0, name: "T_START".to_string(), label: "T_START".to_string() },
            hadError: false,
            inPanicMode: false
        };
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_parse() {
        use super::super::parser::*;
        let p = CodeParser::new() ;
    }
}

