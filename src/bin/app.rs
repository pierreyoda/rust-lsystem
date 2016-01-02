use std::cell::RefCell;
use std::sync::mpsc::{Sender, Receiver};

use time;

use rustlsystem::*;
use rustlsystem::view::start_worker;
use rustlsystem::rules::HashMapRules;
use rustlsystem::interpret::TurtleCommand::*;

/// The application for viewing L-Systems.
pub struct Application;

impl Application {
    pub fn run() {
        use rustlsystem::view::MessageToViewer::*;
        use rustlsystem::view::MessageFromViewer::*;

        // L-System definition
        let lsystem_axiom: Vec<char> = "A".chars().collect();
        let mut lsystem_rules = HashMapRules::new();
        lsystem_rules.set_str('A', "+B-A-B+", AdvanceBy(10f32));
        lsystem_rules.set_str('B', "−A+B+A−", AdvanceBy(15f32));
        lsystem_rules.set_str('+', "+", RotateBy(60f32));
        lsystem_rules.set_str('-', "-", RotateBy(-60f32));

        // worker test
        let processor: Box<process::LProcessor<char> + Send> =
            Box::new(process::ChunksProcessor::new(4, 100_000).ok().unwrap());
        let interpreter: Box<interpret::LInterpreter<char> + Send> =
            Box::new(interpret::SimpleInterpreter);
        let (tx, rx) = view::start_worker(RefCell::new(processor), RefCell::new(interpreter));

        Self::command_and_wait(&tx, &rx, Iterate, true);
        Self::command_and_wait(&tx,
                               &rx,
                               LoadLSystem(lsystem_axiom, Box::new(lsystem_rules)),
                               true);
        for _ in 0..16 {
            Self::command_and_wait(&tx, &rx, Iterate, true);
        }
        Self::command_and_wait(&tx, &rx, Terminate, true);
    }

    /// Send a command to the worker thread and return the associated response or error.
    /// NB : asynchronous waiting should be preferred for long operations (e.g. iteration).
    fn command_and_wait<S: Clone + Eq>(tx: &Sender<view::MessageFromViewer<S>>,
                                       rx: &Receiver<view::MessageToViewer>,
                                       msg: view::MessageFromViewer<S>,
                                       print_time: bool)
                                       -> view::MessageToViewer {
        let error_type = view::MessageToViewer::Error(String::new());
        let response_type = view::MessageToViewer::from_command(&msg);

        let t_start = time::now();
        tx.send(msg).unwrap();

        loop {
            match rx.recv() {
                Ok(response) => {
                    println!("worker thread sent : {:?}", response); // test
                    if response.same_type(&response_type) || response.same_type(&error_type) {
                        if print_time {
                            println!("> response received after {}s", time::now() - t_start);
                        }
                        return response;
                    }
                }
                _ => (),
            }
        }
    }
}
