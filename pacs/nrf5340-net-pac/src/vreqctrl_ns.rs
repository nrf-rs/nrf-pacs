#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0500],
    #[doc = "0x500..0x50c - Unspecified"]
    pub vregradio: VREGRADIO,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct VREGRADIO {
    #[doc = "0x00 - Request high voltage on RADIO After requesting high voltage, the user must wait until VREQHREADY is set to Ready"]
    pub vreqh: crate::Reg<self::vregradio::vreqh::VREQH_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - High voltage on RADIO is ready"]
    pub vreqhready: crate::Reg<self::vregradio::vreqhready::VREQHREADY_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod vregradio;
