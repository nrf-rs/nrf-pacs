#[doc = r"Register block"]
#[repr(C)]
pub struct NETWORK {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Force network core off"]
    pub forceoff: FORCEOFF,
}
#[doc = "FORCEOFF (rw) register accessor: an alias for `Reg<FORCEOFF_SPEC>`"]
pub type FORCEOFF = crate::Reg<forceoff::FORCEOFF_SPEC>;
#[doc = "Force network core off"]
pub mod forceoff;
