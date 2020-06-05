#![feature(core_intrinsics)] // intrinsic division requires nightly
#![no_std]
#![no_main]

use klee_sys::*;
extern crate cortex_m;
extern crate panic_klee;

use cortex_m::peripheral::Peripherals;

use core::{intrinsics::unchecked_div, num::Wrapping, ptr::read_volatile};

#[no_mangle]
fn main() {
    let peripherals = Peripherals::take().unwrap();

    let mut dwt = peripherals.DWT;
    dwt.enable_cycle_counter();
    let a = dwt.cyccnt.read();
    let b = dwt.cyccnt.read();
    let c = dwt.cyccnt.read();
    // let d = (Wrapping(c) - (Wrapping(b) - Wrapping(100))).0;
    // klee_assume(d != 0);
    unsafe {
        let some_time_quota = unchecked_div(a, (Wrapping(c) - (Wrapping(b) - Wrapping(100))).0);
        read_volatile(&some_time_quota); // prevent optimization in release mode
    }
}

// Notice this example currently requires the nightly build of Rust.
// > rustup override set nightly
// When you are done with this example, you may return to stable Rust.
// > rust override unset
//
// > cargo klee --example cortex_m_test_nightly -r -k -g -v
// ...
// KLEE: WARNING: undefined reference to function: rust_eh_personality
// KLEE: ERROR: examples/cortex_m_test_nightly.rs:23: divide by zero
// KLEE: NOTE: now ignoring this error at this location
//
// KLEE: done: total instructions = 1446
// KLEE: done: completed paths = 4
// KLEE: done: generated tests = 3
// ..
//(gdb) shell ls klee-last
// assembly.ll  info  messages.txt  run.istats  run.stats  test000001.div.err  test000001.kquery  test000001.ktest  test000002.ktest  test000003.ktest  warnings.txt
//
// So we see that test000001.ktest was causing a division error,
// the other test case passed
//
// (gdb) set env KTEST_FILE=klee-last/test000001.ktest
// (gdb) run
// Starting program: /home/pln/rust/trustit/klee-examples/target/debug/examples/cortex_m_test_nightly.replay
// Program received signal SIGFPE, Arithmetic exception.
// 0x0000555555555525 in main () at examples/cortex_m_test_nightly.rs:23
// 23              let some_time_quota = unchecked_div(a, (Wrapping(c) - (Wrapping(b) - Wrapping(100))).0);
//
// Let's look at the actual test
// (gdb) shell ktest-tool klee-last/test000001.ktest
// ktest file : 'klee-last/test000001.ktest'
// args       : ['/home/pln/rust/trustit/klee-examples/target/debug/examples/cortex_m_test_nightly-dd58a25289c18430.ll']
// num objects: 5
// object 0: name: 'PRIMASK'
// object 0: size: 4
// object 0: data: b'\x01\x01\x01\x01'
// object 0: hex : 0x01010101
// object 0: int : 16843009
// object 0: uint: 16843009
// object 0: text: ....
// object 1: name: 'vcell'
// object 1: size: 4
// object 1: data: b'\x00\x00\x00\x00'
// object 1: hex : 0x00000000
// object 1: int : 0
// object 1: uint: 0
// object 1: text: ....
// object 2: name: 'vcell'
// object 2: size: 4
// object 2: data: b'\x00\x00\x00\x00'
// object 2: hex : 0x00000000
// object 2: int : 0
// object 2: uint: 0
// object 2: text: ....
// object 3: name: 'vcell'
// object 3: size: 4
// object 3: data: b'd\x00\x00\x00'
// object 3: hex : 0x64000000
// object 3: int : 100
// object 3: uint: 100
// object 3: text: d...
// object 4: name: 'vcell'
// object 4: size: 4
// object 4: data: b'\x00\x00\x00\x00'
// object 4: hex : 0x00000000
// object 4: int : 0
// object 4: uint: 0
// object 4: text: ....
//
// (gdb)  backtrace
// #0  0x0000555555555525 in main () at examples/cortex_m_test_nightly.rs:23
// (gdb) print a
// $1 = 0
// (gdb) print b
// $2 = 100
// (gdb) print c
// $3 = 0
//
// In order to analyze hardware dependent code, hardware access are treated
// as a new symbolic value. In `cortex-m` we give symbolic names to core peripherals.
// The svd2rust generated PAC is currently given the symbolic name `vcell`. This
// might in the future change to giving the address to the register instead.
//
// A breakdown of the example:
// Behind the scenes the PRIMASK register is accessed, and given concrete value.
// (Under the hood, `Peripherals.take` executes in a global critical section.)
//
// This access is along the "happy path" towards the error, so any value would
// suffice (in this case 0x01010101 was selected by KLEE).
//
// The first `vcell` access: was done when enabling the cycle counter.
// The rest of accesses stem from reading `a`, `b`, and `c`.
// Critical here is that KLEE spots that `(c - (b - 100)) = 0`, leading up to a division
// by zero error (satisfied by `c == 0` and `b == 100`)
//
// Notice here, that this error is spotted EVEN while we are telling
// Rust to use the primitive (intrinsic) division for "unchecked_div" performance.
//
// Now re-run the example in --release mode.
// You should find that the error is spotted but the variables are in registers,
// so `print` won't work.
//
// Discussion:
// We can allow for AGGRESSIVE optimization by proving the absence of errors.
// In this case we use the Wrapping for unchecked wrapping arithmetics (along the lines of C/C++)
// and primitive unchecked (intrinsic) division.
//
// Checked arithmetics comes with a high prise at run-time, and for embedded not
// only affects the execution time but also power consumption.
//
// We can fearlessly apply optimisations (including intrinsic/primitive operations)
// and let the tool prove that the code is free of potential errors.
//
// Try uncommenting lines 22 and 23. This introduces a sufficient assumption
// for KLEE to prove the absence of errors. (Beware that this guarantee
// holds only for the exact source code given. If you change anything
// the analysis needs to be re-done.)
//
// In conclusion, programs proven free of errors offer both
// - improved performance (allow safe use of intrinsics), and
// - improved reliability/correctness
// at the same time.
// This is the way!
