#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0428],
    #[doc = "0x428 - Main supply status"]
    pub mainregstatus: MAINREGSTATUS,
    _reserved1: [u8; 0xd4],
    #[doc = "0x500 - System OFF register"]
    pub systemoff: SYSTEMOFF,
    _reserved2: [u8; 0x0c],
    #[doc = "0x510 - Power-fail comparator configuration"]
    pub pofcon: POFCON,
    _reserved3: [u8; 0x01f0],
    #[doc = "0x704 - Unspecified"]
    pub vregmain: VREGMAIN,
    _reserved4: [u8; 0x01f8],
    #[doc = "0x900..0x908 - Unspecified"]
    pub vregradio: VREGRADIO,
    _reserved5: [u8; 0x01f8],
    #[doc = "0xb00 - Unspecified"]
    pub vregh: VREGH,
}
#[doc = "MAINREGSTATUS (r) register accessor: an alias for `Reg<MAINREGSTATUS_SPEC>`"]
pub type MAINREGSTATUS = crate::Reg<mainregstatus::MAINREGSTATUS_SPEC>;
#[doc = "Main supply status"]
pub mod mainregstatus;
#[doc = "SYSTEMOFF (w) register accessor: an alias for `Reg<SYSTEMOFF_SPEC>`"]
pub type SYSTEMOFF = crate::Reg<systemoff::SYSTEMOFF_SPEC>;
#[doc = "System OFF register"]
pub mod systemoff;
#[doc = "POFCON (rw) register accessor: an alias for `Reg<POFCON_SPEC>`"]
pub type POFCON = crate::Reg<pofcon::POFCON_SPEC>;
#[doc = "Power-fail comparator configuration"]
pub mod pofcon;
#[doc = "Unspecified"]
pub use vregmain::VREGMAIN;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod vregmain;
#[doc = "Unspecified"]
pub use vregradio::VREGRADIO;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod vregradio;
#[doc = "Unspecified"]
pub use vregh::VREGH;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod vregh;
