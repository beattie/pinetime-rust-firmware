#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // Initialize hardware peripherals here

    loop {
        // Main firmware loop
    }
}
