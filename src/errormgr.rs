use std::process;

pub fn HandleError(message: String) {
    println!("{}",message);
    QuitApp() ;
}

fn QuitApp() {
    println!("Application terminated with an error") ;
    process::exit(1)
}