extern crate rustlsystem;
use rustlsystem::rules::HashMapRules;
use rustlsystem::state::{LSystem, new_rules_value};
use rustlsystem::process::{LProcessor, SimpleProcessor};

fn main() {
    let mut lsystem_rules = HashMapRules::new();
    lsystem_rules.set_str('A', "AB");
    lsystem_rules.set_str('B', "A");
    let mut lsystem = LSystem::new_with_char("A", new_rules_value(lsystem_rules));
    println!("Iteration {} : {:?}", lsystem.iteration(), lsystem.state());

    for _ in 0..5 {
        lsystem = match SimpleProcessor::iterate(&lsystem) {
            Ok(r) => r,
            Err(why) => panic!(why),
        };
        println!("Iteration {} : {:?}", lsystem.iteration(), lsystem.state());
    }
}
