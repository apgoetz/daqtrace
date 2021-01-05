use daqtrace::trace;
mod common;


// all trace args must be tagged
fn main() {
    let sink = common::DummySink;
    trace!(sink,hello,123);
}

