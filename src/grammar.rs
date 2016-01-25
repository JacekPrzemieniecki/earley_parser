use std::fmt;
use std::fmt::Write;
use std::collections::HashSet;

pub struct Rule {
    pub id: usize,
    pub start: usize,
    pub symbols: Vec<usize>
}

pub struct Grammar<'a> {
    pub rules: Vec<Rule>,
    pub symbols: Vec<Symbol<'a>>,
    pub start: usize
}

impl<'a> Grammar<'a> {
    pub fn print_rule(&self, target: &mut String, rule: &Rule) {
        let mut sub = String::new();
        for sym in rule.symbols.iter().cloned() {
            write!(&mut sub, " {}", self.symbols[sym as usize].name).unwrap();
        }

        write!(target, "{} -> {}", self.symbols[rule.start as usize].name, sub).unwrap();
    }
}

impl<'a> fmt::Display for Grammar<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut s = format!("Start symbol: {}\n", self.symbols[self.start as usize].name);
        write!(&mut s, "Symbols: \n").unwrap();
        for symbol in self.symbols.iter() {
            write!(&mut s, "{}\n", symbol).unwrap();
        }
        write!(&mut s, "Rules: \n").unwrap();
        for rule in self.rules.iter() {
            self.print_rule(&mut s, rule);
            write!(s, "\n").unwrap();
        }
        return formatter.pad(&s);
    }
}

pub struct Symbol<'a> {
    pub id: usize,
    pub name: &'a str,
    pub is_terminal: bool
}

impl<'a> fmt::Display for Symbol<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let s = format!("[{}] {} terminal: {}", self.id, self.name, self.is_terminal);
        return formatter.pad(&s);
    }
}

pub fn build_grammar<'a>(bn_form: &'a str) -> Grammar<'a> {
    let mut symbols = Vec::<Symbol>::new();
    symbols.push(Symbol {name: "End of input", id: 0, is_terminal: true});
    let mut symbols_lut = HashSet::<&str>::new();
    let mut rules = Vec::new();
    for line in bn_form.lines() {
        let mut split = line.split(" ");

        let head = match split.next() {
            Some(x) => x,
            None => panic!("Error reading grammar on {}", line)

        };

        let head_symbol_id: usize = match symbols_lut.contains(head)
        {
            true => symbols.iter().find(|e| e.name == head).expect("Error building grammar {}: 164361").id,
            false => {
                let id = symbols.len();
                let new_symbol = Symbol {id: id, name: &head, is_terminal: false};
                symbols.push(new_symbol);
                id
            }
        };
        symbols[head_symbol_id].is_terminal = false;
        symbols_lut.insert(head);

        match split.next() {
            Some("->") => (),
            _ => panic!("Grammar syntax error: missing '=>' on line {}", line)
        };

        let mut tail_symbols = Vec::new();
        for elem in split {
            let elem_trimmed = elem.trim_matches('\'');
            let tail_elem_id: usize = match symbols_lut.contains(elem)
            {
                true => match symbols.iter().find(|e| e.name == elem_trimmed) {
                    Some(x) => x.id,
                    None => panic!("Error building grammar, searching for {}", elem)
                },
                false => {
                    let id = symbols.len();
                    let is_terminal = elem.starts_with("'");
                    let name = elem_trimmed;
                    let new_symbol = Symbol {id: id, name: &name, is_terminal: is_terminal};
                    symbols.push(new_symbol);
                    id
                }
            };
            tail_symbols.push(tail_elem_id);
            symbols_lut.insert(elem);
        }

        {
            let rule_id = rules.len();
            let new_rule = Rule {id: rule_id, start: head_symbol_id, symbols: tail_symbols};
            rules.push(new_rule);
        }
    }
    let g = Grammar {start: 1, rules: rules, symbols: symbols};
    return g;
}
