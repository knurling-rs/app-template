#![no_main]
#![no_std]

use {{ project_name | replace(from="-", to="_") }} as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    {{project_name | replace(from="-", to="_")}}::exit()
}
