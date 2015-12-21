use super::rules::LRules;

/// Structure containing all that is needed to fully describe the current state
/// of an L-System.
pub struct LSystem<'a, S>
    where S: Eq
{
    /// The current iteration/generation, i.e. the number of processings/evolutions
    /// that were required to reach the current state.
    iteration: u64,
    /// The current internal state of the L-System, stored as a list of symbols.
    state: Vec<S>,
    /// The L-System's production rules.
    /// TODO: better structure ? but : generic is cumbersome + ?Sized problems
    rules: Box<LRules<S> + 'a>,
}

impl<'a, S> LSystem<'a, S> where S: Eq
{
    /// Create a new L-System with the given axiom (initial state, or seed)
    /// and production rules.
    pub fn new(axiom: Vec<S>, rules: Box<LRules<S> + 'a>) -> LSystem<S> {
        LSystem {
            rules: rules,
            iteration: 0,
            state: axiom,
        }
    }

    /// Get the current iteration/generation.
    pub fn iteration(&self) -> u64 {
        self.iteration
    }

    /// Get a view into the current state.
    pub fn state(&self) -> &[S] {
        &self.state[..]
    }

    /// Get the production rules.
    pub fn rules(&self) -> &LRules<S> {
        &*self.rules
    }
}

pub type AsciiLSystem<'a> = LSystem<'a, u8>;

impl<'a> LSystem<'a, char> {
    /// Easily create a new, char-based L-System.
    pub fn new_with_char(axiom: &str, rules: Box<LRules<char>>) -> Self {
        LSystem::new(axiom.chars().collect(), rules)
    }
}
