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
// cargo run --example f401_probe --features f4 --target thumbv7em-none-eabihf
// ...
//
// The ARM M4 core provides the DWT register for cycle accurate measurements (CYCCNT).
// There is a lot more possibilities to tracing etc. that we will not cover in this course
//
// The real power however comes from automation.
// gdb (designed in the mid 80s) offers automation through python (natively)
// and bindings to sereverals high level languages have been developed.
//
// However, internally gdb has a lot of technical dept, and high internal complexity
// partly due to C and the ad-hoc threading model (threading was introduced along the way
// and not part of the "design".)
//
// More recently the llvm project has released debugger with largely compatible
// interface and automation capabalities. lldb is however not yet supporting
// embedded systems (so we stick to gdb atm)
//
// In both cases their code base is huge, and they are vary capable as designed for
// general purpose debugging.
//
// What we need in order to profile embedded code is however just a small but very
// precise subset of the functionality, defined by the ARM coresight API (for core access)
// and probing functionality (providid by e.g. stlink or DAP link).
//
// Using the latter you can e.g.:
// - read and write bytes/words and blocks thereof
// - read and write flash (writing is section based, first eraze then program)
//
// This type of low-level debug access is sufficent to automate profiling,
// tracing, etc.
//
// The recent Rust `probe-rs` team aims at supporting probing and coresight APIs.
// While the code base is young and immature, it already provides what we need to:
// - program (flash code), this was implemented for the M4 this weekend
// - stepping to and from breakpoint instructions
// - managing the DWT
// The latter two I implemented yesterday, so be aware there might be bugs out there...
//
// So here we go:
// 1. confirm that the above program compiles and runs as expected.
// 2. go to the `runner` folder and run `cargo run --bin f401_probe`
//
// You should get something like:
// device STLink V2-1 (VID: 1155, PID: 14155, STLink)
// probe connected
// strategy ChipInfo(ChipInfo { manufacturer: JEP106Code({ cc: 0x00, id: 0x20 } => Some("STMicroelectronics")), part: 1041 })
// target Target { identifier: TargetIdentifier { chip_name: "STM32F411VETx", ...
// Continue running
// Hit breakpoint :Core stopped at address 0x0800019e
// Continue from breakpoint.
// running
// Hit breakpoint :Core stopped at address 0x08000268
// cycles 100
//
// Have a close look at the small program, and the `runner` library.
//
// Hints:
// cargo doc --open
// Documenst the API, and lets you browse the documentation and source code behind.
//
// cargo objdump --example f401_probe --release --features f4,inline-asm --target thumbv7em-none-eabihf -- -d
// Part of the `cargo bin-utils` package that lets you inspect the generated binary.
//
// Later you will build a small analysis framework combining symbolic execution
// with the measurement based testing, to derive WCET estimates for embedded applications.
//
// This is the Way!
