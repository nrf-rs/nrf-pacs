#[doc = r"Register block"]
#[repr(C)]
pub struct HFCLKAUDIO {
    #[doc = "0x00 - Audio PLL frequency in 11.176 MHz - 11.402 MHz or 12.165 MHz - 12.411 MHz frequency bands"]
    pub frequency: FREQUENCY,
}
#[doc = "FREQUENCY (rw) register accessor: an alias for `Reg<FREQUENCY_SPEC>`"]
pub type FREQUENCY = crate::Reg<frequency::FREQUENCY_SPEC>;
#[doc = "Audio PLL frequency in 11.176 MHz - 11.402 MHz or 12.165 MHz - 12.411 MHz frequency bands"]
pub mod frequency;
