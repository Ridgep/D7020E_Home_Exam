#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;

use cortex_m_rt::{entry, pre_init};
use panic_klee as _;

use klee_sys::{klee_abort, klee_make_symbolic};

use cortex_m::peripheral::Peripherals;
extern crate vcell;
use volatile_register::RW;

use core::ptr::read_volatile;
#[entry]
unsafe fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    // let peripherals = Peripherals::take().unwrap();
    let mut dwt = peripherals.DWT;
    dwt.enable_cycle_counter();
    let a: u32 = dwt.cyccnt.read();
    let b: u32 = dwt.cyccnt.read();
    let c: u32 = dwt.cyccnt.read();
    let some_time_quota = a / (c - (b - 100));
    unsafe {
        read_volatile(&some_time_quota); // prevent optimization
    }
    klee_abort!();
}

#[pre_init]
unsafe fn pre_init() {}

// This is the way.
