use std::collections::HashMap;
use grammar::{Symbol, Grammar};

pub struct Token<'a> {
    pub symbol: usize,
    pub text: &'a str
}

pub fn tokenize<'a, 'b>(grammar: &Grammar, text: &'a str) -> Vec<Token<'a>>{
    let symbols = &grammar.symbols;
    let text_trimmed = text.trim();
    let mut terminals: HashMap<String, &Symbol> = HashMap::new();
    for symbol in symbols {
        if symbol.is_terminal {
            terminals.insert(symbol.name.to_string(), symbol);
        }
    }

    let mut result = Vec::new();
    for c in text_trimmed.split(" ") {
        let opt = terminals.get(&c.to_string());
        let sym = opt.unwrap_or_else(|| {panic!("Unrecognized token: {}", c);});
        let new_token = Token {symbol: sym.id, text: c};
        result.push(new_token);
    }
    result.push(Token {symbol: 0, text: "End of input"});
    return result;
}
