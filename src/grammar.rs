use std::fmt;
use std::fmt::Write;

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
        for rule in self.rules.iter() {
            self.print_rule(&mut s, rule);
            write!(s, "\n");
        }
        return formatter.pad(&s);
    }
}

pub struct Symbol<'a> {
    pub id: usize,
    pub name: &'a str,
    pub is_terminal: bool
}

pub fn build_grammar(bn_form: &str) -> Grammar {
    let symbols = vec![
        Symbol {name: "End of input", id: 0, is_terminal: true},
        Symbol {name: "Sum", id: 1, is_terminal: false},
        Symbol {name: "Number", id: 2, is_terminal: false},
        Symbol {name: "+", id: 3, is_terminal: true},
        Symbol {name: "1", id: 4, is_terminal: true},
    ];
    let rules = vec![
        Rule {id: 0, start: 1, symbols: vec![2, 3, 1]},
        Rule {id: 1, start: 1, symbols: vec![2]},
        Rule {id: 2, start: 2, symbols: vec![4]},
        Rule {id: 3, start: 2, symbols: vec![2, 4]},
        ];
    let g = Grammar {start: 1, rules: rules, symbols: symbols};
    return g;
}
