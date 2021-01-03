# QEMU based examples for daqtrace

This crate contains example usecases of daqtrace based off of QEMU. It defines a a simple daqtrace Sink that outputs CBOR .

# EXAMPLES

To run the examples, you will need an arm QEMU.  An example can be seen below:

In order to run these examples, your cargo runner should be set to
something like: `qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb
-nographic -semihosting -serial mon:stdio -kernel `

Since these examples output CBOR, it is recommended to use a helper
program to parse the CBOR output in human readable format. [cbor-diag](https://crates.io/crates/cbor-diag) is recommended. 

_The classic example_

```
$ cargo run --bin hello | cbor-diag --seq
    Finished dev [optimized + debuginfo] target(s) in 0.01s
     Running `qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -semihosting -serial stdio -monitor none -kernel target/thumbv7m-none-eabi/debug/hello`
[0, 0, "world!"]

[1, 0, 1, 2, 3]

```
