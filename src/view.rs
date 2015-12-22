use state::LSystem;
use interpret::TurtleCommand;

/// Common commands from the viewer application to the worker thread.
enum MessageFromViewer<'a, S>
    where S: Clone + Eq
{
    LoadLSystem(LSystem<'a, S>),
    Iterate,
    Interpret,
}

/// Common commands from the core thread to the worker application.
enum MessageToViewer {
    LoadingFinished,
    IterationFinished,
    InterpretationFinished(Vec<TurtleCommand>),
    Error(String),
}

// / Worker running function, to be executed in its own thread.
