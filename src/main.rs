use compiler::compile;
use std::{env, fs, io};
fn main() {
    let mut args = env::args();
    args.next();
    if let Some(filepath) = args.next() {
        if let Ok(code) = fs::read_to_string(filepath) {
            compile(code).unwrap(); //DONT UNWRAP! HANDLE
        } else {
            println!("Error reading code file...")
        }
    } else {
        println!("Error finding file...")
    }
}
