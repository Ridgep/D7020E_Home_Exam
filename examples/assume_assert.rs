#![no_std]
#![no_main]

use core::u32;
use klee_sys::*;
use panic_klee as _;

#[no_mangle]
fn main() {
    let mut a: u32 = 0;
    klee_make_symbolic!(&mut a, "a");

    klee_assert!(f1(a) > a);
}

fn f1(a: u32) -> u32 {
    // klee_assume(a < u32::MAX);
    // klee_assume(a == u32::MAX);
    let r = a + 1;
    // klee_assert!(r > a);
    r
}

// This example showcase how contracts can be encoded
//
// Let us start with the function f1 (above).
// Intuitively `f1(a) > a` right?
//
// Well let's check that....
//
// > cargo klee --example assume_assert
// ...
// KLEE: ERROR: /home/pln/.cargo/git/checkouts/panic-klee-aa8d015442188497/3b0c897/src/lib.rs:8: abort failure
// KLEE: NOTE: now ignoring this error at this location
// ..
// KLEE: done: total instructions = 147
// KLEE: done: completed paths = 2
// KLEE: done: generated tests = 2
//
// So obviously that is not the case. What then?
// Here (again) we are exposed to an overflow on a + 1
//
// more target/debug/examples/klee-last/test000002.abort.err
// Error: abort failure
// File: /home/pln/.cargo/git/checkouts/panic-klee-aa8d015442188497/3b0c897/src/lib.rs
// Line: 8
// assembly.ll line: 23
// Stack:
//         #000000023 in rust_begin_unwind (=94422568770784) at /home/pln/.cargo/git/checkouts/panic-klee-aa8d015442188497/3b0c897/src/lib.rs:8
//         #100000193 in _ZN4core9panicking9panic_fmt17hdeb7979ab6591473E (=94422569870368, =94422566526016) at src/libcore/panicking.rs:139
//         #200000227 in _ZN4core9panicking5panic17hb5daa85c7c72fc62E (=94422566525664, =28, =94422566526016) at src/libcore/panicking.rs:70
//         #300000130 in _ZN13assume_assert2f117h371b5439de984e07E () at examples/assume_assert.rs:19
//        #400000088 in main () at examples/assume_assert.rs:13
//
// So its line 19, (a + 1)
//
// Lets now make a contract, assuming that a < u32.MAX
// uncomment `klee_assume!(a < core::u32::MAX)`
//
// > cargo klee --example assume_assert
//
// We can now finalize the contract, by un-commenting line 20 (with the post condition).
//
// So our pre-condition is that a < u32.MAX and the post condition is that r > a.
//
// Can assumptions go wrong?
// Well they can? And we will spot it!
//
// Try un-commenting line 18, re-run `cargo klee`
//
// You should get...
// KLEE: ERROR: /home/pln/.cargo/git/checkouts/klee-sys-7ee2aa8a1a6bbc46/c8275a3/src/lib.rs:19: invalid klee_assume call (provably false)
//
// So KLEE tracks the "path condition", i.e., at line 18 it knows (assumes) that that
// a < u32::MAX, and finds that the assumption a == u32::MAX cannot be satisfied.
//
// This is extremely powerful as KLEE tracks all known "constraints" and all their raliaitons
// and mathematically checks for the satisfiability of each "assume" and "assert".
// So what we get here is not a mere test, but an actual proof!!!!
// This is the way!
//
// In the future we could provide some additional syntactic support for
// contracts. It could look something like:
//
//#[contract: {
// pre: a < u32::MAX,
// post: f1(a) > a,
// } ]
//fn f1(a: u32) -> u32 {
//     a + 1
//}
//
// The corresponding generated code could look something like:
// fn f1(a: u32) -> u32 {
//     klee_assume(a < u32::MAX); // pre
//     let r = f1_body(a);
//     klee_assert!(r > a); // post
//     r
// }
// #[inline(always)]
// fn f1_body(a: u32) -> u32 {
//     a + 1
// }
//
// It might even be possible to derive post condtitions from pre conditions,
// and report them to the user. Problem is that the conditions are
// represented as "first order logic" (FOL) constraints, which need to be
// converted into readable form (preferably Rust expressions.)
//
// Another potential extension is the extend contracts, assumptions and assertions
// to support FOL natively, this would give us logic quantifiers:
// `forall` and `exists`. Using FOL we can reason on programs in a more
// general way, however the system will be significantly more difficult
// to understand and use, so it remains to be seen/evaluated.
