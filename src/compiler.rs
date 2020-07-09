
use super::scanner::*;
use super::parser::*;
use super::tokens::TokenType::*;
use super::tokens::Token;
use super::rules::* ;
use super::rules::Precedence::* ;
use super::module::* ;
use super::instructions::* ;

use std::ops::Deref;

use std::mem;
use crate::tokens::TokenType;
use crate::errormgr::HandleError;
use crate::module::Module;
use crate::opcodes::OpCode::OP_CONST;
use crate::opcodes::OpCode;
use crate::utils::StringToInt;
use crate::objects::{ObjInteger, ObjType};

use ObjType::* ;


pub struct Compiler {
    pub scanner:  Scanner ,
    pub parser:  CodeParser,
    pub rules: [ParseRule;47],
    pub module: Box<Module>
}

impl Compiler {

    pub fn new(m: String) -> Compiler{

        let mdl = Module::new(m);

        return  Compiler {
            scanner: Scanner::new(),
            parser: CodeParser::new(),
            rules: Compiler::GetRules(),
            module: Box::new(mdl)
        }
    }

    pub fn Compile(&mut self, source: &String) {

        self.scanner.LoadSource(source) ;

        loop {
            // Scan tokens
            let tok = self.scanner.ScanToken() ;
            if tok.tokenType == T_EOF {
                break ;
            }

            self.Evaluate() ;

        }
    }

    fn Evaluate(&mut self) {
        self.Statement() ;
    }

    fn Expression(&mut self) {
        self.ParsePrecedence(PREC_ASSIGNMENT)
    }

    fn Statement(&mut self) {
        let tok = &self.parser.previous ;
        match tok {
            _ => self.ExpressionStatement()
        }
    }

    fn ExpressionStatement(&mut self) {
        self.Expression() ;
    }

    pub fn GetRules() -> [ParseRule;47] {

        let mut p = [ParseRule{prefix: None, infix: None, prec: PREC_NONE};47] ;

        p[T_INTEGER as usize] =  ParseRule{prefix: Some(Compiler::Integer), infix: None, prec: PREC_NONE};

        return p ;
    }

    fn GetRule(&self, t:TokenType) -> ParseRule {
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
        return self.parser.Check(t) ;
    }

    fn Consume(&mut self, t:TokenType, s:&str) {
        self.parser.Consume(t,s) ;
    }

    fn Grouping(&mut self, _canAssign:bool) {
        self.Expression() ;
        self.Consume(T_RIGHT_PAREN, "Expect ')' after expression");
    }

    // Emit operations
    fn EmitOp(&mut self, op: OpCode) {
        self.module.AddInstruction(op) ;
    }

    fn EmitOp2(&mut self, op: OpCode, index: u16 ) {
        self.module.AddInstruction(op) ;
        self.module.AddOperand(index) ;
    }

    // Expression functions
    fn Integer(&mut self, _canAssign:bool) {
        let intg:u64 = u64::Convert(&self.parser.previous.name);
        let idx = self.module.NewConstant(ObjInteger::new(intg)) ;
        self.EmitOp2(OP_CONST, idx);
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

        while prec <= (&self.GetRule(self.parser.current.tokenType)).prec {
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