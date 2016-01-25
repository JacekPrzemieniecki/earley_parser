use grammar::{Grammar, Rule, Symbol};
use std::fmt::{Write};
use tokenizer::Token;

#[derive(Clone, PartialEq, Eq)]
struct EarleyItem {
    rule_id: usize,
    position: usize,
    start: usize,
}


impl EarleyItem {
    fn get_current_symbol<'a>(&'a self, gram: &'a Grammar<'a>) -> &Symbol {
        let rule = self.get_rule(gram);
        let current_symbol = rule.symbols[self.position as usize];
        return &gram.symbols[current_symbol as usize];
    }

    fn get_rule<'a>(&self, gram: &'a Grammar) -> &'a Rule {
        &gram.rules[self.rule_id]
    }

    fn get_symbols<'a>(&self, gram: &'a Grammar) -> &'a Vec<usize> {
        &self.get_rule(gram).symbols
    }

    fn is_complete(&self, gram: &Grammar) -> bool {
        self.get_symbols(gram).len() == self.position as usize
    }

    fn fits(&self, gram: &Grammar, other_item: &EarleyItem) -> bool {
        self.get_current_symbol(gram).id == gram.rules[other_item.rule_id as usize].start
    }

    fn to_string(&self, gram: &Grammar) -> String {
        let mut f = String::new();
        let rule = self.get_rule(gram);
        let rule_symbols = self.get_symbols(gram);
        write!(f, "({}) ", self.start).unwrap();
        write!(f, "{} -> ", gram.symbols[rule.start].name).unwrap();
        for (i, sym) in rule_symbols.iter().enumerate() {
            if i == self.position as usize {
                write!(f, " \\*/ ").unwrap();
            }
            write!(f, "{} ", gram.symbols[sym.clone()].name).unwrap();
        }
        if self.is_complete(gram) {
            write!(f, " \\*/ ").unwrap();
        }
        return f;
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
            //complete parse
            for past_item in &old[item.start as usize] {
                if !past_item.is_complete(&gram) && past_item.fits(&gram, &item) {
                    let new_item = EarleyItem {rule_id: past_item.rule_id, position: past_item.position + 1, start: past_item.start};
                    if !current_state_set.contains(&new_item) {
                        current_state_set.push(new_item);
                    }
                }
            }
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
                for rule in gram.rules.iter().filter(|&r| r.start == next_symbol.id) {
                    let new_item = EarleyItem {rule_id: rule.id, position: 0, start: index as usize};
                    if !current_state_set.contains(&new_item) {
                        current_state_set.push(new_item);
                    }
                }
            }
        }
        i += 1;
    }
}

fn reverse_and_sort(state_sets: &Vec<Vec<EarleyItem>>, gram: &Grammar) -> Vec<Vec<EarleyItem>> {
    let mut result = Vec::new();
    for _ in 0..state_sets.len() {
        result.push(Vec::new());
    }
    for (i, state_set) in state_sets.iter().rev().enumerate() {
        let bin = state_sets.len() - i - 1;
        for item in state_set.iter().filter(|i| i.is_complete(&gram)) {
            let start = item.start;
            let copy = EarleyItem {rule_id: item.rule_id, position: item.position, start: bin};
            result[start].push(copy);
        }
    }
    return result;
}

fn print_state_sets(state_sets: &Vec<Vec<EarleyItem>>, gram: &Grammar, tokens: &Vec<Token>, print_tokens: bool) {
    for (set_index, set) in state_sets.iter().enumerate() {
        for item in set {
            println!("{}", item.to_string(&gram));
        }
        if set_index < tokens.len() {
            if print_tokens {
                println!("{}: {}", set_index, tokens[set_index].text);
            }
            else {
                println!("{}", set_index);
            }
        }
    }
}

struct ASTNode {
    children: Vec<ASTNode>,
    value: usize
}

fn find_split<'a>(
    from: usize, 
    to: usize, 
    symbols: &[usize], 
    lookup_sets: &'a Vec<Vec<EarleyItem>>, 
    gram: &Grammar
    ) -> Option<Vec<&'a EarleyItem>> {

    if from == to && symbols.len() == 0 {
        return Some(Vec::new());
    }
    let (symbol, rest) = match symbols.split_first() {
        Some((x, y)) => (x, y),
        None => return None
    };
    for item in lookup_sets[from].iter().filter(
            |e| e.start <= to && &gram.rules[e.rule_id].start == symbol
        ) {
        match find_split(item.start, to, rest, &lookup_sets, &gram) {
            Some(mut v) => { v.push(item); return Some(v);}
            None => {}
        }
    }
    return None;
}

fn build_ast_internal(
    from: usize, 
    split: &Vec<&EarleyItem>,
    lookup_sets: &Vec<Vec<EarleyItem>>, 
    gram: &Grammar) -> Option<Vec<ASTNode>> {

    let mut ast_children = Vec::new();
    let mut inner_from = from;
    for item in split.iter().rev() {
        let child = match build_ast(inner_from, item.start, gram.rules[item.rule_id].start, &lookup_sets, &gram) {
            Some(sub_tree) => sub_tree,
            None => return None
        };
        ast_children.push(child);
        inner_from = item.start;
    }
    return Some(ast_children);
}

fn build_ast(
    from: usize, 
    to: usize, 
    root_symbol: usize, 
    lookup_sets: &Vec<Vec<EarleyItem>>, 
    gram: &Grammar) -> Option<ASTNode> {
    
    if gram.symbols[root_symbol].is_terminal {
        return Some(ASTNode {children: Vec::new(), value: root_symbol});
    }
    let edges : Vec<_> = lookup_sets[from].iter().filter(|item| item.start == to ).collect();
    if edges.is_empty() {
        return None;
    }

    for edge in edges {
        if gram.rules[edge.rule_id].symbols.iter().all(|i| gram.symbols[i.clone()].is_terminal) {
            let mut children = Vec::new();
            for terminal in &gram.rules[edge.rule_id].symbols {
                children.push(ASTNode {children: Vec::new(), value: terminal.clone()});
            }
            return Some(ASTNode {children: children, value: root_symbol});
        }
        let split = match find_split(from, to, &gram.rules[edge.rule_id].symbols[..], &lookup_sets, &gram) {
            Some(x) => x,
            None => continue
        };
        // we found a split, let's try to make a parse tree out of it
        let ast_children = match build_ast_internal(from, &split, &lookup_sets, &gram) {
            Some(x) => x,
            None => continue
        };
        return Some(ASTNode {children: ast_children,  value: root_symbol});
    }
    return None
}

impl ASTNode {
    fn to_string(&self, gram: &Grammar, indent: &str, last: bool) -> String {
        let mut s = String::new();
        write!(s, "{}", indent);
        let new_indent = match last {
            true => { write!(s, "\\-"); indent.to_string() + "  "},
            false => { write!(s, "|-"); indent.to_string() + "| "}
        };
        write!(s, "{}\n", &gram.symbols[self.value].name);
        for (i, node) in self.children.iter().enumerate() {
            write!(s, "{}", node.to_string(&gram, &new_indent, i == self.children.len() -1));
        }
        return s;
    }
}
pub fn parse(gram: Grammar, tokens: Vec<Token>) {
    let mut state_sets = Vec::new();
    
    // initial state set
    {
        let mut init_state_set = Vec::new();
        for rule in gram.rules.iter().filter(|&r| r.start == gram.start) {
            let mut s = String::new();
            gram.print_rule(&mut s, &rule);
            init_state_set.push(EarleyItem {rule_id: rule.id, position: 0, start: 0});
        }
        state_sets.push(init_state_set);
    }

    for (index, token) in tokens.iter().enumerate() {
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
        }
        state_sets.push(next_state_set);
    }

    let reversed = reverse_and_sort(&state_sets, &gram);

    let ast = match build_ast(0, tokens.len()-1, gram.start, &reversed, &gram) {
        Some(x) => x,
        None => panic!("Failed to build ast")
    };
    println!("{}", gram);

    print_state_sets(&state_sets, &gram, &tokens, true);

    println!("\n\n");
    println!("{}", ast.to_string(&gram, "", true));
}
