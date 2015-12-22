mod app;

extern crate rustlsystem;
use rustlsystem::rules::HashMapRules;
use rustlsystem::state::{LSystem, new_rules_value};
use rustlsystem::process::{LProcessor, SimpleProcessor};
use rustlsystem::interpret::TurtleCommand;

fn main() {
    let mut lsystem_rules = HashMapRules::new();
    lsystem_rules.set_str('A', "AB", TurtleCommand::None);
    lsystem_rules.set_str('B', "A", TurtleCommand::None);
    let mut lsystem = LSystem::new_with_char("A", new_rules_value(lsystem_rules));
    println!("Iteration {} : {:?}", lsystem.iteration(), lsystem.state());

    let mut processor = SimpleProcessor;
    for _ in 0..30 {
        lsystem = match processor.iterate(&lsystem) {
            Ok(r) => r,
            Err(why) => panic!(why),
        };
        println!("{}", lsystem.state().len());
        // println!("Iteration {} : {:?}", lsystem.iteration(), lsystem.state());
    }
}
