#![no_std]
#![no_main]

use cortex_m_rt::{entry, pre_init};
use panic_klee as _;

use klee_sys::klee_abort;

use cortex_m::peripheral::Peripherals;

const X: u32 = 100;

#[entry]
unsafe fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut dwt = peripherals.DWT;
    dwt.enable_cycle_counter();
    let a = dwt.cyccnt.read();
    match a {
        0 => klee_abort!(),
        X => klee_abort!(),
        _ => (),
    }
    klee_abort!();
}

#[pre_init]
unsafe fn pre_init() {}

// This is the way!
