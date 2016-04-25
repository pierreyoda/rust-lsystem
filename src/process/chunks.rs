use std::sync::{Arc, Mutex};
use simple_parallel;

use state::LSystem;
use super::{LProcessor, SimpleProcessor};

/// Parallel processor dividing a state into chunks to be individually iterated
/// within a pool of threads.
pub struct ChunksProcessor {
    /// The number of symbols per full chunk.
    chunk_size: usize,
    /// The thread pool.
    pool: simple_parallel::Pool,
}

impl ChunksProcessor {
    /// Try and create a new 'ChunksProcessor' instance with the given parameters.
    /// Typical values:
    /// - max_tasks : number of CPU logical cores
    /// - chunks_size : between 100_000 and 1_000_000 symbols per chunk
    pub fn new(max_tasks: usize, chunks_size: usize) -> Result<ChunksProcessor, String> {
        if max_tasks == 0 {
            Err(format!("ChunksProcessor::new : invalid maximum tasks number ({})",
                        max_tasks))
        } else if chunks_size == 0 {
            Err(format!("ChunksProcessor::new : invalid chunks size ({})",
                        chunks_size))
        } else {
            Ok(ChunksProcessor {
                chunk_size: chunks_size,
                pool: simple_parallel::Pool::new(max_tasks),
            })
        }
    }
}

impl<S> LProcessor<S> for ChunksProcessor
    where S: Clone + Eq + Send + Sync
{
    // TODO : better error handling...
    fn iterate<'a>(&mut self, lsystem: &LSystem<'a, S>) -> Result<LSystem<'a, S>, String> {
        // Set-up
        let mut vec: Vec<Vec<S>> = Vec::new();
        let state_len = lsystem.state().len();
        if state_len == 0 {
            return Err(format!("cannot iterate an empty state"));
        }
        let rem = state_len % self.chunk_size;
        let chunks_number = state_len / self.chunk_size +
                            match rem {
            0 => 0,
            _ => 1,
        };
        for _ in 0..chunks_number {
            vec.push(Vec::new());
        }
        let sub_states = Arc::new(Mutex::new(vec));

        // Chunks processing
        let rules = lsystem.rules().clone();
        let errors = Mutex::new(String::new());
        let chunks_iter = lsystem.state().chunks(self.chunk_size);
        self.pool.for_(chunks_iter.enumerate(), |(n, chunk)| {
            let result: Vec<S> = match SimpleProcessor::iterate_slice(chunk, &rules) {
                Ok(v) => v,
                Err(why) => {
                    let mut error_lock = errors.lock().unwrap();
                    *error_lock = format!("{}\n{}", *error_lock, why);
                    Vec::new()
                }
            };
            let mut chunk_data = sub_states.lock().unwrap();
            chunk_data[n] = result;
        });

        // Error handling
        let error_lock = errors.lock().unwrap();
        if !error_lock.is_empty() {
            return Err(format!("ChunksProcessor : iteration error(s):\n{}", *error_lock));
        }

        // Final assembling
        let mut new_state_size = 0usize;
        let mut new_state: Vec<S> = Vec::new();
        let data = sub_states.lock().unwrap();
        for n in 0..chunks_number {
            let chunk_iterated = &data[n];
            new_state_size = match new_state_size.checked_add(chunk_iterated.len()) {
                Some(v) => v,
                None => {
                    return Err(format!("ChunksProcessor::iterate : usize overflow, state too big \
                                        for for Vec"))
                }
            };
            new_state.extend(chunk_iterated.iter().cloned());
        }
        Ok(LSystem::<S>::new(new_state, rules, Some(lsystem.iteration() + 1)))
    }
}

#[cfg(test)]
mod test {
    use rules::HashMapRules;
    use state::{LSystem, new_rules_value};
    use interpret::TurtleCommand;
    use process::{LProcessor, ChunksProcessor};

    #[test]
    fn chunks_processing() {
        let mut rules = HashMapRules::new(); // algae rules
        rules.set_str('A', "AB", TurtleCommand::None);
        rules.set_str('B', "A", TurtleCommand::None);
        let expected_sizes = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597,
                              2584, 4181, 6765, 10946, 17711, 28657, 46368];
        let mut lsystem = LSystem::new_with_char("A", new_rules_value(rules));
        let mut processor = ChunksProcessor::new(4, 10_000).ok().unwrap();

        for n in 0..expected_sizes.len() {
            assert_eq!(lsystem.iteration(), n as u64);
            assert_eq!(lsystem.state().len(), expected_sizes[n]);
            lsystem = processor.iterate(&lsystem).ok().unwrap();
        }
    }
}
