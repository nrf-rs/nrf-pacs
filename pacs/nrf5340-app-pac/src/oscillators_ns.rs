#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x05c4],
    #[doc = "0x5c4 - Programmable capacitance of XC1 and XC2"]
    pub xosc32mcaps: XOSC32MCAPS,
    _reserved1: [u8; 0xf8],
    #[doc = "0x6c0..0x6d4 - Unspecified"]
    pub xosc32ki: XOSC32KI,
}
#[doc = "XOSC32MCAPS (rw) register accessor: an alias for `Reg<XOSC32MCAPS_SPEC>`"]
pub type XOSC32MCAPS = crate::Reg<xosc32mcaps::XOSC32MCAPS_SPEC>;
#[doc = "Programmable capacitance of XC1 and XC2"]
pub mod xosc32mcaps;
#[doc = "Unspecified"]
pub use xosc32ki::XOSC32KI;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod xosc32ki;
