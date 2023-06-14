#![no_std]
#![no_main]


use nrf9160_hal::{gpio, Uarte, uarte, prelude::OutputPin};
use nrf9160_pac::{P0_NS, UARTE0_NS};
// extern crate trustzone_m_nonsecure_rt;
// use trustzone_m_macros::secure_callable;
// use rustBoot_hal::nrf::nrf9160::jump;


// #[cfg(feature = "defmt")]
// use defmt_rtt as _; // global logger

// include!(concat!(env!("OUT_DIR"), "/trustzone_bindings.rs"));

#[no_mangle]
// #[cmse_nonsecure_entry]
pub extern "C" fn boot_from(fw_base_address: u32) { 
    // defmt::println!("In NS function");
    // let data :[u8;8]= [0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF];
    // let p0: P0_NS = unsafe { core::mem::transmute(()) };
    // let p0 = gpio::p0::Parts::new(p0);
    // let uarte0: UARTE0_NS = unsafe { core::mem::transmute(()) };
    
    // let pins = uarte::Pins {
    //     txd: p0.p0_02.into_push_pull_output(gpio::Level::High).degrade(),
    //     rxd: p0.p0_00.into_floating_input().degrade(),
    //     cts: None,
    //     rts: None,
    // };
    
    // let mut uarte = Uarte::new(uarte0, pins, uarte::Parity::EXCLUDED, uarte::Baudrate::BAUD1200);
    
    // uarte.write(&data).unwrap();  
    // jump(fw_base_address); 
    unsafe {
        let vector_table = fw_base_address as *const u32;
        let msp = *vector_table.offset(0) as *const u32;
        let rsv = vector_table.offset(1);

        let ns_rsv = (*rsv) & !1;
        cortex_m::asm::dsb();
        cortex_m::asm::isb();
        cortex_m::asm::bootstrap(msp, ns_rsv as *const u32);
    }
    
    
}

#[no_mangle]
pub fn hello_test(fw_base_address: u32) {
    loop{

    }
}

/// Called when our code panics.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    cortex_m::asm::udf();
}
