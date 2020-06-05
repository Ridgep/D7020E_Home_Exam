// example that shows different types of path termination
// and their effect to KLEE test case generation
#![no_std]
#![no_main]

use klee_sys::{klee_abort, klee_assert, klee_assert_eq, klee_make_symbolic};
use panic_klee as _;

#[no_mangle]
fn main() {
    let mut a: i32 = 0;
    klee_make_symbolic!(&mut a, "a");
    // Rust panic on a == 200 (div by zero), or a - 200 (causing an arithmetic overflow).
    let _ = 100 / a.wrapping_sub(200);

    let _ = match a {
        0 => klee_abort!(),
        1 => klee_abort!(),
        2 => klee_abort!(),
        3 => panic!("3"), // just one instance of panic! will be spotted
        4 => klee_assert!(false),
        5 => klee_assert_eq!(false, true),
        6 => klee_assert_eq!(false, true),
        _ => (),
    };
    panic!("at end"); // just one instance of panic! will be spotted
}

// KLEE will generate one test per explicit error:
// namely lines 17,18,19,21,22,23,26.
//
// Rust panics are also caught but detected at the `panic-klee` trampoline
// (and reported just once, either line 14, 20 or 26 we don't know.
//
// > cargo klee --example paths
// ...
// KLEE: ERROR: examples/paths.rs:22: abort failure
// KLEE: NOTE: now ignoring this error at this location
// KLEE: ERROR: examples/paths.rs:17: abort failure
// KLEE: NOTE: now ignoring this error at this location
// KLEE: ERROR: examples/paths.rs:19: abort failure
// KLEE: NOTE: now ignoring this error at this location
// KLEE: ERROR: examples/paths.rs:21: abort failure
// KLEE: NOTE: now ignoring this error at this location
// KLEE: ERROR: examples/paths.rs:23: abort failure
// KLEE: NOTE: now ignoring this error at this location
// KLEE: ERROR: examples/paths.rs:18: abort failure
// KLEE: NOTE: now ignoring this error at this location
// KLEE: ERROR: /home/pln/.cargo/git/checkouts/panic-klee-aa8d015442188497/3b0c897/src/lib.rs:8: abort failure
// KLEE: NOTE: now ignoring this error at this location
//
// KLEE: done: total instructions = 269
// KLEE: done: completed paths = 10
// KLEE: done: generated tests = 7
//
// (Notice here the order of generated tests is non-deterministic.)
//
// We can inspect the generated tests:
//
// > ls target/debug/examples/klee-last/
// assembly.ll   run.istats            test000001.kquery     test000002.kquery     test000003.kquery     test000004.kquery     test000005.kquery     test000006.kquery     test000007.kquery
// info          run.stats             test000001.ktest      test000002.ktest      test000003.ktest      test000004.ktest      test000005.ktest      test000006.ktest      test000007.ktest
// messages.txt  test000001.abort.err  test000002.abort.err  test000003.abort.err  test000004.abort.err  test000005.abort.err  test000006.abort.err  test000007.abort.err  warnings.txt
//
// Let's try to find out the path leading up to the panic. In this case the last test.
//
// > more target/debug/examples/klee-last/test000007.abort.err
// Error: abort failure
// File: /home/pln/.cargo/git/checkouts/panic-klee-aa8d015442188497/3b0c897/src/lib.rs
// Line: 8
// assembly.ll line: 33
// Stack:
//         #000000033 in rust_begin_unwind (=94250200401696) at /home/pln/.cargo/git/checkouts/panic-klee-aa8d015442188497/3b0c897/src/lib.rs:8
//         #100000241 in _ZN4core9panicking9panic_fmt17hdeb7979ab6591473E (=94250202388144, =94250199410592) at src/libcore/panicking.rs:139
//         #200000275 in _ZN4core9panicking5panic17hb5daa85c7c72fc62E (=94250198589360, =33, =94250199410592) at src/libcore/panicking.rs:70
//         #300000204 in main () at examples/paths.rs:14
//
// We see that it was origin from line 14.
//
// We can look at the actual test.
// > ktest-tool target/debug/examples/klee-last/test000007.ktest
// ktest file : 'target/debug/examples/klee-last/test000007.ktest'
// args       : ['/home/pln/rust/trustit/klee-examples/target/debug/examples/paths-cb1d2fb40e2b17af.ll']
// num objects: 1
// object 0: name: 'a'
// object 0: size: 4
// object 0: data: b'\x00\x00\x00\x80'
// object 0: hex : 0x00000080
// object 0: int : -2147483648
// object 0: uint: 2147483648
// object 0: text: ....
//
// In this case the problem was the arithmetic overflow.
// Change line 14 to
// let _ = 100 / a.wrapping_sub(200);
//
// Now re-run KLEE.
//
// > ktest-tool target/debug/examples/klee-last/test000007.ktest
// ktest file : 'target/debug/examples/klee-last/test000007.ktest'
// args       : ['/home/pln/rust/trustit/klee-examples/target/debug/examples/paths-cb1d2fb40e2b17af.ll']
// num objects: 1
// object 0: name: 'a'
// object 0: size: 4
// object 0: data: b'\xc8\x00\x00\x00'
// object 0: hex : 0xc8000000
// object 0: int : 200
// object 0: uint: 200
// object 0: text: ....
//
// In this case we see that the panic was triggered by the division by zero.
//
// This is the way!
