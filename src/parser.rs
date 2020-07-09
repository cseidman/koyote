
use super::tokens::TokenType ;
use super::tokens::Token;
use super::errormgr::*;

#[derive(Clone)]
pub struct CodeParser {
    pub current: Token,
    pub previous: Token,
    pub hadError: bool,
    pub inPanicMode: bool
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

    pub fn Previous(self) -> Token {
        return self.previous ;
    }

    pub fn Current(self) -> Token {
        return self.current ;
    }

    pub fn Check(self, t:TokenType) -> bool {
        return self.current.tokenType == t ;
    }

    pub fn Match(&mut self, t:TokenType) -> bool {
        if self.current.tokenType == t {
            self.previous = self.current.clone() ;
            return true;
        } else {
            return false ;
        }
    }

    pub fn Consume(&mut self, t:TokenType, s:&str) {
        if self.current.tokenType == t {
            self.previous = self.current.clone() ;
            return;
        }
        HandleError(s) ;
    }
}
