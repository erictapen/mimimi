use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
enum Symbol {
    M,
    I,
    U,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Symbol::M => write!(f, "M"),
            Symbol::I => write!(f, "I"),
            Symbol::U => write!(f, "U"),
        }
    }
}

struct Theorem(Vec<Symbol>);

impl fmt::Display for Theorem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for ele in &self.0[0..self.0.len() - 1] {
            result.push_str(&ele.to_string());
        }

        result.push_str(&self.0[self.0.len() - 1].to_string());
        write!(f, "{}", result)
    }
}

/*
 * In this part, the 4 transformation rules are implemented by four rust functions.
 */

impl Theorem {
    // Rule 1: `xI -> xIU` or every String ending on I can be appended by an U.
    fn rule_1(&self) -> Option<Theorem> {
        if &self.0[&self.0.len() - 1] == &Symbol::I {
            Some(Theorem(vec![Symbol::U]))
        } else {
            None
        }
    }
    
}


fn main() {
    let x = Theorem(vec![Symbol::M,
             Symbol::I,
        //   Symbol::U,
        ]);
    match x.rule_1() {
        None => println!("{}", "Fail"),
        Some(theorem) => println!("{}", theorem),
    }
}
