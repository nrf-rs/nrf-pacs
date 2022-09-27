#[doc = r"Register block"]
#[repr(C)]
pub struct XOSC32KI {
    #[doc = "0x00 - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    pub bypass: BYPASS,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Control usage of internal load capacitors"]
    pub intcap: INTCAP,
}
#[doc = "BYPASS (rw) register accessor: an alias for `Reg<BYPASS_SPEC>`"]
pub type BYPASS = crate::Reg<bypass::BYPASS_SPEC>;
#[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
pub mod bypass;
#[doc = "INTCAP (rw) register accessor: an alias for `Reg<INTCAP_SPEC>`"]
pub type INTCAP = crate::Reg<intcap::INTCAP_SPEC>;
#[doc = "Control usage of internal load capacitors"]
pub mod intcap;
