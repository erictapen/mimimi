use std::fmt;

// run `cargo test` for these
#[cfg(test)]
mod tests {
    use Theorem;
    use Symbol;


    #[test]
    fn test_rule_1_appends_u() {
        let mi: Theorem = Theorem(vec![Symbol::M,
                                        Symbol::I,
        ]);
        let miu: Theorem = Theorem(vec![Symbol::M,
                                        Symbol::I,
                                        Symbol::U,
        ]);
        assert!(mi.rule_1() == Some(miu));
    }

    #[test]
    fn test_rule_1_fails_on_u_as_last_one() {
        let miu: Theorem = Theorem(vec![Symbol::M,
                                        Symbol::I,
                                        Symbol::U,
        ]);
        assert!(miu.rule_1() == None);
    }
    #[test]
    fn test_rule_1_is_okay_on_empty_theorem() {
        let empty: Theorem = Theorem(vec![]);
        assert!(empty.rule_1() == None);
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
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

#[derive(PartialEq)]
pub struct Theorem(Vec<Symbol>);

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
 * In this part, the four transformation rules are implemented by four rust functions.
 */

impl Theorem {
    /// Rule 1: `xI -> xIU` or every String ending on I can be appended by an U.
    fn rule_1(&self) -> Option<Theorem> {
        if !&self.0.is_empty() && &self.0[&self.0.len() - 1] == &Symbol::I {
            let mut result = self.0.to_vec();
            result.push(Symbol::U);
            Some(Theorem(result))
        } else {
            None
        }
    }
}


fn main() {
    let mi = Theorem(vec![Symbol::M,
             Symbol::I,
        //   Symbol::U,
        ]);
    match mi.rule_1() {
        None => println!("{}", "Fail"),
        Some(theorem) => println!("{}", theorem),
    }
}
