mod parser;
mod scanner ;
mod tokens ;
mod errormgr ;
mod compiler ;
mod module ;
mod instructions ;
mod opcodes ;

use errormgr::* ;

use std::fs ;
use std::path::*;

fn main() {

}

pub fn ReadSource(filePath: &str) -> String {
    if !Path::new(filePath).exists() {
       HandleError(format!("Unable to open {}",filePath)) ;
    }
    return fs::read_to_string(filePath).expect("Unable to read source file") ;
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_scan() {
        use super::scanner::Scanner;
        use super::tokens::*;

        let testLocation = format!("{}{}", "./testfiles", "/test.cy");

        let code = super::ReadSource(testLocation.as_str());

        let mut s: Scanner = super::scanner::Scanner::new(code) ;
        println!("Starting ..");
        loop {
            let tok = s.ScanToken();
            println!("{}", tok.name);
            if tok.tokenType == TokenType::T_EOF {
                break;
            }
        }
        println!("Done!")
    }
}

