use daqtrace::Sink;
use serde::Serialize;

pub struct DummySink;
impl Sink for DummySink {
    fn write<S:Serialize>(&self, _id : u16, _message : &S) {
    }
}
