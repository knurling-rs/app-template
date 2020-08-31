#![no_std]
#![no_main]

use cortex_m_rt::entry;
use {{project-name}} as _; // memory layout + panic handler

#[entry]
fn main() -> ! {
    assert!(false, "TODO: Write actual tests");

    {{project-name}}::exit();
}
