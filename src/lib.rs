#![no_std]
use serde::Serialize;

#[macro_export]
macro_rules! trace {
    ($sink:expr, $id:ident, $($msg:expr),*) => {
        {
            trace!(stringify!($id));
            let id = &DAQTRACE_PLACEHOLDER as *const u8 as u16;
            let ts = $sink.timestamp();
            let msg = (id, ts, $($msg),*);
            $sink.write(id, &msg);
        }
    };
    ($quoted:expr) => {
        #[link_section = ".daqtrace"]
        #[export_name = $quoted]
        static DAQTRACE_PLACEHOLDER: u8 = 0;
    }
}

pub trait Sink {
    fn timestamp(&self) -> u16 {
        return 0
    }
    fn write<S:Serialize>(&mut self, _id : u16, message : &S);
}


