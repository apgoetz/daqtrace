use daqtrace::trace;

fn main() {
    let sink = 1;
    let (a,b,c) = (1,2,3);
    trace!(sink, hello, a, b, c);
}

