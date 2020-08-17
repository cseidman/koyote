use std::ops::Deref;
use std::mem;

use crate::errormgr::HandleError;
use crate::module::Module;
use crate::opcodes::* ;
use crate::objects::datatypes::*;
use crate::scanner::{Scanner};
use crate::parser::{CodeParser};
use crate::rules::{ParseRule, Precedence, PrecList};
use crate::tokens::{Token, TokenType, TokenList};
use crate::utils::conversion::StringToInt;

use ObjType::* ;
use TokenType::* ;
use Precedence::* ;

pub struct Compiler {
    pub scanner:  Scanner ,
    pub parser:  CodeParser,
    pub rules: Vec<ParseRule>,
    pub module: Box<Module>
}

impl Compiler {

    pub fn new(m: String) -> Compiler{

        let mdl = Module::new(m);

        let mut c =  Compiler {
            scanner: Scanner::new(),
            parser: CodeParser::new(),
            rules: Compiler::GetRules(),
            module: Box::new(mdl)
        } ;

        return c ;

    }

    pub fn Compile(&mut self, source: &String) {

        self.scanner.LoadSource(source) ;
        self.parser = CodeParser::new() ;

        // Load the first active token
        self.Advance();
        loop {
            if self.Match(T_EOF) {
                break ;
            }
            self.Statement();
        }
    }

    pub fn GetByteCode(&mut self) -> Vec<u8> {
        return  self.module.GetByteCode();
    }


    fn Expression(&mut self) {
        self.ParsePrecedence(PREC_ASSIGNMENT)
    }

    fn Statement(&mut self) {

        let retval = true ;
        let tok = &self.parser.previous ;

        match tok.tokenType {
            T_EOF => {return},
            _ => self.ExpressionStatement()
        }
    }

    fn ExpressionStatement(&mut self) {
        self.Expression() ;

        if self.Match(T_EOF) {
            EmitOp!(self OP_IPRINT) ;
        }
    }

    pub fn GetRules() -> Vec<ParseRule> {
        use Precedence::* ;

        let mut v = vec![
            ParseRule{prefix: None, infix: None, prec: PREC_NONE}; TokenList.len()
        ];

        v[T_LEFT_PAREN as usize]    =  ParseRule{prefix: Some(Compiler::Grouping), infix: None, prec: PREC_NONE} ;
        v[T_INTEGER as usize]       =  ParseRule{prefix: Some(Compiler::Integer), infix: None, prec: PREC_NONE} ;
        v[T_MINUS as usize]         =  ParseRule{prefix: Some(Compiler::Unary), infix: Some(Compiler::Binary), prec: PREC_TERM} ;
        v[T_PLUS as usize]         =  ParseRule{prefix: Some(Compiler::Unary), infix: Some(Compiler::Binary), prec: PREC_TERM} ;

        return v ;
    }

    fn GetRule(&self, t:TokenType) -> ParseRule {
        //println!("{}", t as usize);
        return self.rules[t as usize] ;
    }

    fn Advance(&mut self) {
        self.parser.previous =  self.parser.current.clone();
        self.parser.current = self.scanner.ScanToken() ;
    }

    fn Match(&mut self, t:TokenType) -> bool {
       return self.parser.Match(t) ;
    }

    fn Check(self, t:TokenType) -> bool {
        return self.parser.current.tokenType == t ;
    }

    fn Consume(&mut self, t:TokenType, s:&str) {
        if self.parser.current.tokenType == t {
            self.Advance();
        } else {
            HandleError(s) ;
        }
    }

    fn Grouping(&mut self, _canAssign:bool) {
        self.Expression() ;
        self.Consume(T_RIGHT_PAREN, "Expect ')' after expression");
    }

    // Expression functions
    fn Integer(&mut self, _canAssign:bool) {
        //self.Advance();
        let intg:i64 = self.parser.previous.name.parse::<i64>().unwrap();
        let b:[u8;8] = intg.to_le_bytes() ;
        EmitOp!(self OP_IPUSH b);
    }

    fn Unary(&mut self, _canAssign:bool) {

        let operatorType = self.parser.previous.tokenType ;
        self.ParsePrecedence(PREC_UNARY) ;

        match operatorType {
            T_PLUS => EmitOp!(self OP_IADD) ,
            _ => {}
        }
    }

    fn Binary(&mut self, _canAssign:bool) {

        let operatorType = self.parser.previous.tokenType ;
        let rule = self.GetRule(operatorType) ;
        let rprec = rule.prec as usize + 1 ;
        self.ParsePrecedence(PrecList[rprec]) ;

        match operatorType {
            T_PLUS => EmitOp!(self OP_IADD) ,
            _ => {}
        }
    }

    fn ParsePrecedence(&mut self, prec:Precedence) {

        // This loads the prefix rule which either contains a value such as
        // a variable or literal or a prefix that affects the next value
        self.Advance() ;

        let prefix = self.GetRule(self.parser.previous.tokenType).prefix;

        // This is an error in that an expression needs to at least begin
        // with a prefix rule
        if prefix.is_none() {
            //println!("RULE: {} has no prefix", self.parser.previous.tokenType) ;
            HandleError("Expect expression");
            return ;
        }
        let canAssign = prec <= PREC_ASSIGNMENT;
        prefix.unwrap()(self,canAssign);

        while prec <= (&self.GetRule(self.parser.previous.tokenType)).prec {
            self.Advance() ;

            let infix = &self.GetRule(self.parser.previous.tokenType).infix ;
            if infix.is_some()  {
                infix.unwrap()(self,canAssign) ;
            }

            if canAssign && self.Match(T_EQUAL) {
                HandleError("Invalid assignment target.") ;
            }
        }

    }

}