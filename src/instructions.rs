use super::module::* ;

pub struct Instructions<'a> {
    module: &'a Module,
    instructionPtr: usize,           // Current instruction pointer
}