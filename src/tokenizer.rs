pub mod tokenizer {

use func::{Func, FuncType, FuncExec};

pub struct Tokenizer {
    raw_func: String
}

impl Tokenizer {
    pub fn new(raw_func: String) -> Self {
        Self{
            raw_func
        }
    }

    pub process(&self) -> Func {

    }
}
}