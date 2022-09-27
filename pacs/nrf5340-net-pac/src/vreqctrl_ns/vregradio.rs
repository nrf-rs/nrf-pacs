#[doc = r"Register block"]
#[repr(C)]
pub struct VREGRADIO {
    #[doc = "0x00 - Request high voltage on RADIO After requesting high voltage, the user must wait until VREQHREADY is set to Ready"]
    pub vreqh: VREQH,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - High voltage on RADIO is ready"]
    pub vreqhready: VREQHREADY,
}
#[doc = "VREQH (rw) register accessor: an alias for `Reg<VREQH_SPEC>`"]
pub type VREQH = crate::Reg<vreqh::VREQH_SPEC>;
#[doc = "Request high voltage on RADIO After requesting high voltage, the user must wait until VREQHREADY is set to Ready"]
pub mod vreqh;
#[doc = "VREQHREADY (r) register accessor: an alias for `Reg<VREQHREADY_SPEC>`"]
pub type VREQHREADY = crate::Reg<vreqhready::VREQHREADY_SPEC>;
#[doc = "High voltage on RADIO is ready"]
pub mod vreqhready;
