#![no_std]
#![no_main]

// pick a panicking behavior
use pl011_qemu;
use panic_halt as _; 
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug};
use serde_cbor::{ser::SliceWrite, Serializer};
use embedded_hal::serial;
use embedded_hal::serial::Write;
use nb::block;
use daqtrace::trace;
use daqtrace::{Sink};
use serde::Serialize;


//#[link_section = ".daqtrace"]
//#[export_name = "exported_symbol_name"]
// static __asdf__ : u16  = 1;


struct PL011Sink<U> {
    tx : pl011_qemu::Tx<U>
}

impl<U> PL011Sink<U> {
    fn new(tx : pl011_qemu::Tx<U>) -> Self {
        PL011Sink{tx}
    }
}

impl<U> daqtrace::Sink for PL011Sink<U>
    where
    pl011_qemu::Tx<U> : serial::Write<u8>,
{
    fn write<S:Serialize>(&mut self, _id : u16, message : &S) {
        let mut buffer = [0u8 ; 128];
        let writer = SliceWrite::new(&mut buffer);
        let mut ser = Serializer::new(writer).packed_format();
        message.serialize(&mut ser).unwrap();
        let num_written = ser.into_inner().bytes_written();
        for b in buffer[0..num_written].iter() {
            if block!(self.tx.write(*b)).is_err() {
                panic!();
            }
        }
    }
}

#[entry]
fn main() -> ! {

    let uart = pl011_qemu::PL011::new(pl011_qemu::UART1::take().unwrap());
    let (_tx,_) = uart.split();
    let mut sink = PL011Sink::new(_tx);

    
    trace!(sink, hello, "world!");
    trace!(sink, abc, 1,2,3);
    loop {
            debug::exit(debug::EXIT_SUCCESS);
    }    
}
