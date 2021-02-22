#![no_main]
#![no_std]

use {{crate_name}} as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    ack(10, 10);
    {{crate_name}}::exit()
}

fn ack(m: u32, n: u32) -> u32 {
    defmt::info!("ack(m={=u32}, n={=u32})", m, n);
    let mut big = [2; 512];
    if m == 0 {
        n + 1
    } else {
        big[100] += 1;
        if n == 0 {
            ack(m - 1, 1)
        } else {
            ack(m - 1, ack(m, n - 1))
        }
    }
}
