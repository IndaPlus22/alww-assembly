use std::env;
use std::fs;
mod compiler;
mod emulator;
fn main() {
    // let args: Vec<String> = env::args().collect();
    // let file_path = &args[1];
    // let arg = &args[2];

    let file_path = "test.txt";
    let arg = "compile";

    let file_contents = fs::read_to_string(file_path).expect("Error parsing file path.");

    match arg {
        "compile" => compiler::compile(file_contents),
        "execute" => emulator::execute(file_contents),
        _ => panic!("Error parsing arguments!"),
    }
}
