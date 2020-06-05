// minimal example for the stm32-f401 (and the f4 series)
//! Prints "Hello, world!" on the host console using semihosting
#![feature(asm)]
#![no_main]
#![no_std]

extern crate panic_halt;

use stm32f4::stm32f401 as stm32;

use cortex_m::{asm, bkpt};
use cortex_m_rt::entry;
// // use klee_sys::klee_make_symbolic2;
// // Mimic RTFM resources
// static mut X: u32 = 54;
#[entry]
#[inline(never)]
fn main() -> ! {
    let mut x = 54;

    klee_make_symbolic(&mut x);

    if x == 0 {
        bkpt!();
    }

    loop {
        asm::nop();
    }
}

#[inline(never)]
pub fn klee_make_symbolic<T>(data: &mut T) {
    // force llvm to consider data to be mutaded
    unsafe {
        asm!("bkpt #0" : /* output */: /* input */ "r"(data): /* clobber */ : "volatile")
    }
}

// pub fn taint() {
//     unsafe {
//         X = 73;
//     }
// }

// #[no_mangle]
// pub extern "C" fn klee_bkpt(data: *mut core::ffi::c_void) {
//     bkpt!();
// }

// extern "C" {
//     pub fn klee_bkpt(ptr: *mut core::ffi::c_void, // pointer to the data
//     );
// }
// cargo objdump --bin app --release -- -disassemble -no-show-raw-insn

// unsafe { asm!("mov $0,R15" : "=r"(r) ::: "volatile") }
// cargo objdump --example f401_ktest --release --features f4,inline-asm --target thumbv7em-none-eabihf -- -d
