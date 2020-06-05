// showcase partially symbolic structures

#![no_std]
#![no_main]

use klee_sys::*;
use panic_klee as _;

struct A {
    a: u8,
    b: u32,
}

#[no_mangle]
fn main() {
    let mut a = 0;
    klee_make_symbolic!(&mut a, "a");
    let mut u = A { a: a, b: 0 };
    u.b = 7;
    let _ = f2(f1(u.a));
}

fn f1(u: u8) -> u8 {
    u.wrapping_add(1)
}

fn f2(u: u8) -> u8 {
    100 / u
}

// Often we may have structures with partially unknown data.
//
// Let us find out what valu of `u` that may cause a ponic by replay in gdb.
//
// > cargo klee --example struct -k -g -r
//
// KLEE: ERROR: /home/pln/.cargo/git/checkouts/panic-klee-aa8d015442188497/3b0c897/src/lib.rs:8: abort failure
// KLEE: NOTE: now ignoring this error at this location
//
// KLEE: done: total instructions = 175
// KLEE: done: completed paths = 2
// KLEE: done: generated tests = 2
// ...
// Reading symbols from struct.replay...
// (gdb) set env KTEST_FILE=klee-last/test000001.ktest
// (gdb) run
// Starting program: /home/pln/rust/trustit/klee-examples/target/debug/examples/struct.replay
// [Inferior 1 (process 114832) exited with code 0144]
// (gdb) set env KTEST_FILE=klee-last/test000002.ktest
// (gdb) run
// Starting program: /home/pln/rust/trustit/klee-examples/target/debug/examples/struct.replay

// Program received signal SIGABRT, Aborted.
// 0x00007ffff7dd3f25 in raise () from /usr/lib/libc.so.6
// (gdb) bt
// #0  0x00007ffff7dd3f25 in raise () from /usr/lib/libc.so.6
// #1  0x00007ffff7dbd897 in abort () from /usr/lib/libc.so.6
// #2  0x000055555555533b in rust_begin_unwind (_info=0x7fffffffd308) at /home/pln/.cargo/git/checkouts/panic-klee-aa8d015442188497/3b0c897/src/lib.rs:8
// #3  0x000055555555529d in core::panicking::panic_fmt () at src/libcore/panicking.rs:139
// #4  0x0000555555555309 in core::panicking::panic () at src/libcore/panicking.rs:70
// #5  0x000055555555526d in struct::f2 (u=0) at examples/struct.rs:28
// #6  0x000055555555520f in main () at examples/struct.rs:20
//
// So it's f2(u=0) that crashes.
// Let us take a look at frame 6, calling f2(f1(u.a)).
//
// (gdb) f 6
// #6  0x000055555555520f in main () at examples/struct.rs:20
// 20          let _ = f2(f1(u.a));
// (gdb) i locals
// u = struct::A {a: 255, b: 7}
// a = 255
//
// So in this cose u.a = 255, causes a wraparound, and that in turn causes the crash.
// Not obvious to the naked eye.
// This is the way!
