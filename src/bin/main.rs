mod app;

extern crate time;

extern crate rustlsystem;
use rustlsystem::rules::HashMapRules;
use rustlsystem::state::{LSystem, new_rules_value};
use rustlsystem::process::{LProcessor, SimpleProcessor, ChunksProcessor};
use rustlsystem::interpret::TurtleCommand;

fn main() {
    let mut lsystem_rules = HashMapRules::new();
    lsystem_rules.set_str('A', "AB", TurtleCommand::None);
    lsystem_rules.set_str('B', "A", TurtleCommand::None);
    let mut lsystem = LSystem::new_with_char("A", new_rules_value(lsystem_rules));
    println!("Iteration {} : {:?}", lsystem.iteration(), lsystem.state());

    let mut processor: Box<LProcessor<char>> = if false {
        println!("=== SIMPLE PROCESSOR ===");
        Box::new(SimpleProcessor)
    } else {
        println!("=== CHUNKS PROCESSOR ===");
        Box::new(ChunksProcessor::new(4, 50_000).ok().unwrap())
    };

    let t_start = time::now();
    for _ in 0..35 {
        // WARNING : better use --release with iterations > 35
        lsystem = match processor.iterate(&lsystem) {
            Ok(r) => r,
            Err(why) => panic!(why),
        };
        println!("Iteration {} : len = {}",
                 lsystem.iteration(),
                 lsystem.state().len());
    }
    let t_end = time::now();

    // RESULTS :
    // for 35 iterations
    // chunk processing with 4 threads and 50_000 symbols per chunk
    // on my laptop : debug | release
    //    simple    :  47s  | 1086 ms
    //    chunks    :  26s  | 947 ms

    println!("Duration : {}s", t_end - t_start);
}
