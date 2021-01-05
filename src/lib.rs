#![no_std]

use serde::Serialize;
pub use daqtrace_macros::trace as trace;



pub trait Sink {
    fn timestamp(&self) -> u16 {
        return 0
    }
    fn write<S:Serialize>(&self, _id : u16, message : &S);
}


