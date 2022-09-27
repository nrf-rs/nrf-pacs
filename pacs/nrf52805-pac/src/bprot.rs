#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0600],
    #[doc = "0x600 - Block protect configuration register 0"]
    pub config0: CONFIG0,
    #[doc = "0x604 - Block protect configuration register 1"]
    pub config1: CONFIG1,
    #[doc = "0x608 - Disable protection mechanism in debug mode"]
    pub disableindebug: DISABLEINDEBUG,
}
#[doc = "CONFIG0 (rw) register accessor: an alias for `Reg<CONFIG0_SPEC>`"]
pub type CONFIG0 = crate::Reg<config0::CONFIG0_SPEC>;
#[doc = "Block protect configuration register 0"]
pub mod config0;
#[doc = "CONFIG1 (rw) register accessor: an alias for `Reg<CONFIG1_SPEC>`"]
pub type CONFIG1 = crate::Reg<config1::CONFIG1_SPEC>;
#[doc = "Block protect configuration register 1"]
pub mod config1;
#[doc = "DISABLEINDEBUG (rw) register accessor: an alias for `Reg<DISABLEINDEBUG_SPEC>`"]
pub type DISABLEINDEBUG = crate::Reg<disableindebug::DISABLEINDEBUG_SPEC>;
#[doc = "Disable protection mechanism in debug mode"]
pub mod disableindebug;
