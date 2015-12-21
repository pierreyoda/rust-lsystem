use super::rules::LRules;
use super::state::LSystem;

pub trait LProcessor<S: Clone+Eq> {
    /// Try and iterate the given L-System into its next state according to
    /// its production rules.
    /// Return None if successful, Some(error_string) otherwise.
    fn iterate<'a>(lsystem: &LSystem<'a, S>) -> Result<LSystem<'a, S>, String>;
}

/// Simple thread-blocking L-System processor.
pub struct SimpleProcessor;

impl<S> LProcessor<S> for SimpleProcessor where S: Clone + Eq
{
    fn iterate<'a>(lsystem: &LSystem<'a, S>) -> Result<LSystem<'a, S>, String> {
        // allocate a new state with the worst possible size
        let rules = lsystem.rules().clone();
        let new_state_size = lsystem.state().len() * rules.biggest_expansion();
        if new_state_size > usize::max_value() {
            return Err(format!("SimpleProcessor : cannot allocate a Vec of size {}",
                               new_state_size));
        }
        let mut new_state: Vec<S> = Vec::with_capacity(new_state_size);

        // iterate over the symbols
        for s in lsystem.state() {
            match rules.production(&s) {
                Some(symbols) => new_state.extend(symbols.iter().cloned()),
                None => (),
            }
        }
        new_state.shrink_to_fit();
        Ok(LSystem::<S>::new(new_state, rules.clone(), Some(lsystem.iteration() + 1)))
    }
}

#[cfg(test)]
mod test {
    use rules::HashMapRules;
    use state::{LSystem, new_rules_value};
    use super::*;

    #[test]
    fn simple_processing() {
        let mut rules = HashMapRules::new(); // algae rules
        rules.set_str('A', "AB");
        rules.set_str('B', "A");
        let expected_states = ["A",
                               "AB",
                               "ABA",
                               "ABAAB",
                               "ABAABABA",
                               "ABAABABAABAAB",
                               "ABAABABAABAABABAABABA",
                               "ABAABABAABAABABAABABAABAABABAABAAB"];
        let mut lsystem = LSystem::new_with_char(&expected_states[0], new_rules_value(rules));

        for n in 0..expected_states.len() {
            assert_eq!(lsystem.iteration(), n as u64);
            let expected: Vec<char> = expected_states[n].chars().collect();
            assert_eq!(lsystem.state(), &expected[..]);
            lsystem = SimpleProcessor::iterate(&lsystem).ok().unwrap();
        }
    }
}
