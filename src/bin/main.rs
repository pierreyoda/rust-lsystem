extern crate rustlsystem;

use rustlsystem::rules::HashMapRules;
use rustlsystem::state::LSystem;

fn main() {
    let mut lsystem_rules = HashMapRules::new();
    lsystem_rules.set_str('A', "AB");
    lsystem_rules.set_str('B', "A");
    let lsystem = LSystem::new_with_char("A", Box::new(lsystem_rules));
    println!("Iteration {} : {:?}", lsystem.iteration(), lsystem.state());
}
