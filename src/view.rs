use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::cell::RefCell;

use rules::LRules;
use process::LProcessor;
use state::{LSystem, RulesValue};
use interpret::{LInterpreter, TurtleCommand};

/// Common messages from the viewer application to the worker thread.
pub enum MessageFromViewer<S>
    where S: Clone + Eq
{
    /// Create a new 'LSystem' from the given axiom and 'LRules', and wait for
    /// the 'LoadingFinished' message.
    /// If no L-System is loaded, the other commands will do nothing.
    LoadLSystem(Vec<S>, Box<LRules<S> + Send + Sync>),
    /// Reset the L-System to its inital state and wait for the 'ResettingFinised'
    /// confirmation.
    ResetLSystem,
    /// Iterate the current L-System and wait for the 'IterationFinished' message.
    Iterate,
    /// Interpret the current L-System state and wait for the 'InterpretationFinished' result.
    Interpret,
    /// Terminate the worker thread.
    Terminate,
}

/// Common messages from the core thread to the worker application.
#[derive(Clone, Debug, PartialEq)]
pub enum MessageToViewer {
    LoadingFinished,
    ResettingFinised,
    /// Confirm that the current 'LSystem' was succesfully evolved to its n-th state.
    IterationFinished(u64),
    InterpretationFinished(Vec<TurtleCommand>),
    /// Confirm the worker thread termination then end the thread.
    Terminated,
    Error(String),
}

impl MessageToViewer {
    /// Return the type of 'MessageToViewer' expected from the associated command,
    /// ignoring any possible errors.
    /// If the result carries meaningful data, load it with dummy values.
    pub fn from_command<S: Clone + Eq>(msg: &MessageFromViewer<S>) -> MessageToViewer {
        use self::MessageToViewer::*;
        use self::MessageFromViewer::*;
        match *msg {
            LoadLSystem(_, _) => LoadingFinished,
            ResetLSystem => ResettingFinised,
            Iterate => IterationFinished(0),
            Interpret => InterpretationFinished(Vec::new()),
            Terminate => Terminated,
        }
    }

    /// Return true if the given argument is of the same 'MessageToViewer' type,
    /// with no considerations of any eventual carried data (see 'PartialEq' for
    /// equality check).
    pub fn same_type(&self, other: &MessageToViewer) -> bool {
        use self::MessageToViewer::*;
        match (self, other) {
            (&LoadingFinished, &LoadingFinished) => true,
            (&ResettingFinised, &ResettingFinised) => true,
            (&IterationFinished(_), &IterationFinished(_)) => true,
            (&InterpretationFinished(_), &InterpretationFinished(_)) => true,
            (&Terminated, &Terminated) => true,
            (&Error(_), &Error(_)) => true,
            _ => false,
        }
    }
}

/// Start the worker thread responsible for evolving and/or interpreting L-Systems,
/// and return the needed communication channels.
/// These allow, in association with a front-end GUI / CLI, to offer a non-blocking
/// L-System viewer application.
pub fn start_worker<S: 'static + Clone + Eq + Send>
    (processor: RefCell<Box<LProcessor<S> + Send>>,
     interpreter: RefCell<Box<LInterpreter<S> + Send>>)
     -> (Sender<MessageFromViewer<S>>, Receiver<MessageToViewer>) {
    let (tx, rx_ui) = channel::<MessageToViewer>();
    let (tx_ui, rx) = channel::<MessageFromViewer<S>>();

    thread::spawn(move || { worker_loop(tx, rx, processor, interpreter); });

    (tx_ui, rx_ui)
}

/// Worker running function, to be executed in its own thread.
fn worker_loop<S: Clone + Eq>(tx: Sender<MessageToViewer>,
                              rx: Receiver<MessageFromViewer<S>>,
                              processor: RefCell<Box<LProcessor<S> + Send>>,
                              interpreter: RefCell<Box<LInterpreter<S> + Send>>) {
    use self::MessageFromViewer::*;
    use self::MessageToViewer::*;

    let sleep_time = Duration::from_millis(25);

    let mut axiom: Option<Vec<S>> = None;
    let mut rules: Option<RulesValue<S>> = None;
    let mut lsystem: Option<LSystem<S>> = None;

    'main: loop {
        match rx.try_recv() {
            Ok(message_from_ui) => {
                match message_from_ui {
                    LoadLSystem(new_axiom, new_rules) => {
                        axiom = Some(new_axiom);
                        rules = Some(Arc::new(new_rules));
                        lsystem = Some(LSystem::<S>::new(axiom.clone().unwrap(),
                                                         rules.clone().unwrap(),
                                                         None));
                        tx.send(LoadingFinished).unwrap();

                    }
                    ResetLSystem if lsystem.is_some() => {
                        lsystem = Some(LSystem::<S>::new(axiom.clone().unwrap(),
                                                         rules.clone().unwrap(),
                                                         None));
                        tx.send(ResettingFinised).unwrap();
                    }
                    Iterate if lsystem.is_some() => {
                        lsystem = match processor.borrow_mut().iterate(lsystem.as_ref().unwrap()) {
                            Ok(v) => {
                                tx.send(IterationFinished(v.iteration())).unwrap();
                                Some(v)
                            }
                            Err(why) => {
                                tx.send(Error(why)).unwrap();
                                None
                            }
                        };
                        println!("> state len = {:?}\n",
                                 lsystem.as_ref().unwrap().state().len());
                    }
                    Interpret if lsystem.is_some() => {
                        match interpreter
                                  .borrow_mut()
                                  .interpret(lsystem.as_ref().unwrap()) {
                            Ok(v) => tx.send(InterpretationFinished(v)).unwrap(),
                            Err(why) => tx.send(Error(why)).unwrap(),
                        }
                    }
                    Terminate => {
                        tx.send(Terminated).unwrap();
                        break 'main;
                    }
                    _ => tx.send(Error(format!("no LSystem loaded"))).unwrap(),
                }
            }
            _ => (),
        }
        // avoid over-charging the CPU thread
        // when waiting for a command
        thread::sleep(sleep_time);
    }
}
