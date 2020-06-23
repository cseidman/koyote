use super::module::* ;

pub struct Instructions {
    module: *Module,
    instructionPtr: usize,           // Current instruction pointer
}