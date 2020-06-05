#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception, pre_init, ExceptionFrame};
use panic_klee as _;

use klee_sys::{klee_abort, klee_make_symbolic};

use cortex_m::peripheral::Peripherals;

static mut S: u32 = 100;

#[entry]
unsafe fn main() -> ! {
    // let peripherals = Peripherals::take().unwrap();
    // let mut dwt = peripherals.DWT;
    // let a = dwt.cyccnt.read();
    // if a == unsafe { S } {
    //     panic!();
    // }

    klee_abort!();
}

#[pre_init]
unsafe fn pre_init() {
    // let mut a = 0;
    // klee_make_symbolic!(&mut a, "a");
    // if a == 100 {
    //     klee_abort!();
    // }
}

#[exception]
fn DefaultHandler(irqn: i16) -> ! {
    // static mut X: i16 = 0;
    // if irqn > 255 {
    //     unsafe { core::ptr::write_volatile(&mut X, irqn) };
    //     klee_abort!();
    // } else {
    //     unsafe { core::ptr::write_volatile(&mut X, 0) };
    //     klee_abort!();
    // }
    klee_abort!();
}

#[exception]
fn NonMaskableInt() -> ! {
    klee_abort!();
}

#[exception]
fn HardFault(eh: &ExceptionFrame) -> ! {
    // match eh.xpsr {
    //     0 => klee_abort!(),
    //     1 => klee_abort!(),
    //     2 => klee_abort!(),
    //     _ => (),
    // }
    klee_abort!();
}

#[exception]
fn MemoryManagement() {
    // klee_abort!();
}

#[exception]
fn BusFault() {
    //klee_abort!();
}

#[exception]
fn UsageFault() {
    //klee_abort!();
}

#[exception]
fn SecureFault() {
    //klee_abort!();
}

#[exception]
fn SVCall() {
    //klee_abort!();
}

#[exception]
fn DebugMonitor() {
    //klee_abort!();
}

#[exception]
fn PendSV() {
    //klee_abort!();
}

#[exception]
fn SysTick() {
    //klee_abort!();
}

// Test all bindings to the cortex core.
// Todo: handle weak bindings
// Todo: handle arbitrary interrupt bindings
// This is the way!
