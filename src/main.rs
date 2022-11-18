use std::env;
use std::fs;
mod interpreter;
mod lang_spec;
fn main() {
    // let args: Vec<String> = env::args().collect();
    // let file_path = &args[1];
    // let arg = &args[2];
    let file_path = "factorial.fesu";
    let arg = "execute";

    let file_contents = fs::read_to_string(file_path).expect("Error parsing file path.");

    match arg {
        "execute" => interpreter::execute(file_contents),
        _ => panic!("Error parsing arguments!"),
    }
}
