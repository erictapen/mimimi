use std::fmt;
use std::collections::HashSet;

// run `cargo test` for these
#[cfg(test)]
mod tests {
    use Theorem;
    use Symbol;


    #[test]
    fn test_rule_1_appends_u() {
        let mi: Theorem = Theorem(vec![Symbol::M, Symbol::I]);
        let miu: Theorem = Theorem(vec![Symbol::M, Symbol::I, Symbol::U]);
        assert!(mi.rule_1() == Some(miu));
    }

    #[test]
    fn test_rule_1_fails_on_u_as_last_one() {
        let miu: Theorem = Theorem(vec![Symbol::M, Symbol::I, Symbol::U]);
        assert!(miu.rule_1() == None);
    }
    #[test]
    fn test_rule_1_is_okay_on_empty_theorem() {
        let empty: Theorem = Theorem(vec![]);
        assert!(empty.rule_1() == None);
    }
    #[test]
    fn test_rule_2_is_okay_on_empty_theorem() {
        let empty: Theorem = Theorem(vec![]);
        assert!(empty.rule_2() == None);
    }
    #[test]
    fn test_rule_2_duplicates_empty() {
        let m: Theorem = Theorem(vec![Symbol::M]);
        assert!(m.rule_2() == Some(m));
    }
    #[test]
    fn test_rule_2_duplicates_sequence() {
        let mmu: Theorem = Theorem(vec![Symbol::M, Symbol::M, Symbol::U]);
        let mmumu: Theorem = Theorem(vec![Symbol::M, Symbol::M, Symbol::U, Symbol::M, Symbol::U]);
        assert!(mmu.rule_2() == Some(mmumu));
    }
    #[test]
    fn test_rule_3_is_okay_on_empty_theorem() {
        let empty: Theorem = Theorem(vec![]);
        assert!(empty.rule_3(0) == None);
        let empty: Theorem = Theorem(vec![]);
        assert!(empty.rule_3(1) == None);
    }
    #[test]
    fn test_rule_3_applies_not() {
        let mmu: Theorem = Theorem(vec![Symbol::M, Symbol::M, Symbol::U]);
        assert!(mmu.rule_3(1) == None);
    }
    #[test]
    fn test_rule_3_applies_on_single_occasion() {
        let miiim: Theorem = Theorem(vec![Symbol::M, Symbol::I, Symbol::I, Symbol::I, Symbol::M]);
        let mum: Theorem = Theorem(vec![Symbol::M, Symbol::U, Symbol::M]);
        match miiim.rule_3(1) {
            None => println!("{}", "Fail"),
            Some(theorem) => println!("{}", theorem),
        }
        assert!(miiim.rule_3(1) == Some(mum));
    }
    #[test]
    fn test_rule_3_applies_on_first_occasion() {
        let miiimimiiim: Theorem = Theorem(vec![
            Symbol::M,
            Symbol::I,
            Symbol::I,
            Symbol::I,
            Symbol::M,
            Symbol::I,
            Symbol::M,
            Symbol::I,
            Symbol::I,
            Symbol::I,
            Symbol::M,
        ]);
        let mumimiiim: Theorem = Theorem(vec![
            Symbol::M,
            Symbol::U,
            Symbol::M,
            Symbol::I,
            Symbol::M,
            Symbol::I,
            Symbol::I,
            Symbol::I,
            Symbol::M,
        ]);
        assert!(miiimimiiim.rule_3(1) == Some(mumimiiim));
    }
    #[test]
    fn test_rule_3_applies_on_second_occasion() {
        let miiimimiiim: Theorem = Theorem(vec![
            Symbol::M,
            Symbol::I,
            Symbol::I,
            Symbol::I,
            Symbol::M,
            Symbol::I,
            Symbol::M,
            Symbol::I,
            Symbol::I,
            Symbol::I,
            Symbol::M,
        ]);
        let miiimimum: Theorem = Theorem(vec![
            Symbol::M,
            Symbol::I,
            Symbol::I,
            Symbol::I,
            Symbol::M,
            Symbol::I,
            Symbol::M,
            Symbol::U,
            Symbol::M,
        ]);
        assert!(miiimimiiim.rule_3(2) == Some(miiimimum));
    }
    #[test]
    fn test_rule_3_on_just_iii() {
        let iii: Theorem = Theorem(vec![Symbol::I, Symbol::I, Symbol::I]);
        let u: Theorem = Theorem(vec![Symbol::U]);
        assert!(iii.rule_3(1) == Some(u));
    }
    #[test]
    fn test_rule_4_is_okay_on_empty_theorem() {
        let empty: Theorem = Theorem(vec![]);
        assert!(empty.rule_4(0) == None);
        let empty: Theorem = Theorem(vec![]);
        assert!(empty.rule_4(1) == None);
    }
    #[test]
    fn test_rule_4_applies_not() {
        let mmu: Theorem = Theorem(vec![Symbol::M, Symbol::M, Symbol::U]);
        assert!(mmu.rule_4(1) == None);
    }
    #[test]
    fn test_rule_4_applies_on_single_occasion() {
        let muum: Theorem = Theorem(vec![Symbol::M, Symbol::U, Symbol::U, Symbol::M]);
        let mm: Theorem = Theorem(vec![Symbol::M, Symbol::M]);
        assert!(muum.rule_4(1) == Some(mm));
    }
    #[test]
    fn test_rule_4_applies_on_first_occasion() {
        let muumumuum: Theorem = Theorem(vec![
            Symbol::M,
            Symbol::U,
            Symbol::U,
            Symbol::M,
            Symbol::U,
            Symbol::M,
            Symbol::U,
            Symbol::U,
            Symbol::M,
        ]);
        let mmumuum: Theorem = Theorem(vec![
            Symbol::M,
            Symbol::M,
            Symbol::U,
            Symbol::M,
            Symbol::U,
            Symbol::U,
            Symbol::M,
        ]);
        assert!(muumumuum.rule_4(1) == Some(mmumuum));
    }
    #[test]
    fn test_rule_4_applies_on_second_occasion() {
        let muumumuum: Theorem = Theorem(vec![
            Symbol::M,
            Symbol::U,
            Symbol::U,
            Symbol::M,
            Symbol::U,
            Symbol::M,
            Symbol::U,
            Symbol::U,
            Symbol::M,
        ]);
        let muumumm: Theorem = Theorem(vec![
            Symbol::M,
            Symbol::U,
            Symbol::U,
            Symbol::M,
            Symbol::U,
            Symbol::M,
            Symbol::M,
        ]);
        assert!(muumumuum.rule_4(2) == Some(muumumm));
    }
    #[test]
    fn test_rule_4_on_just_uu() {
        let uu: Theorem = Theorem(vec![Symbol::U, Symbol::U]);
        let empty: Theorem = Theorem(vec![]);
        assert!(uu.rule_4(1) == Some(empty));
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Hash)]
#[derive(Eq)]
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
#[derive(Debug)]
#[derive(Hash)]
#[derive(Eq)]
#[derive(Clone)]
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

    /// Rule 2: `Mx -> Mxx` or if a String starts with M, the following part can be doubled.
    fn rule_2(&self) -> Option<Theorem> {
        if !&self.0.is_empty() && &self.0[0] == &Symbol::M {
            let mut result = self.0.to_vec();
            for i in 1..self.0.len() {
                result.push(self.0[i].clone());
            }
            Some(Theorem(result))
        } else {
            None
        }
    }

    /// Rule 3: `xIIIy -> xUy` or replace the n'th III with a U.
    fn rule_3(&self, n: i32) -> Option<Theorem> {
        if self.0.is_empty() || n < 1 {
            return None;
        }
        let mut counter = n;
        let mut cursor: usize = std::usize::MAX;
        let mut c: usize = 0;
        for i in 0..self.0.len() {
            if &self.0[i] == &Symbol::I {
                c += 1;
            } else {
                c = 0;
            }
            if c >= 3 {
                if counter > 1 {
                    c = 0;
                    counter -= 1;
                } else {
                    cursor = i - 2;
                    break;
                }
            }
        }
        if cursor == std::usize::MAX {
            return None;
        }
        let mut result: Vec<Symbol> = vec![];
        for i in 0..cursor {
            result.push(self.0[i].clone());
        }
        result.push(Symbol::U);
        for i in (cursor + 3)..self.0.len() {
            result.push(self.0[i].clone());
        }
        Some(Theorem(result))
    }

    // Rule 4: `xUUy -> xy` or cut out the n'th UU.
    fn rule_4(&self, n: i32) -> Option<Theorem> {
        if self.0.is_empty() || n < 1 {
            return None;
        }
        let mut counter = n;
        let mut cursor: usize = std::usize::MAX;
        let mut c: usize = 0;
        for i in 0..self.0.len() {
            if &self.0[i] == &Symbol::U {
                c += 1;
            } else {
                c = 0;
            }
            if c >= 2 {
                if counter > 1 {
                    c = 0;
                    counter -= 1;
                } else {
                    cursor = i - 1;
                    break;
                }
            }
        }
        if cursor == std::usize::MAX {
            return None;
        }
        let mut result: Vec<Symbol> = vec![];
        for i in 0..cursor {
            result.push(self.0[i].clone());
        }
        for i in (cursor + 2)..self.0.len() {
            result.push(self.0[i].clone());
        }
        Some(Theorem(result))
    }
}

fn main() {
    let mi = Theorem(vec![Symbol::M, Symbol::I]);
    let mut wellknown_theorems: HashSet<Theorem> = HashSet::new();
    let mut new_theorems: HashSet<Theorem> = HashSet::new();

    new_theorems.insert(mi);

    loop {
        let mut temp: HashSet<Theorem> = HashSet::new();
        let mut clone: HashSet<Theorem> = new_theorems.clone();
        for x in clone.drain() {
            if let Some(t) = x.rule_1() {
                if !wellknown_theorems.contains(&t) && t.0.len() < 100 {
                    //println!("{} -> {}", &x, &t);
                    //println!("{}", &t);
                    temp.insert(t);
                }
            }

            if let Some(t) = x.rule_2() {
                if !wellknown_theorems.contains(&t) && t.0.len() < 100 {
                    //println!("{} -> {}", &x, &t);
                    //println!("{}", &t);
                    temp.insert(t);
                }
            }

            for i in 1.. {
                match x.rule_3(i) {
                    Some(t) => {
                        if !wellknown_theorems.contains(&t) && t.0.len() < 100 {
                            //println!("{} -> {}", &x, &t);
                            //println!("{}", &t);
                            temp.insert(t);
                        }
                    },
                    None => break
                }
            }

            for i in 1.. {
                match x.rule_4(i) {
                    Some(t) => {
                        if !wellknown_theorems.contains(&t) && t.0.len() < 100 {
                            //println!("{} -> {}", &x, &t);
                            //println!("{}", &t);
                            temp.insert(t);
                        }
                    },
                    None => break
                }
            }

            wellknown_theorems.insert(x);
            //println!("wellknown: {:?}", wellknown_theorems);
        }
        for x in temp.drain() {
            if !new_theorems.contains(&x) {
                println!("{}", &x);
                new_theorems.insert(x);
            }
        }

    }

}
