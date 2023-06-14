#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(cmse_nonsecure_entry)]
#![feature(abi_c_cmse_nonsecure_call)]

use nrf9160_hal::{gpio, Uarte, uarte, prelude::OutputPin};
use nrf9160_pac::{P0_NS, UARTE0_NS};
use cortex_m_rt::entry;

#[cfg(feature = "defmt")]
use defmt_rtt as _; // global logger

#[entry]
fn main()->! {   
    
    let p0: P0_NS = unsafe { core::mem::transmute(()) };
    let p0 = gpio::p0::Parts::new(p0);
    let mut led3 = p0.p0_04.into_push_pull_output(gpio::Level::Low).degrade();
    let mut led4 = p0.p0_05.into_push_pull_output(gpio::Level::High).degrade();
    led3.set_high().unwrap();
    
    loop{
        led4.set_low().unwrap();
        cortex_m::asm::delay(5000000);
        led4.set_high().unwrap();
        cortex_m::asm::delay(5000000);
    }
    
}

/// Called when our code panics.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    cortex_m::asm::udf();
}

