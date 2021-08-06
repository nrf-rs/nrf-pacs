#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0600],
    #[doc = "0x600 - Block protect configuration register 0"]
    pub config0: crate::Reg<config0::CONFIG0_SPEC>,
    #[doc = "0x604 - Block protect configuration register 1"]
    pub config1: crate::Reg<config1::CONFIG1_SPEC>,
    #[doc = "0x608 - Disable protection mechanism in debug interface mode"]
    pub disableindebug: crate::Reg<disableindebug::DISABLEINDEBUG_SPEC>,
    #[doc = "0x60c - Unspecified"]
    pub unused0: crate::Reg<unused0::UNUSED0_SPEC>,
    #[doc = "0x610 - Block protect configuration register 2"]
    pub config2: crate::Reg<config2::CONFIG2_SPEC>,
    #[doc = "0x614 - Block protect configuration register 3"]
    pub config3: crate::Reg<config3::CONFIG3_SPEC>,
}
#[doc = "CONFIG0 register accessor: an alias for `Reg<CONFIG0_SPEC>`"]
pub type CONFIG0 = crate::Reg<config0::CONFIG0_SPEC>;
#[doc = "Block protect configuration register 0"]
pub mod config0;
#[doc = "CONFIG1 register accessor: an alias for `Reg<CONFIG1_SPEC>`"]
pub type CONFIG1 = crate::Reg<config1::CONFIG1_SPEC>;
#[doc = "Block protect configuration register 1"]
pub mod config1;
#[doc = "DISABLEINDEBUG register accessor: an alias for `Reg<DISABLEINDEBUG_SPEC>`"]
pub type DISABLEINDEBUG = crate::Reg<disableindebug::DISABLEINDEBUG_SPEC>;
#[doc = "Disable protection mechanism in debug interface mode"]
pub mod disableindebug;
#[doc = "UNUSED0 register accessor: an alias for `Reg<UNUSED0_SPEC>`"]
pub type UNUSED0 = crate::Reg<unused0::UNUSED0_SPEC>;
#[doc = "Unspecified"]
pub mod unused0;
#[doc = "CONFIG2 register accessor: an alias for `Reg<CONFIG2_SPEC>`"]
pub type CONFIG2 = crate::Reg<config2::CONFIG2_SPEC>;
#[doc = "Block protect configuration register 2"]
pub mod config2;
#[doc = "CONFIG3 register accessor: an alias for `Reg<CONFIG3_SPEC>`"]
pub type CONFIG3 = crate::Reg<config3::CONFIG3_SPEC>;
#[doc = "Block protect configuration register 3"]
pub mod config3;
