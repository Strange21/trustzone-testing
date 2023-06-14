//! NVMC (i.e. flash) driver for the nrf52840 board, written in pure-rust.

use core::{
    ops::{Add, Sub},
    usize, cell::{OnceCell}, 
};

use nrf9160_pac as pac;

use crate::FlashInterface;
use pac::{Peripherals, NVMC_S, NVMC_NS};
use nrf9160_constants::*;

extern crate non_secure;

// #[cfg(feature = "defmt")]
// use defmt_rtt as _; // global logger

#[rustfmt::skip]
mod nrf9160_constants {
    pub const FLASH_PAGE_SIZE : u32 = 4096;
    pub const STACK_LOW       : u32 = 0x20_000_000;
    pub const STACK_UP        : u32 = 0x20_040_000;
    pub const RB_HDR_SIZE     : u32 = 0x100;
    pub const BASE_ADDR       : u32 = 0x40000;
    pub const VTR_TABLE_SIZE  : u32 = 0x100;
    pub const FW_RESET_VTR    : u32 = BASE_ADDR + RB_HDR_SIZE + VTR_TABLE_SIZE + 1;
}
// include!(concat!(env!("OUT_DIR"), "/trustzone_bindings.rs"));
// extern crate trustzone_m_nonsecure_rt;

pub struct FlashWriterEraser {
    pub nvmc_s: NVMC_S,
    pub nvmc_ns: NVMC_NS,
    pub secure: bool
}

impl FlashWriterEraser
{
    pub fn new(nvmc_s: NVMC_S, nvmc_ns:NVMC_NS, secure: bool) -> Self {
        FlashWriterEraser {
                nvmc_s: nvmc_s,
                nvmc_ns: nvmc_ns,
                secure: secure
            }
    }
}

impl FlashInterface for FlashWriterEraser {
    fn hal_flash_write(&self, address: usize, data: *const u8, len: usize) {
        let address = address as u32;
        let len = len as u32;
        let mut idx = 0u32;
        let mut src = data as *mut u32;
        let mut dst = address as *mut u32;
        // defmt::println!("writting {:?} at address {:?} len {:?}", data.clone() ,address.clone(), len.clone() );

        while idx < len {
            let data_ptr = (data as *const u32) as u32;
            // defmt::println!("hal flash write 1");
            if self.secure {
                // Check if the following holds true and do a full word write i.e. 4-byte write
                // - if `len - idx` is greater than 3 (i.e. 4 bytes).
                // - if the address is aligned on a word (i.e. 4-byte) boundary.
                // - if the data_ptr is aligned on a word (i.e. 4-byte) boundary.
                if ((len - idx > 3)
                    && ((((address + idx) & 0x03) == 0) && ((data_ptr + idx) & 0x03) == 0))
                {
                    // // defmt::println!("hal flash write 2");
                    // Enable NVM writes
                    
                    self.nvmc_s.configns.write(|w| w.wen().wen());
                    // // defmt::println!("Config nvmc enabled");
                    while self.nvmc_s.readynext.read().readynext().is_busy() {
                        // // defmt::println!("waiting for busy bit");
                    }
                    // // defmt::println!("NVMC enabled");
                    unsafe {
                        *dst = *src; // 4-byte write
                    };
                    // // defmt::println!("hal flash write 3");
                    // Wait until writing is done
                    while self.nvmc_s.ready.read().ready().is_busy() {}
                    // // defmt::println!("writting to memory done ");
                    src = ((src as u32) + 4) as *mut u32; // increment pointer by 4
                    dst = ((dst as u32) + 4) as *mut u32; // increment pointer by 4
                    idx += 4;
                } else {
                    // // defmt::println!("hal flash write 4");
                    // else do a single byte write i.e. 1-byte write
                    let mut val = 0u32;
                    let val_bytes = ((&mut val) as *mut u32) as *mut u8;
                    let offset = (address + idx) - (((address + idx) >> 2) << 2); // offset from nearest word aligned address
                    dst = ((dst as u32) - offset) as *mut u32; // subtract offset from dst addr
                    unsafe {
                        val = *dst; // assign current val at dst to val
                                    // store data byte at idx to `val`. `val_bytes` is a byte-pointer to val.
                        *val_bytes.add(offset as usize) = *data.add(idx as usize);
                    }
                    // // defmt::println!("hal flash write 5");
                    // Enable NVM writes
                    self.nvmc_s.configns.write(|w| w.wen().wen());
                    while self.nvmc_s.readynext.read().readynext().is_busy() {}
                    // // defmt::println!("hal flash write 6");
                    unsafe {
                        *dst = val; // Technically this is a 1-byte write ONLY
                                    // but only full 32-bit words can be written to Flash using the NVMC interface
                    };
                    // Wait until writing is done
                    while self.nvmc_s.ready.read().ready().is_busy() {}
                    // defmt::println!("hal flash write 7");
                    src = ((src as u32) + 1) as *mut u32; // increment pointer by 1
                    dst = ((dst as u32) + 1) as *mut u32; // increment pointer by 1
                    idx += 1;
                    // defmt::println!("doing single byte write");
                }
            }
            else {
                // Check if the following holds true and do a full word write i.e. 4-byte write
                // - if `len - idx` is greater than 3 (i.e. 4 bytes).
                // - if the address is aligned on a word (i.e. 4-byte) boundary.
                // - if the data_ptr is aligned on a word (i.e. 4-byte) boundary.
                if ((len - idx > 3)
                    && ((((address + idx) & 0x03) == 0) && ((data_ptr + idx) & 0x03) == 0))
                {
                    // // defmt::println!("hal flash write 2");
                    // Enable NVM writes
                    
                    self.nvmc_ns.configns.write(|w| w.wen().wen());
                    // // defmt::println!("Config nvmc enabled");
                    while self.nvmc_ns.readynext.read().readynext().is_busy() {
                        // // defmt::println!("waiting for busy bit");
                    }
                    // // defmt::println!("NVMC enabled");
                    unsafe {
                        *dst = *src; // 4-byte write
                    };
                    // // defmt::println!("hal flash write 3");
                    // Wait until writing is done
                    while self.nvmc_ns.ready.read().ready().is_busy() {}
                    // // defmt::println!("writting to memory done ");
                    src = ((src as u32) + 4) as *mut u32; // increment pointer by 4
                    dst = ((dst as u32) + 4) as *mut u32; // increment pointer by 4
                    idx += 4;
                } else {
                    // // defmt::println!("hal flash write 4");
                    // else do a single byte write i.e. 1-byte write
                    let mut val = 0u32;
                    let val_bytes = ((&mut val) as *mut u32) as *mut u8;
                    let offset = (address + idx) - (((address + idx) >> 2) << 2); // offset from nearest word aligned address
                    dst = ((dst as u32) - offset) as *mut u32; // subtract offset from dst addr
                    unsafe {
                        val = *dst; // assign current val at dst to val
                                    // store data byte at idx to `val`. `val_bytes` is a byte-pointer to val.
                        *val_bytes.add(offset as usize) = *data.add(idx as usize);
                    }
                    // // defmt::println!("hal flash write 5");
                    // Enable NVM writes
                    self.nvmc_ns.configns.write(|w| w.wen().wen());
                    while self.nvmc_ns.readynext.read().readynext().is_busy() {}
                    // // defmt::println!("hal flash write 6");
                    unsafe {
                        *dst = val; // Technically this is a 1-byte write ONLY
                                    // but only full 32-bit words can be written to Flash using the NVMC interface
                    };
                    // Wait until writing is done
                    while self.nvmc_ns.ready.read().ready().is_busy() {}
                    // defmt::println!("hal flash write 7");
                    src = ((src as u32) + 1) as *mut u32; // increment pointer by 1
                    dst = ((dst as u32) + 1) as *mut u32; // increment pointer by 1
                    idx += 1;
                    // defmt::println!("doing single byte write");
                }
            }
        }
    }


    fn hal_flash_erase(&self, addr: usize, len: usize) {
        let starting_page = (addr/0x1000) as u32;
        let ending_page = ((addr + len)/0x1000) as u32;

        // let address = starting_page * 0x1000;
        // defmt::info!("starting_page={}, ending_page={}, len={}", starting_page, ending_page, len);
        for start_addr in (starting_page..ending_page) {
            // Enable erasing
            if self.secure{
                // Enable the erase functionality of the flash
                self.nvmc_s.configns.modify(|_, w| w.wen().een());
                // Start the erase process by writing a u32 word containing all 1's to the first word of the page
                // This is safe because the flash slice is page aligned, so a pointer to the first byte is valid as a pointer to a u32.
                unsafe {
                    let first_word = (start_addr * 0x1000) as *mut u32;
                    first_word.write_volatile(0xFFFFFFFF);
                }
                // Wait for the erase to be done
                while self.nvmc_s.ready.read().ready().is_busy() {}

                self.nvmc_s.configns.modify(|_, w| w.wen().ren());
            }
            else {
                // Enable the erase functionality of the flash
                self.nvmc_ns.configns.modify(|_, w| w.wen().een());
                // Start the erase process by writing a u32 word containing all 1's to the first word of the page
                // This is safe because the flash slice is page aligned, so a pointer to the first byte is valid as a pointer to a u32.
                unsafe {
                    let first_word = (start_addr * 0x1000) as *mut u32;
                    first_word.write_volatile(0xFFFFFFFF);
                }
                // Wait for the erase to be done
                while self.nvmc_ns.ready.read().ready().is_busy() {}

                self.nvmc_ns.configns.modify(|_, w| w.wen().ren());
            }
        }
        // Synchronize the changes
        cortex_m::asm::dsb();
        cortex_m::asm::isb();
        
    }

    fn hal_init() {}
    fn hal_flash_lock(&self) {}
    fn hal_flash_unlock(&self) {}
}

pub fn preboot() {}

struct RefinedUsize<const MIN: u32, const MAX: u32, const VAL: u32>(u32);

impl<const MIN: u32, const MAX: u32, const VAL: u32> RefinedUsize<MIN, MAX, VAL> {
    pub fn bounded_int(i: u32) -> Self {
        assert!(i >= MIN && i <= MAX);
        RefinedUsize(i)
    }
    pub fn single_valued_int(i: u32) -> Self {
        // defmt::println!("i {:?} == VAL {:?}", i, VAL);
        assert!(i == VAL);
        RefinedUsize(i)
    }
}

#[rustfmt::skip]
// #[cmse_nonsecure_call]
pub fn boot_from(fw_base_address: usize) -> ! {
    unsafe {
        // defmt::println!("jumping to NS function at firmware address {:?}", fw_base_address.clone());
        non_secure::boot_from(fw_base_address as u32);
        // trustzone_bindings::boot_from(fw_base_address as u32);
        loop{}   
    }
}

pub fn jump(fw_base_address: u32) {    
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
