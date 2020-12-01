#![no_std]
#![no_main]

use {{crate_name}} as _; // memory layout + panic handler

// See https://crates.io/crates/defmt-test/0.1.0 for more documentation (e.g. about the 'state'
// feature)
#[defmt_test::tests]
mod tests {
    #[test]
    fn assert_true() {
        defmt::assert!(true)
    }

    #[test]
    fn assert_eq() {
        defmt::assert_eq!(24, 42, "TODO: write actual tests")
    }
}
