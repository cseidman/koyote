mod scanner ;
mod tokens ;
mod errormgr ;

use scanner::* ;
use tokens::* ;
use errormgr::* ;

use std::fs ;
use std::path::Path;


fn main() {

}

pub fn ReadSource(filePath: String) -> String {
    if !Path::new(filePath).exists() {
       HandleError(format!("Unable to open {}",filePath)) ;
    }
    return fs::read_to_string(filePath).Expect("Unable to read source file") ;
}


#[cfg(test)]
#[test]
pub fn scantest() {

    let testLocation = "../testfiles" ;

    let code = ReadSource(testLocation + "/test.cy") ;


    let mut s :Scanner = super::NewScanner(code) ;
    println!("Starting ..");
    loop {
        let tok = s.ScanToken();
        println!("{}",tok.name);
        if tok.tokenType == TokenType::T_EOF {
            break ;
        }
    }
}

