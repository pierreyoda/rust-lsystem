use std::hash::Hash;
use std::collections::HashMap;


/// The production rules describe how an L-System should evolve to its next
/// state.
/// A production (or evolution) rule consists in a symbol combined with its
/// corresponding result. The internal data structure is up to the structures
/// implementing this trait.
pub trait LRules<S> {
    /// Get the production of the given symbol, or None if no matching rule is
    /// found.
    fn production(&self, symbol: &S) -> Option<&Vec<S>>;
}

/// HashMap-based Rules structure.
#[derive(Clone)]
pub struct HashMapRules<S>
    where S: Eq + Hash
{
    rules: HashMap<S, Vec<S>>,
}

impl<S> HashMapRules<S> where S: Eq + Hash
{
    pub fn new() -> HashMapRules<S> {
        HashMapRules { rules: HashMap::new() }
    }

    /// Add a new production rule or modify an existing one.
    /// Return true if an existing rule was modified, false otherwise.
    pub fn set(&mut self, symbol: S, production: Vec<S>) -> bool {
        match self.rules.insert(symbol, production) {
            Some(_) => true,
            None => false,
        }
    }
}

impl HashMapRules<char> {
    /// Convenience method for calling 'set' directly with an str slice.
    pub fn set_str(&mut self, symbol: char, production: &str) -> bool {
        self.set(symbol, production.chars().collect())
    }
}

impl<S> LRules<S> for HashMapRules<S> where S: Eq + Hash
{
    fn production(&self, symbol: &S) -> Option<&Vec<S>> {
        self.rules.get(symbol)
    }
}

/// Default, ASCII-only 'HashMapRules' type.
pub type AsciiHashMapRules = HashMapRules<u8>;

impl AsciiHashMapRules {
    pub fn set_ascii(&mut self, symbol: u8, production: &[u8]) -> bool {
        self.set(symbol, production.to_vec())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hashmap_rules_char() {
        let mut rules: HashMapRules<char> = HashMapRules::new();
        rules.set_str('A', "+B−A−B+");
        rules.set_str('B', "−A+B+A−");

        assert_eq!(rules.production(&'A'),
                   Some(&"+B−A−B+".chars().collect()));
        assert_eq!(rules.production(&'B'),
                   Some(&"−A+B+A−".chars().collect()));
        assert_eq!(rules.production(&'C'), None);
    }

    #[test]
    fn hashmap_rules_ascii() {
        let mut rules = AsciiHashMapRules::new();
        rules.set_ascii(b'A', b"AB");
        rules.set_ascii(b'B', b"A");

        assert_eq!(rules.production(&b'A'), Some(&b"AB".to_vec()));
        assert_eq!(rules.production(&b'B'), Some(&b"A".to_vec()));
        assert_eq!(rules.production(&b'C'), None);
    }
}
