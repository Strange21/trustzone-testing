use stm32f4xx_hal as hal;

use crate::FlashInterface;
use core::ptr::write_volatile;
use hal::pac::{Peripherals, FLASH};
use stm32f469rc_constants::*;
#[rustfmt::skip]
mod stm32f469rc_constants {
    pub const FLASH_PAGE_SIZE : u32 = 131072;   // 1 sector size = 128KB   
    pub const STACK_LOW       : u32 = 0x2000_0000;
    pub const STACK_UP        : u32 = 0x2002_0000;
    pub const RB_HDR_SIZE     : u32 = 0x100;
    pub const BASE_ADDR       : u32 = 0x08020000;   //  sector 5 starting address
    pub const VTR_TABLE_SIZE  : u32 = 0x100;
    pub const FW_RESET_VTR    : u32 = BASE_ADDR + RB_HDR_SIZE + VTR_TABLE_SIZE + 0xb5;
    pub const UNLOCKKEY1      : u32 = 0x45670123;
    pub const UNLOCKKEY2      : u32 = 0xCDEF89AB;
    pub const PSIZE_X8        : u8  = 0b00;
    pub const PSIZE_X16       : u8  = 0b01;
    pub const PSIZE_X32       : u8  = 0b10;
    pub const PSIZE_X64       : u8  = 0b11;
}

pub struct FlashWriterEraser {
    pub nvm: FLASH,
}

impl FlashWriterEraser {
    pub fn new() -> Self {
        FlashWriterEraser {
            nvm: Peripherals::take().unwrap().FLASH,
        }
    }
}

impl FlashInterface for FlashWriterEraser {
    /// This method is to write data on flash
    ///
    /// Method arguments:
    /// -   address: It holds the address of flash where data has to be written
    /// -   data: u8 pointer holding the holding data.
    /// -   len :  number of bytes
    ///
    /// Returns:
    /// -  NONE
    fn hal_flash_write(&self, address: usize, data: *const u8, len: usize) {
        let address = address as u32;
        let len = len as u32;
        let mut idx = 0u32;
        let mut src = data as *mut u32;
        let mut dst = address as *mut u32;
        //Unlock the FLASH
        self.hal_flash_unlock();
        while idx < len {
            let data_ptr = (data as *const u32) as u32;
            //checking if the len is more than 4 bytes to compute a 4 byte write on flash
            if (len - idx > 3) {
                // Enable FLASH Page writes
                self.nvm.cr.modify(|_, w| unsafe {
                    w.psize()
                        .bits(PSIZE_X32)
                        // no sector erase
                        .ser()
                        .clear_bit()
                        // programming
                        .pg()
                        .set_bit()
                });
                while self.nvm.sr.read().bsy().bit() {}
                unsafe {
                    // *dst = data; // 4-byte write
                    write_volatile(dst, *src);
                };

                src = ((src as u32) + 4) as *mut u32; // increment pointer by 4
                dst = ((dst as u32) + 4) as *mut u32; // increment pointer by 4
                idx += 4;
            } else {
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
                // Enable FLASH Page writes
                self.nvm.cr.modify(|_, w| unsafe {
                    w.psize()
                        .bits(PSIZE_X32)
                        // no sector erase
                        .ser()
                        .clear_bit()
                        // programming
                        .pg()
                        .set_bit()
                });
                while self.nvm.sr.read().bsy().bit() {}
                unsafe {
                    *dst = val; // Technically this is a 1-byte write ONLY
                                // but only full 32-bit words can be written to Flash using the NVMC interface
                };
                src = ((src as u32) + 1) as *mut u32; // increment pointer by 1
                dst = ((dst as u32) + 1) as *mut u32; // increment pointer by 1
                idx += 1;
            }
        }
        //Lock the FLASH
        self.hal_flash_lock();
    }

    /// This method is used to erase data on flash
    ///
    /// In STM32F469 only sector erase is available. whatever be the length of bytes we pass to this function will erase
    /// the whole sector, whichever the sector the address belong to.
    ///
    /// Method arguments:
    /// -   addr: Address where data has to be erased
    /// -   len :  number of bytes to be erased
    ///
    /// Returns:
    /// -  NONE

    fn hal_flash_erase(&self, addr: usize, len: usize) {
        let mut sec: u8 = 0;
        let mut flag: bool = true;
        let address = addr as u32;
        match address {
            (0x0800_0000..=0x0800_3FFF) => sec = 0,
            (0x0800_4000..=0x0800_7FFF) => sec = 1,
            (0x0800_8000..=0x0800_BFFF) => sec = 2,
            (0x0800_C000..=0x0800_FFFF) => sec = 3,
            (0x0801_0000..=0x0801_FFFF) => sec = 4,
            (0x0802_0000..=0x0803_FFFF) => sec = 5,
            (0x0804_0000..=0x0805_FFFF) => sec = 6,
            (0x0806_0000..=0x0807_FFFF) => sec = 7,
            (0x0808_0000..=0x0809_FFFF) => sec = 8,
            (0x080A_0000..=0x080B_FFFF) => sec = 9,
            (0x080C_0000..=0x080D_FFFF) => sec = 10,
            (0x080E_0000..=0x080F_FFFF) => sec = 11,
            (0x0810_0000..=0x0810_3FFF) => sec = 12,
            (0x0810_4000..=0x0810_7FFF) => sec = 13,
            (0x0810_8000..=0x0810_BFFF) => sec = 14,
            (0x0810_C000..=0x0810_FFFF) => sec = 15,
            (0x0811_0000..=0x0811_FFFF) => sec = 16,
            (0x0812_0000..=0x0813_FFFF) => sec = 17,
            (0x0814_0000..=0x0815_FFFF) => sec = 18,
            (0x0816_0000..=0x0817_FFFF) => sec = 19,
            (0x0818_0000..=0x0819_FFFF) => sec = 20,
            (0x081A_0000..=0x081B_FFFF) => sec = 21,
            (0x081C_0000..=0x081D_FFFF) => sec = 22,
            (0x081E_0000..=0x081F_FFFF) => sec = 23,
            _ => flag = false,
        }

        if flag {
            self.hal_flash_unlock();
            // Erase page starting at addr
            #[rustfmt::skip]
            self.nvm.cr.modify(|_, w| unsafe {
                w
                    // start
                    .strt().set_bit()
                    .psize().bits(PSIZE_X8)
                    // sector number
                    .snb().bits(sec)
                    // sectore erase
                    .ser().set_bit()
                    // no programming
                    .pg().clear_bit()
            });
            // Wait until erasing is done
            while self.nvm.sr.read().bsy().bit() {}
            //Lock the FLASH
            self.hal_flash_lock();
        }
    }
    /// This method is used to lock the flash
    ///
    /// Once the flash is locked no operation on flash can be perfomed.
    /// Method arguments:
    /// -   NONE
    /// Returns:
    /// -  NONE
    fn hal_flash_lock(&self) {
        self.nvm.cr.modify(|_, w| w.lock().set_bit());
    }
    /// This method is used to unlock the flash
    ///
    /// Flash has to be unlocked to do any operation on it.
    /// Method arguments:
    /// -   NONE
    /// Returns:
    /// -  NONE
    fn hal_flash_unlock(&self) {
        self.nvm.keyr.write(|w| unsafe { w.key().bits(UNLOCKKEY1) });
        self.nvm.keyr.write(|w| unsafe { w.key().bits(UNLOCKKEY2) });
    }
    fn hal_init() {}
}
pub fn preboot() {}

struct RefinedUsize<const MIN: u32, const MAX: u32, const VAL: u32>(u32);

impl<const MIN: u32, const MAX: u32, const VAL: u32> RefinedUsize<MIN, MAX, VAL> {
    /// This method is used to check the address bound of stack pointer
    ///
    /// Method arguments:
    /// -   i : starting address of stack  
    /// Returns:
    /// -  It returns u32 address of stack pointer
    pub fn bounded_int(i: u32) -> Self {
        assert!(i >= MIN && i <= MAX);
        RefinedUsize(i)
    }
    /// This method is used to check the address of reset pointer
    ///
    /// Method arguments:
    /// -   i : starting address of reset  
    /// Returns:
    /// -  It returns u32 address of reset pointer
    pub fn single_valued_int(i: u32) -> Self {
        assert!(i == VAL);
        RefinedUsize(i)
    }
}

/// This method is used to boot the firmware from a particular address
    ///
    /// Method arguments:
    /// -   fw_base_address  : address of the firmware
    /// Returns:
    /// -  NONE
#[rustfmt::skip]
pub fn boot_from(fw_base_address: usize) -> ! {
       let address = fw_base_address as u32;
       let scb = hal::pac::SCB::ptr();
       unsafe {
       let sp = RefinedUsize::<STACK_LOW, STACK_UP, 0>::bounded_int(
        *(fw_base_address as *const u32)).0;
       let rv = RefinedUsize::<0, 0, FW_RESET_VTR>::single_valued_int(
        *((fw_base_address + 4) as *const u32)).0;
       let jump_vector = core::mem::transmute::<usize, extern "C" fn() -> !>(rv as usize);
       (*scb).vtor.write(address);
       cortex_m::register::msp::write(sp);
       jump_vector();
    
       }
       loop{}
}
