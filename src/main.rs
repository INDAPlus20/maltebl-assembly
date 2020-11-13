use std::{env, fs};
mod compiler;
use compiler::compile;
mod emulator;
use emulator::execute;
mod language_specs;
fn main() {
    let mut args = env::args();
    args.next();
    if let (Some(filepath), Some(output_file)) = (args.next(), args.next()) {
        if let Ok(code) = fs::read_to_string(filepath) {
            compile(code, &output_file).unwrap(); //DONT UNWRAP! HANDLE
            execute(output_file + ".formexe").unwrap();
        } else {
            println!("Error reading code file...")
        }
    } else {
        println!("Error finding file...")
    }
}
