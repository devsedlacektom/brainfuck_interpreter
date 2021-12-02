mod file_handling;
mod interpreter;

use self::file_handling::{cli_init, get_file_path, open_desired_brainfuck_file};
use self::interpreter::Interpreter;

fn main() {
    let mut interpreter = Interpreter::new();
    let mut brainfuck_instructions = open_desired_brainfuck_file(get_file_path(cli_init()));

    interpreter.initialize_instructions(&mut brainfuck_instructions);
    interpreter.interpret();
}
