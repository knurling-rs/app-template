#![no_std]
#![no_main]

use cortex_m_rt::entry;
use {{crate_name}} as _; // memory layout + panic handler

#[entry]
fn main() -> ! {
    assert!(false, "TODO: Write actual tests");

    {{crate_name}}::exit();
}
