#![no_main]
#![no_std]

use {{crate_name}} as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("main");

    defmt::panic!()
}
