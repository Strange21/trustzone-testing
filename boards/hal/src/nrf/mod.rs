#![feature(cmse_nonsecure_entry)]
#![feature(abi_c_cmse_nonsecure_call)]

#[cfg(feature = "nrf52840")]
pub mod nrf52840;
#[cfg(feature = "nrf9160")]
pub mod nrf9160;
