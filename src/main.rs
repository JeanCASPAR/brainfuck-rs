pub mod interpreter;
#[allow(dead_code)]
pub mod parser;

use std::path::Path;

fn main() {
    let path = Path::new("./hello_world");
    let parser = parser::Parser::from_path(path).unwrap();
    let instructions = parser.parse().unwrap();
    let mut interpreter = interpreter::Interpreter::new(instructions);
    interpreter.run();
}
