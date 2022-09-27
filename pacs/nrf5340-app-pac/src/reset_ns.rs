#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    #[doc = "0x400 - Reset reason"]
    pub resetreas: RESETREAS,
    _reserved1: [u8; 0x020c],
    #[doc = "0x610..0x618 - ULP network core control"]
    pub network: NETWORK,
}
#[doc = "RESETREAS (rw) register accessor: an alias for `Reg<RESETREAS_SPEC>`"]
pub type RESETREAS = crate::Reg<resetreas::RESETREAS_SPEC>;
#[doc = "Reset reason"]
pub mod resetreas;
#[doc = "ULP network core control"]
pub use network::NETWORK;
#[doc = r"Cluster"]
#[doc = "ULP network core control"]
pub mod network;
