use daqtrace::trace;
use serde::Serialize;
use serde_json;
use std::io;
struct JSONStdOutSink;

impl daqtrace::Sink for JSONStdOutSink {
    fn write<S:Serialize>(&self, _id : u16, message : &S) {
        serde_json::to_writer(io::stdout(), message).unwrap();
    }
}

fn main() {
    let sink = JSONStdOutSink;
    let (a,b,c) = (1,2,3);
    trace!(sink, hello, a, b, c);
    trace!(sink, world, s: "world!");
}
