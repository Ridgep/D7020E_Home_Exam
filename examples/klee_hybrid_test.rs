// example to show conditional compilation of hybrid applications
// compiling both for klee analysis and just as bare metal
//
#![no_std]
#![no_main]

#[cfg(feature = "klee-analysis")]
mod klee_analysis {
    use klee_sys::*;
    use panic_klee as _;

    // we could use `cortex-m-rt` and entry here as well but it introduces some noise.
    // so in principle main could be exactly the same, for analysis and run
    #[no_mangle]
    fn main() {
        let mut v = 0;
        klee_make_symbolic!(&mut v, "v");
        let r = super::f(v);
    }
}

#[cfg(not(feature = "klee-analysis"))]
mod bare_metal {
    extern crate panic_halt;
    use cortex_m_rt::entry;
    use cortex_m_semihosting::hprintln;
    use stm32f4::stm32f401 as stm32;

    #[entry]
    fn main() -> ! {
        let r = super::f(72); // 72 happened to be the value but it could be u32::MAX, who knows?
        hprintln!("Hello, world! {}", r).unwrap();

        loop {}
    }
}
// some potentially dangerous function
fn f(v: u32) -> u32 {
    v + 1
}

// For KLEE analysis:
// cargo klee --example klee_hybrid_test
//
// For bare metal execution
// cargo run --example klee_hybrid_test --features f4 --target thumbv7em-none-eabihf
//
// This is the way!
