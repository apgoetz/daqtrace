use daqtrace::trace;
mod common;


// trace macro requires two parameters
fn main() {
    let sink = common::DummySink;
    trace!(sink);
}

