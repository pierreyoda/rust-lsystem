use std::sync::Arc;

use super::rules::LRules;

pub type RulesValue<'a, S> = Arc<Box<LRules<S> + 'a + Sync + Send>>;

pub fn new_rules_value<'a, S: Eq, R: 'a + LRules<S> + Sync + Send>(rules: R) -> RulesValue<'a, S> {
    Arc::new(Box::new(rules))
}


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
    rules: RulesValue<'a, S>,
}

impl<'a, S> LSystem<'a, S> where S: Eq
{
    /// Create a new L-System with the given axiom (initial state, or seed)
    /// and production rules.
    /// Optionally, one can specify the current iteration of the L-System.
    pub fn new(axiom: Vec<S>, rules: RulesValue<'a, S>, iteration: Option<u64>) -> LSystem<S> {
        LSystem {
            rules: rules,
            iteration: iteration.unwrap_or(0),
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
    pub fn rules(&self) -> &RulesValue<'a, S> {
        &self.rules
    }
}

pub type AsciiLSystem<'a> = LSystem<'a, u8>;

impl<'a> LSystem<'a, char> {
    /// Easily create a new, char-based L-System.
    pub fn new_with_char(axiom: &str, rules: RulesValue<'a, char>) -> Self {
        LSystem::new(axiom.chars().collect(), rules, None)
    }
}
