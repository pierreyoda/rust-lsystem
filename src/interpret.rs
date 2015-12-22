use super::state::LSystem;

/// Enumerates all the commands needed for a Turtle-type rendering of an
/// L-System's state.
/// Only 2D is supported for now.
#[derive(Clone, Debug, PartialEq)]
pub enum TurtleCommand {
    /// Advance the turtle by a certain amount (forward if this amount is
    /// positive, backward otherwise), in pixels by default.
    AdvanceBy(f32),
    /// Rotate the turtle by a certain angle, in degrees by default.
    RotateBy(f32),
    /// Push (save) the current turtle state (position and angle) to the stack.
    PushState,
    /// Pop (restore) the last turtle state from the stack.
    PopState,
    /// Do nothing command (useful for text-only tests).
    None,
}

unsafe impl Send for TurtleCommand {}

/// L-System interpreters translate the state of an L-System into a sequence
/// of drawing instructions in order to represent it (think Turtle graphics
/// from Logo).
pub trait LInterpreter<S: Clone+Eq> {
    fn interpret(&mut self, lsystem: &LSystem<S>) -> Result<Vec<TurtleCommand>, String>;
}

/// Simple, linear L-System interpreter.
/// NB: can rapidly freeze its container thread.
pub struct SimpleInterpreter;

impl<S> LInterpreter<S> for SimpleInterpreter where S: Clone + Eq
{
    fn interpret(&mut self, lsystem: &LSystem<S>) -> Result<Vec<TurtleCommand>, String> {
        let rules = lsystem.rules();
        let mut commands = Vec::with_capacity(lsystem.state().len());

        for s in lsystem.state() {
            match rules.interpretation(&s) {
                Some(command) => {
                    match *command {
                        TurtleCommand::None => (), // save memory
                        _ => commands.push(command.clone()),
                    }
                }
                None => (),
            }
        }
        commands.shrink_to_fit();

        Ok(commands)
    }
}

#[cfg(test)]
mod test {
    use rules::HashMapRules;
    use state::{LSystem, new_rules_value};
    use process::{LProcessor, SimpleProcessor};
    use super::{LInterpreter, SimpleInterpreter};
    use super::TurtleCommand::*;

    #[test]
    fn simple_interpreter() {
        let mut rules: HashMapRules<char> = HashMapRules::new();
        rules.set_str('A', "+B-A-B+", AdvanceBy(10f32));
        rules.set_str('B', "−A+B+A−", AdvanceBy(15f32));
        rules.set_str('+', "+", RotateBy(60f32));
        rules.set_str('-', "-", RotateBy(-60f32));
        let mut lsystem = LSystem::new_with_char("A", new_rules_value(rules));

        let at_iteration = 1;
        let expected_commands = [RotateBy(60.0),
                                 AdvanceBy(15.0),
                                 RotateBy(-60.0),
                                 AdvanceBy(10.0),
                                 RotateBy(-60.0),
                                 AdvanceBy(15.0),
                                 RotateBy(60.0)];

        for _ in 0..at_iteration {
            lsystem = SimpleProcessor.iterate(&lsystem).ok().unwrap();
        }
        assert_eq!(lsystem.iteration(), at_iteration);
        let commands = SimpleInterpreter.interpret(&lsystem).ok().unwrap();
        assert_eq!(commands.len(), expected_commands.len());
        for i in 0..commands.len() {
            assert_eq!(commands[i], expected_commands[i]);
        }
    }
}
