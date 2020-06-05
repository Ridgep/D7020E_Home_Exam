#![no_std]
#![no_main]

use klee_sys::klee_abort;
use panic_klee as _;
use vcell;

#[no_mangle]
fn main() {
    let mut vc = vcell::VolatileCell::new(0u32);
    match vc.get() {
        2 => klee_abort!(),
        1 => klee_abort!(),
        _ => (),
    };
}

// cargo klee --example vcell_test
//
// We get three paths, as the vc is made symbolic.
//
// This is the way!
