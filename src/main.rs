use std::io::prelude::*;
use std::fs::File;
use std::env;

mod grammar;
mod parser;
use grammar::build_grammar;

fn read_all(file_name: &str) -> String {
    let mut f = File::open(file_name).unwrap_or_else(|e| {panic!("Error reading file {}: {}", file_name, e);});
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap_or_else(|e| {panic!("Error reading file {}: {}", file_name, e);});
    return buffer;
}

static USAGE_MESSAGE : &'static str = "Usage:\n earley_parser <grammar file> <text_file>";

fn main() {
    let arguments : Vec<_> = env::args().collect();
    let arg_count = arguments.len();
    if arg_count != 3 {
        panic!(USAGE_MESSAGE);
    }
    let to_parse = read_all(&arguments[2]);
    let grammar_str = read_all(&arguments[1]); 
    let gram = grammar::build_grammar(&grammar_str);
    parser::parse(gram, &to_parse);
}
