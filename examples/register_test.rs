#![no_std]
#![no_main]

use panic_klee as _;
use volatile_register::RW;
#[no_mangle]
fn main() {
    // we emulate a read/write hardware register (rw)
    let rw: RW<u32> = unsafe { core::mem::MaybeUninit::uninit().assume_init() };

    // reading will render a symbolic value of type u32
    let read_1 = rw.read();
    if read_1 == 1 {
        // we emulate a write to the hardware register
        unsafe { rw.write(0) };
        // this read is still treated as a new symbolic value of type u32
        let read_2 = rw.read();
        if read_2 == 2 {
            // will generate a panic!() if reached.
            unreachable!();
        }
    }
}

// showcase volatile register
// This is the underlying abstraction to all register accesses using the
// embedded Rust ecosystem.
//
// When analyzed by KLEE, we make the return value symbolic, thus each access
// givs a new unique symbol. Even if we write a value to it, the next read
// will still be treated as a new symbol. That might be overly pessimistic
// but is a safe approximation for the worst case behavior of the hardware.
//
// > cargo klee --example register_test --release
// ...
// KLEE: ERROR: /home/pln/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-abort-0.3.2/src/lib.rs:49: abort failure
// KLEE: NOTE: now ignoring this error at this location

// KLEE: done: total instructions = 33
// KLEE: done: completed paths = 3
// KLEE: done: generated tests = 3
//
// > ls target/release/examples/klee-last/
// assembly.ll  info  messages.txt  run.istats  run.stats  test000001.ktest  test000002.ktest  test000003.abort.err  test000003.kquery  test000003.ktest  warnings.txt
//
// We see that KLEE spotted the test000003 hits unreachable (and thus panics)
//
// Let's look at the test cases separately:
//
// test1 passed:
// > ktest-tool target/release/examples/klee-last/test000001.ktest
// ktest file : 'target/release/examples/klee-last/test000001.ktest'
// args       : ['...']
// num objects: 1
// object 0: name: 'vcell'
// object 0: size: 4
// object 0: data: b'\x00\x00\x00\x00'
// object 0: hex : 0x00000000
// object 0: int : 0
// object 0: uint: 0
// object 0: text: ....
//
// If the first read of the register is not 1 then we are ok (program ends).
//
// The second test also passed.
// ktest-tool target/release/examples/klee-last/test000002.ktest
// ktest file : 'target/release/examples/klee-last/test000002.ktest'
// args       : ['...']
// num objects: 2
// object 0: name: 'vcell'
// object 0: size: 4
// object 0: data: b'\x01\x00\x00\x00'
// object 0: hex : 0x01000000
// object 0: int : 1
// object 0: uint: 1
// object 0: text: ....
// object 1: name: 'vcell'
// object 1: size: 4
// object 1: data: b'\x00\x00\x00\x00'
// object 1: hex : 0x00000000
// object 1: int : 0
// object 1: uint: 0
// object 1: text: ....
// Here we are saved by the second reading of the register NOT giving
// returning 2.
//
// The third test gives the error.
// ktest-tool target/release/examples/klee-last/test000003.ktest
// ktest file : 'target/release/examples/klee-last/test000003.ktest'
// args       : ['...']
// num objects: 2
// object 0: name: 'vcell'
// object 0: size: 4
// object 0: data: b'\x01\x00\x00\x00'
// object 0: hex : 0x01000000
// object 0: int : 1
// object 0: uint: 1
// object 0: text: ....
// object 1: name: 'vcell'
// object 1: size: 4
// object 1: data: b'\x02\x00\x00\x00'
// object 1: hex : 0x02000000
// object 1: int : 2
// object 1: uint: 2
// object 1: text: ....
//
// The first read gives 1, the second 2, and we hit unreacable.
//
// We can replay the last test to gather more information:
// $ cargo klee --example register_test -r -k -g -v
// ...
// Reading symbols from register.replay...done.
//
// (gdb) set env KTEST_FILE=klee-last/test000001.ktest
// (gdb) run # for the generated test the program will run to end
// Starting program: /home/pln/rust/trustit/klee-examples/target/debug/examples/register_test.replay
// [Inferior 1 (process 25074) exited with code ...]
//
// (gdb) set env KTEST_FILE=klee-last/test000003.ktest
// (gdb) run # for the generated test the program will panic due to unreachable.
// // Starting program: /home/pln/rust/trustit/klee-examples/target/debug/examples/register_test.replay
// Program received signal SIGABRT, Aborted.
// 0x00007ffff7dd3f25 in raise () from /usr/lib/libc.so.6
// (gdb) backtrace
// #0  0x00007ffff7dd3f25 in raise () from /usr/lib/libc.so.6
// #1  0x00007ffff7dbd897 in abort () from /usr/lib/libc.so.6
// #2  0x00005555555553db in rust_begin_unwind (_info=0x7fffffffd268) at /home/pln/.cargo/git/checkouts/panic-klee-aa8d015442188497/3b0c897/src/lib.rs:8
// #3  0x000055555555533d in core::panicking::panic_fmt () at src/libcore/panicking.rs:139
// #4  0x00005555555553a9 in core::panicking::panic () at src/libcore/panicking.rs:70
// #5  0x0000555555555303 in main () at examples/register_test.rs:104
// (gdb) frame 5
// #5  0x0000555555555303 in main () at examples/register_test.rs:104
// 104                 unreachable!();
// (gdb) print read_1
// $1 = 1
// (gdb) print read_2
// $2 = 2
//
// If this does not work its a gdb problem, try lldb the LLVM debugger
// (debug info may not be completely compatible)
//
// This is the way!
