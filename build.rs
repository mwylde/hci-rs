extern crate protogen_compiler;

use std::env;

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("src");
    path.push("protogen");

    protogen_compiler::process_dir(&path).unwrap();
}
