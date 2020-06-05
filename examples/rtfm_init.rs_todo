//! examples/init.rs

// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

use klee_sys::{klee_abort, klee_assert, klee_assert_eq, klee_make_symbolic};
use panic_klee as _;

#[rtfm::app(device = lm3s6965)]
const APP: () = {
    #[init]
    fn init(cx: init::Context) {
        static mut X: u32 = 0;

        // // Safe access to local `static mut` variable
        // let _x: &'static mut u32 = X;
        // if cfg!(feature = "klee-analysis") {
        //     fn_to_test();
        // } else {
        //     hprintln!("init").unwrap();
        //     debug::exit(debug::EXIT_SUCCESS);
        // }
    }
};

// #[cfg(feature = "klee-analysis")]
// fn fn_to_test() {
//     let mut a = 0;
//     klee_make_symbolic!(&mut a, "a");
//     match a {
//         0 => klee_abort!(),
//         1 => klee_abort!(),
//         2 => panic!(),
//         3 => panic!("3"), // just one instance of panic! will be spotted
//         4 => klee_assert!(false),
//         5 => klee_assert_eq!(false, true),
//         6 => klee_assert_eq!(false, true),
//         _ => (),
//     };
//     panic!("at end"); // just one instane of panic! will be spotted
// }

// #[cfg(not(feature = "klee-analysis"))]
// fn fn_to_test() {}
