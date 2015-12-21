use super::rules::LRules;
use super::state::LSystem;

/// L-System processors are responsible for taking a L-System and evolving it to
// its next state.
pub trait LProcessor<S: Clone+Eq> {
    /// Try and iterate the given L-System into its next state according to
    /// its production rules.
    /// Return None if successful, Some(error_string) otherwise.
    fn iterate<'a>(&mut self, lsystem: &LSystem<'a, S>) -> Result<LSystem<'a, S>, String>;
}

/// Simple thread-blocking L-System processor.
pub struct SimpleProcessor;

impl<S> LProcessor<S> for SimpleProcessor where S: Clone + Eq
{
    fn iterate<'a>(&mut self, lsystem: &LSystem<'a, S>) -> Result<LSystem<'a, S>, String> {
        // allocate a new state with the worst possible size
        // (may cause overflow one or more iteration(s) earlier with huge states/production rules)
        let rules = lsystem.rules().clone();
        let new_state_size = match lsystem.state().len().checked_mul(rules.biggest_expansion()) {
            Some(v) => v,
            None => {
                return Err(format!("SimpleProcessor : usize overflow when computing new Vec size"))
            }
        };
        let mut new_state: Vec<S> = Vec::with_capacity(new_state_size);

        // iterate over the symbols
        for s in lsystem.state() {
            match rules.production(&s) {
                Some(symbols) => new_state.extend(symbols.iter().cloned()),
                None => new_state.push(s.clone()),
            }
        }
        new_state.shrink_to_fit();

        // return the evolved L-System
        Ok(LSystem::<S>::new(new_state, rules.clone(), Some(lsystem.iteration() + 1)))
    }
}

#[cfg(test)]
mod test {
    use rules::HashMapRules;
    use state::{LSystem, new_rules_value};
    use interpret::TurtleCommand;
    use super::*;

    #[test]
    fn simple_processing() {
        let mut rules = HashMapRules::new(); // algae rules
        rules.set_str('A', "AB", TurtleCommand::None);
        rules.set_str('B', "A", TurtleCommand::None);
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
            lsystem = SimpleProcessor.iterate(&lsystem).ok().unwrap();
        }
    }
}
