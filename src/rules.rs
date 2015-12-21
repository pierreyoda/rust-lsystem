use std::hash::Hash;
use std::collections::HashMap;

use super::interpret::TurtleCommand;

/// These rules describe how an L-System should evolve to its next
/// state and how this state should be interpreted in order to draw it.
/// These two different concepts are stored alongside to facilitate the definition
/// of L-System, both in code and in file.
///
/// A production (or evolution) rule consists in a symbol combined with its
/// corresponding result. The internal data structure is up to the structures
/// implementing this trait.
///
/// See 'TurtleCommand' for further detail on the interpretation rules.
pub trait LRules<S: Eq> {
    /// Get the production of the given symbol, or None if no matching rule is
    /// found.
    fn production(&self, symbol: &S) -> Option<&Vec<S>>;

    /// Get the interpreter command for the given symbol, or None if no matching
    /// command is found.
    fn interpretation(&self, symbol: &S) -> Option<&TurtleCommand>;

    /// Get the expansion size of the worse-case production.
    fn biggest_expansion(&self) -> usize;
}

#[derive(Clone, Debug)]
struct SymbolRule<S: Eq>(Vec<S>, TurtleCommand);

/// HashMap-based Rules structure.
#[derive(Clone, Debug)]
pub struct HashMapRules<S>
    where S: Eq + Hash
{
    rules: HashMap<S, SymbolRule<S>>,
    biggest_expansion: usize,
}

impl<S> HashMapRules<S> where S: Eq + Hash
{
    pub fn new() -> HashMapRules<S> {
        HashMapRules {
            rules: HashMap::new(),
            biggest_expansion: 0,
        }
    }

    /// Add a new symbol rule or modify an existing one.
    /// Return true if an existing rule was modified, false otherwise.
    pub fn set(&mut self, symbol: S, production: Vec<S>, interpretation: TurtleCommand) -> bool {
        let production_len = production.len();
        let modified = match self.rules.insert(symbol, SymbolRule(production, interpretation)) {
            Some(_) => true,
            None => false,
        };
        if production_len > self.biggest_expansion {
            self.biggest_expansion = production_len;
        }
        modified
    }
}

impl HashMapRules<char> {
    /// Convenience method for calling 'set' directly with an str slice.
    /// NB: unicode char should be avoided at all cost, which is why ASCII is
    /// preferred.
    pub fn set_str(&mut self,
                   symbol: char,
                   production: &str,
                   interpretation: TurtleCommand)
                   -> bool {
        self.set(symbol, production.chars().collect(), interpretation)
    }
}

impl<S> LRules<S> for HashMapRules<S> where S: Eq + Hash
{
    fn production(&self, symbol: &S) -> Option<&Vec<S>> {
        self.rules.get(symbol).map(|r| &r.0)
    }

    fn interpretation(&self, symbol: &S) -> Option<&TurtleCommand> {
        self.rules.get(symbol).map(|r| &r.1)
    }

    fn biggest_expansion(&self) -> usize {
        self.biggest_expansion
    }
}

/// Default, ASCII-only 'HashMapRules' type.
pub type AsciiHashMapRules = HashMapRules<u8>;

impl AsciiHashMapRules {
    pub fn set_ascii(&mut self,
                     symbol: u8,
                     production: &[u8],
                     interpretation: TurtleCommand)
                     -> bool {
        self.set(symbol, production.to_vec(), interpretation)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use interpret::TurtleCommand;

    #[test]
    fn hashmap_rules_char() {
        let mut rules: HashMapRules<char> = HashMapRules::new();
        rules.set_str('A', "+B−A−B+", TurtleCommand::AdvanceBy(10f32));
        rules.set_str('B', "−A+B+A−", TurtleCommand::AdvanceBy(10f32));
        rules.set_str('+', "+", TurtleCommand::RotateBy(60f32));
        rules.set_str('-', "-", TurtleCommand::RotateBy(-60f32));

        assert_eq!(rules.biggest_expansion(), 7);
        assert_eq!(rules.production(&'A'),
                   Some(&"+B−A−B+".chars().collect()));
        assert_eq!(rules.production(&'B'),
                   Some(&"−A+B+A−".chars().collect()));
        assert_eq!(rules.production(&'C'), None);
    }

    #[test]
    fn hashmap_rules_ascii() {
        let mut rules = AsciiHashMapRules::new();
        rules.set_ascii(b'A', b"AB", TurtleCommand::None);
        rules.set_ascii(b'B', b"A", TurtleCommand::None);

        assert_eq!(rules.biggest_expansion(), 2);
        assert_eq!(rules.production(&b'A'), Some(&b"AB".to_vec()));
        assert_eq!(rules.production(&b'B'), Some(&b"A".to_vec()));
        assert_eq!(rules.production(&b'C'), None);
    }
}
