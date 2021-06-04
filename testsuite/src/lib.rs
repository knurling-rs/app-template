#![no_std]
#![cfg_attr(test, no_main)]

use {{crate_name}} as _; // memory layout + panic handler

#[defmt_test::tests]
mod tests {}
