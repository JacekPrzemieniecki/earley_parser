use std::collections::HashMap;
use grammar::{Grammar, Rule, Symbol};
use std::fmt::{Write};

struct Token<'a> {
    symbol: u32,
    text: &'a str
}

#[derive(Clone, PartialEq, Eq)]
struct EarleyItem {
    rule_id: u32,
    position: u32,
    start: u32,
}


impl EarleyItem {
    fn get_current_symbol<'a>(&'a self, gram: &'a Grammar<'a>) -> &Symbol {
        let rule = self.get_rule(gram);
        let current_symbol = rule.symbols[self.position as usize];
        return &gram.symbols[current_symbol as usize];
    }

    fn get_rule<'a>(&self, gram: &'a Grammar) -> &'a Rule {
        &gram.rules[self.rule_id as usize]
    }

    fn get_symbols<'a>(&self, gram: &'a Grammar) -> &'a Vec<u32> {
        &self.get_rule(gram).symbols
    }

    fn is_complete(&self, gram: &Grammar) -> bool {
        self.get_symbols(gram).len() == self.position as usize
    }

    fn fits(&self, gram: &Grammar, other_item: &EarleyItem) -> bool {
        self.get_current_symbol(gram).id == gram.rules[other_item.rule_id as usize].name
    }

    fn to_string(&self, gram: &Grammar) -> String {
        let mut f = String::new();
        let rule = self.get_rule(gram);
        let rule_symbols = self.get_symbols(gram);
        write!(f, "({}) ", self.start).unwrap();
        write!(f, "{} -> ", gram.symbols[rule.name as usize].name).unwrap();
        for (i, sym) in rule_symbols.iter().enumerate() {
            if i == self.position as usize {
                write!(f, " \\*/ ").unwrap();
            }
            write!(f, "{} ", gram.symbols[sym.clone() as usize].name).unwrap();
        }
        if self.is_complete(gram) {
            write!(f, " \\*/ ").unwrap();
        }
        return f;
    }
}

fn tokenize<'a>(symbols: &Vec<Symbol>, text: &'a str) -> Vec<Token<'a>>{
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
    result.push(Token {symbol: 4, text: "End of input"});
    return result;
}

fn complete_partial_parse(item: &EarleyItem, gram: &Grammar, current_state_set: &mut Vec<EarleyItem>, old: &[Vec<EarleyItem>]) {
    for past_item in &old[item.start as usize] {
        if !past_item.is_complete(&gram) && past_item.fits(&gram, &item) {
            let new_item = EarleyItem {rule_id: past_item.rule_id, position: past_item.position + 1, start: past_item.start};
            if !current_state_set.contains(&new_item) {
                current_state_set.push(new_item);
            }
        }
    }
}

fn process_state_set(
    current_state_set: &mut Vec<EarleyItem>, 
    next_state_set: &mut Vec<EarleyItem>, 
    gram: &Grammar, 
    old: &[Vec<EarleyItem>],
    token: &Token,
    index: u32
    ) {
    let mut i = 0;
    while i < current_state_set.len() {
        let item = current_state_set[i as usize].clone();
        let rule_len = gram.rules[item.rule_id as usize].symbols.len();
        if item.position as usize >= rule_len {
            complete_partial_parse(&item, &gram, current_state_set, old); 
        }
        else {
            // incomplete parse
            let next_symbol = item.get_current_symbol(&gram);

            if next_symbol.is_terminal { 
                // scan current symbol
                if next_symbol.id == token.symbol {
                    let new_item = EarleyItem {rule_id: item.rule_id, position: item.position + 1, start: item.start};
                    if !next_state_set.contains(&new_item) {
                        next_state_set.push(new_item);
                    }
                }
            }
            else {
                // predict next rule
                for rule in gram.rules.iter().filter(|&r| r.name == next_symbol.id) {
                    let new_item = EarleyItem {rule_id: rule.id, position: 0, start: index as u32};
                    if !current_state_set.contains(&new_item) {
                        current_state_set.push(new_item);
                    }
                }
            }
        }
        i += 1;
    }
}

pub fn parse(gram: Grammar, text: &str) {
    let mut state_sets = Vec::new();
    let tokens = tokenize(&gram.symbols, text);

    let mut init_state_set = Vec::new();
    for rule in gram.rules.iter().filter(|&r| r.name == gram.start) {
        init_state_set.push(EarleyItem {rule_id: rule.name, position: 0, start: 0});
    }
    state_sets.push(init_state_set);

    for (index, token) in tokens.iter().enumerate() {
        println!("parsing token {}: {}", index, token.text);
        let mut next_state_set = Vec::new();
    
        {
            let (old, next) = state_sets.split_at_mut(index as usize);
            let mut current_state_set = &mut next[0];

            process_state_set(current_state_set, 
                              &mut next_state_set,
                              &gram,
                              old,
                              token,
                              index as u32);
            println!("finished item set {}", index);
        }
        state_sets.push(next_state_set);
    }

    println!("{}", gram);

    for (set_index, set) in state_sets.iter().enumerate() {
        for item in set {
            println!("{}", item.to_string(&gram));
        }
        if set_index < tokens.len() {
            println!("{}: {}", set_index, tokens[set_index].text);
        }
    }
}
