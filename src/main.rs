use std::io::prelude::*;
use std::fs::File;
use std::env;

mod grammar;
mod tokenizer;
mod parser;
use grammar::build_grammar;
use tokenizer::tokenize;

fn read_all(file_name: &str) -> String {
    let mut f = File::open(file_name).unwrap_or_else(|e| {panic!("Error reading file {}: {}, \n{}", file_name, e, USAGE_MESSAGE);});
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap_or_else(|e| {panic!("Error reading file {}: {}, \n{}", file_name, e, USAGE_MESSAGE);});
    return buffer;
}

static USAGE_MESSAGE : &'static str = "Usage:\n earley_parser <grammar file> <tokens file> <text_file>";

fn main() {
    let arguments : Vec<_> = env::args().collect();
    let arg_count = arguments.len();
    if arg_count != 3 {
        panic!(USAGE_MESSAGE);
    }

    let to_parse = read_all(&arguments[2]);
    let grammar_str = read_all(&arguments[1]); 
    let gram = grammar::build_grammar(&grammar_str);
    let tokens = tokenize(&gram, &to_parse);
    parser::parse(gram, tokens);
}
