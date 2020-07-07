use self::Precedence::* ;
use super::tokens::TokenType::* ;
use super::compiler::*;

pub type ParseFn = fn(&Compiler,bool) ;

#[derive(Copy, Clone)]
pub struct ParseRule {
    pub prefix:  Option<ParseFn>,
    pub infix:   Option<ParseFn>,
    pub prec:    Precedence
}

#[derive(PartialEq, PartialOrd)]
#[derive(Copy, Clone)]
pub enum Precedence {
    PREC_NONE,
    PREC_ASSIGNMENT,
    PREC_OR,
    PREC_AND,
    PREC_EQUALITY,
    PREC_COMPARISON,
    PREC_TERM,
    PREC_INCR,
    PREC_FACTOR,
    PREC_UNARY,
    PREC_CALL,
    PREC_ARRAY,
    PREC_INDEX,
    PREC_LIST,
    PREC_PRIMARY
}



