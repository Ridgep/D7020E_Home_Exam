#![no_std]
#![no_main]

use klee_sys::klee_abort;
extern crate cortex_m;
extern crate panic_klee;

use cortex_m::peripheral::Peripherals;

static mut S: u32 = 100;

#[no_mangle]
fn main() {
    let peripherals = Peripherals::take().unwrap();
    let mut dwt = peripherals.DWT;
    let a = dwt.cyccnt.read();
    if a == unsafe { S } {
        panic!();
        // klee_abort!();
    }

    klee_abort!();
}

// In this example we showcase
// - how hardware reads are automatically made symbolic
// - semantic paths (LLVM-IR vs source code)
//
// In the Cargo.toml we pass the "klee-analysis" feature to `vcell` and `cortex-m`.
// This implies that hardware reads are made symbolic, while writes are surpressed.
//
// > cargo klee --example cortex_m_test1
// ...
// KLEE: ERROR: examples/cortex_m_test1.rs:22: abort failure
// KLEE: NOTE: now ignoring this error at this location
// KLEE: ERROR: /home/pln/.cargo/git/checkouts/panic-klee-aa8d015442188497/3b0c897/src/lib.rs:8: abort failure
// KLEE: NOTE: now ignoring this error at this location
//
// KLEE: done: total instructions = 743
// KLEE: done: completed paths = 4
// KLEE: done: generated tests = 2
//
// In this case KLEE generates to tests leading up to the terminating lines (errors)
// 18 and 22 respectively.
//
// This is due to that the `dwt.read` has rendered `a` as a symbolic value.
//
// Now let's comment out line 18 and un-comment line 19.
//
// > cargo klee --example cortex_m_test1
// ...
// KLEE: ERROR: examples/cortex_m_test1.rs:16: abort failure
// KLEE: NOTE: now ignoring this error at this location
//
// KLEE: done: total instructions = 663
// KLEE: done: completed paths = 2
// KLEE: done: generated tests = 1
//
// So, previously we made the claim that all `klee_abort` calls will render a unique test,
// but clearly this is not the case here.
//
// What happened is that the LLVM-IR the code has already been optimized to
// to semantically equivalent LLVM-IR representation.
//
// ; Function Attrs: nounwind nonlazybind
// define void @main() unnamed_addr #4 !dbg !1013 {
//     start:
// ...
//   call void @klee_make_symbolic(i8* %_3.i.i.i, i64 %24, i8* %27) #14, !dbg !1145
//   %28 = load i32, i32* %symbolic_value.i.i, align 4, !dbg !1146
//   store i32 %28, i32* %a, align 4, !dbg !1119
//   call void @abort(), !dbg !1147
//   unreachable, !dbg !1147
// ...
//
// So what we see here, is that the if statement is nowhere to be found
// and we call @obort(), and the code below is unreachable (as abort() -> !)
//
// This is semantically equivalent to:
// if a == unsafe { S } {
//     // panic!();
//     klee_abort!(); // (A)
// }
// klee_abort!(); // (B)
//
// So LLVM-IR paths and source code paths are not necessarily equivalent.
// rustc (the Rust compiler) and LLVM is free to make any choices of the representation
// as long as the semantics (meaning) is preserved.
//
// Does that imply that (A) is unreachable (dead code)?
//
// Well that depends on the view...
// rustc and/or LLVM has decided it is redundant and therefore not present in the LLVM-IR.
//
// What about code coverage?
//
// In this case there would be no "test" that visits this path, thus
// "classical" coverage test, would indicate that we have not covered all source code paths.
//
// So classical code coverage would go down, indicating lower quality test (bad for certfication).
// However we can proudly claim that the "classical" coverage as a measure of quality is
// indeed at fault. It is not the source code we should worry about!
//
// Our test provides "complete" coverage regarding the semantics of the source code
// (as we have not missed any potential error).
//
// This example is of course contrived, but showcase that "classical" measures/
// quality goals restated and certification procedures updated accordingly!
// This is the way!
