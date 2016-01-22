use std::fmt;
use std::fmt::Write;

pub struct Rule {
    pub id: u32,
    pub name: u32,
    pub symbols: Vec<u32>
}

pub struct Grammar<'a> {
    pub rules: Vec<Rule>,
    pub symbols: Vec<Symbol<'a>>,
    pub start: u32
}

impl<'a> Grammar<'a> {
    fn print_rule(&self, target: &mut String, rule: &Rule) {
        let mut sub = String::new();
        for sym in rule.symbols.iter().cloned() {
            write!(&mut sub, " {}", self.symbols[sym as usize].name).unwrap();
        }

        write!(target, "{} -> {}\n", self.symbols[rule.name as usize].name, sub).unwrap();
    }
}

impl<'a> fmt::Display for Grammar<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut s = format!("Start symbol: {}\n", self.symbols[self.start as usize].name);
        for rule in self.rules.iter() {
            self.print_rule(&mut s, rule);
        }
        return formatter.pad(&s);
    }
}

pub struct Symbol<'a> {
    pub id: u32,
    pub name: &'a str,
    pub is_terminal: bool
}

pub fn build_grammar(bn_form: &str) -> Grammar {
    let symbols = vec![
        Symbol {name: "Sum", id: 0, is_terminal: false},
        Symbol {name: "Number", id: 1, is_terminal: false},
        Symbol {name: "+", id: 2, is_terminal: true},
        Symbol {name: "1", id: 3, is_terminal: true},
        Symbol {name: "End of input", id: 4, is_terminal: true},
    ];
    let rules = vec![
        Rule {id: 0, name: 0, symbols: vec![1, 2, 0]},
        Rule {id: 1, name: 0, symbols: vec![1]},
        Rule {id: 2, name: 1, symbols: vec![3]},
        Rule {id: 3, name: 1, symbols: vec![1, 3]},
        ];
    let g = Grammar {start: 0, rules: rules, symbols: symbols};
    return g;
}
