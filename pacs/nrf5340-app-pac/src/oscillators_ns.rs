#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x05c4],
    #[doc = "0x5c4 - Programmable capacitance of XC1 and XC2"]
    pub xosc32mcaps: crate::Reg<xosc32mcaps::XOSC32MCAPS_SPEC>,
    _reserved1: [u8; 0xf8],
    #[doc = "0x6c0..0x6d4 - Unspecified"]
    pub xosc32ki: XOSC32KI,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct XOSC32KI {
    #[doc = "0x00 - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    pub bypass: crate::Reg<self::xosc32ki::bypass::BYPASS_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Control usage of internal load capacitors"]
    pub intcap: crate::Reg<self::xosc32ki::intcap::INTCAP_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod xosc32ki;
#[doc = "XOSC32MCAPS register accessor: an alias for `Reg<XOSC32MCAPS_SPEC>`"]
pub type XOSC32MCAPS = crate::Reg<xosc32mcaps::XOSC32MCAPS_SPEC>;
#[doc = "Programmable capacitance of XC1 and XC2"]
pub mod xosc32mcaps;
