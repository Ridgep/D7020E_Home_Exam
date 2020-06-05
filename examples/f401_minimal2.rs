// minimal example for the stm32-f401 (and the f4 series)
//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

extern crate panic_halt;

use stm32f4::stm32f401 as stm32;

use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
#[inline(never)] // to keep the symbol
fn main() -> ! {
    asm::nop();
    asm::nop();
    asm::bkpt();
    do_some_work();
    asm::bkpt();
    loop {
        asm::nop();
    }
}

fn do_some_work() {
    for _ in 0..100 {
        asm::nop();
    }
}
// cargo run --example f401_minimal2 --features f4 --target thumbv7em-none-eabihf
// ...
// halted: PC: 0x08000322
// DefaultPreInit () at /home/pln/.cargo/git/checkouts/cortex-m-rt-073d0396a6df513c/8d26860/src/lib_thumb_rt.rs:571
// 571     pub unsafe extern "C" fn DefaultPreInit() {}
// (gdb) disassemble
// Dump of assembler code for function DefaultPreInit:
// => 0x08000322 <+0>:     bx      lr
// End of assembler dump.
// (gdb) c
// Continuing.

// Breakpoint 1, main () at examples/f401_minimal2.rs:14
// 14      #[entry]
// (gdb) c
// Continuing.
// halted: PC: 0x0800019a

// Program received signal SIGTRAP, Trace/breakpoint trap.
// 0x08000420 in __bkpt ()
// (gdb) backtrace
// #0  0x08000420 in __bkpt ()
// #1  0x080001a6 in cortex_m::asm::bkpt () at /home/pln/.cargo/git/checkouts/cortex-m-514878a7410beb63/d8f2851/src/asm.rs:19
// #2  f401_minimal2::__cortex_m_rt_main () at examples/f401_minimal2.rs:18
// inline-frame.c:156: internal-error: void inline_frame_this_id(frame_info*, void**, frame_id*): Assertion `frame_id_p (*this_id)' failed.
// A problem internal to GDB has been detected,
// further debugging may prove unreliable.
// Quit this debugging session? (y or n) n
//
// This is a bug, please report it.  For instructions, see:
// <http://www.gnu.org/software/gdb/bugs/>.
//
// inline-frame.c:156: internal-error: void inline_frame_this_id(frame_info*, void**, frame_id*): Assertion `frame_id_p (*this_id)' failed.
// A problem internal to GDB has been detected,
// further debugging may prove unreliable.
// Create a core file of GDB? (y or n) n
// Command aborted.
// .... eeeehhh no worries, GDB is written in C and still kind of works
// (gdb) frame 2
// #2  f401_minimal2::__cortex_m_rt_main () at examples/f401_minimal2.rs:18
// 18          asm::bkpt();
// So we can get back to the "caller" (that called the `__bkpt()` function).
// At this point we can disassassemble the program.
//
// (gdb) disassemble
// Dump of assembler code for function f401_minimal2::__cortex_m_rt_main:
// 0x08000584 <+0>:     bl      0x800077c <__nop>
// 0x08000588 <+4>:     bl      0x800077c <__nop>
// 0x0800058c <+8>:     bl      0x8000778 <__bkpt>
// => 0x08000590 <+12>:    bl      0x800059c <f401_minimal2::do_some_work>
// 0x08000594 <+16>:    b.n     0x8000596 <f401_minimal2::__cortex_m_rt_main+18>
// 0x08000596 <+18>:    bl      0x800077c <__nop>
// 0x0800059a <+22>:    b.n     0x8000596 <f401_minimal2::__cortex_m_rt_main+18>
//
// We see the two calls to the nop function, the bkpt call, the call to do_some_work_
// followed by the infinite loop.
// Great!
//
// Now lets try to find out how long it takes to `do_some_work`.
//
// One way would be by stepping the code and counting the number of steps.
// That is tedious and boring, and prone to errors.
//
// The ARM M4 core has some nice features allowing making our life easier.
//
// The DWT (Debug and Watchpoint and Trace Unit)
// http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.100166_0001_00_en/ric1417175910926.html
// And in particular, the DWT programmers model.
// What we want to do is
// (gdb) mon mww 0xe0001000 1
// (gdb) mon mww 0xe0001004 0
// (gdb) mon mdw 0xe0001004
// Which
//  - enables the DWT
//  - sets the cyclecounter to 0
//  - reads the cyclcounter
//
// Now let the program run (from the first bkpt to the next bkpt.
// (gdb) c
// Continuing.
// Program received signal SIGTRAP, Trace/breakpoint trap.
// 0x0800077c in __bkpt ()
//
// (gdb) mon mdw 0xe0001004
// 0xe0001004: 0000b92f
//
// Alternativle you may use the `x` (examine memory command)
// (gdb) x 0xe0001004
// 0xe0001004:     0x0000b92f
//
// Or in decimal
// (gdb) x/d 0xe0001004
// 0xe0001004:     47407
//
// So some 47k clock cycles... Hmm quite a lot for a loop 0..100 doing ... nothing
//
// Let's repeat the experiment in --release (optimized mode)
//
// cargo run --example f401_minimal2 --features f4 --target thumbv7em-none-eabihf --release
// ...
// run to first breakpoint, setup the DWT cyclecounter, run to breakpoint
// I got 505, cycles. Good but not great... still 5 cycles per nop, right?
//
// Well let's look at the code.
// (gdb) disassemble
// Dump of assembler code for function f401_minimal2::__cortex_m_rt_main:
// => 0x0800019a <+0>:     bl      0x80003f4 <__nop>
// 0x0800019e <+4>:     bl      0x80003f4 <__nop>
// 0x080001a2 <+8>:     bl      0x80003f0 <__bkpt>
// 0x080001a6 <+12>:    bl      0x80003f4 <__nop>
// 0x080001aa <+16>:    bl      0x80003f4 <__nop>
// 0x080001ae <+20>:    bl      0x80003f4 <__nop>
// 0x080001b2 <+24>:    bl      0x80003f4 <__nop>
// 0x080001b6 <+28>:    bl      0x80003f4 <__nop>
// 0x080001ba <+32>:    bl      0x80003f4 <__nop>
// 0x080001be <+36>:    bl      0x80003f4 <__nop>
// 0x080001c2 <+40>:    bl      0x80003f4 <__nop>
// 0x080001c6 <+44>:    bl      0x80003f4 <__nop>
// 0x080001ca <+48>:    bl      0x80003f4 <__nop>
// 0x080001ce <+52>:    bl      0x80003f4 <__nop>
// 0x080001d2 <+56>:    bl      0x80003f4 <__nop>
// ... a 100 in a row, that's some heavy inlining right!
//
// Well at this point, Rust + LLVM is not ALLOWED to do better.
// The assembly (nop) instruction is marked "volatile", meaning it implies
// a side effect, so Rust + LLVM is not allowed to optimize it out.
// That is indeed excellent, as we will discuss later.
//
// But why is the `__nop` a function call and not a native assebly nop?
// Well, Rust has yet to decide on a "stable" format for inline assembly.
// It's merely a syntactical thing, and the RFC has not yet been accepted
// (in due time we will have it.)
//
// In the meantime we can use inline assembly as an "unstable" feature
// by enabled in the nightly toolchain.
//
// > rustup override set nightly
//
// Now we have enabled the nightly toolchain for the current directory (a bit Nix like)
//
// cargo run --example f401_minimal2 --features f4 --target thumbv7em-none-eabihf --release --features inline-asm
//
// run until the first bkpt instruction...
//
// Program received signal SIGTRAP, Trace/breakpoint trap.
// f401_minimal2::__cortex_m_rt_main () at examples/f401_minimal2.rs:19
// 19          asm::bkpt();
// (gdb) disassemble
// Dump of assembler code for function f401_minimal2::__cortex_m_rt_main:
//    0x0800019a <+0>:     nop
//    0x0800019c <+2>:     nop
// => 0x0800019e <+4>:     bkpt    0x0000
//    0x080001a0 <+6>:     nop
//    0x080001a2 <+8>:     nop
//    0x080001a4 <+10>:    nop
//    0x080001a6 <+12>:    nop
//    0x080001a8 <+14>:    nop
//    0x080001aa <+16>:    nop
//    0x080001ac <+18>:    nop
//    0x080001ae <+20>:    nop
//    0x080001b0 <+22>:    nop
//    0x080001b2 <+24>:    nop
//    0x080001b4 <+26>:    nop
//
// (gdb) mon mww 0xe0001000 1
// (gdb) mon mww 0xe0001004 0
// (gdb) c
// Continuing.
//
// Program received signal SIGTRAP, Trace/breakpoint trap.
// f401_minimal2::__cortex_m_rt_main () at examples/f401_minimal2.rs:21
// 21          asm::bkpt();
// (gdb) mon mdw 0xe0001004
// 0xe0001004: 00000064
//
// (gdb) x/d 0xe0001004
// 0xe0001004:     100
//
// So what have we learned?
// - how we can do NON INTRUSIVE cycle accurate execution time measuremnts
// - how we can speed up a program 470 times without altering its semantics
//
// So back to the question on "volatile" (assembly) instructions.
// "volatile" in this context implies a number of things.
// - it may note be optimized out
// - the order of volatile instructions along an execution path must be preserved
//
// This allows us to
// - write inline assembler for parts of our code that is extremely timing critical
// - introduce precise delays
// - fiddle with CPU registers
//   - stack pointer
//   - link register
//   - special purpose core registers (interrupt enable/disable etc.)
// Essentially using inline assembly we can ALL in Rust. We do not NEED to link
// to external assembly code or external C code (still we can if we want)...
//
// All in Rust, That is the Way!
