#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use bare_test::{print, println, time::since_boot};

pub mod sdhci_reg;
pub mod sdhci_cmd;
pub mod sdhci;

pub fn delay_us(us: u64) {
    let start = since_boot();
    let duration = core::time::Duration::from_micros(us);
    
    while since_boot() - start < duration {
        core::hint::spin_loop();
    }
}
