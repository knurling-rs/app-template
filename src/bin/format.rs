#![no_main]
#![no_std]

use {{ project_name | replace(from="-", to="_") }} as _; // global logger + panicking-behavior + memory layout
use defmt::Format; // <- derive attribute

#[derive(Format)]
struct S1<T> {
    x: u8,
    y: T,
}

#[derive(Format)]
struct S2 {
    z: u8,
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let s = S1 { x: 42, y: S2 { z: 43 } };
    defmt::info!("s={:?}", s);
    let x = 42;
    defmt::info!("x={:u8}", x);

    {{project_name | replace(from="-", to="_")}}::exit()
}
