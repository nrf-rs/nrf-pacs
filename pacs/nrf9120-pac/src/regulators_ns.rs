#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0500],
    #[doc = "0x500 - System OFF register"]
    pub systemoff: SYSTEMOFF,
    _reserved1: [u8; 0x10],
    #[doc = "0x514 - External power failure warning configuration"]
    pub extpofcon: EXTPOFCON,
    _reserved2: [u8; 0x60],
    #[doc = "0x578 - Enable DC/DC mode of the main voltage regulator."]
    pub dcdcen: DCDCEN,
}
#[doc = "SYSTEMOFF (w) register accessor: an alias for `Reg<SYSTEMOFF_SPEC>`"]
pub type SYSTEMOFF = crate::Reg<systemoff::SYSTEMOFF_SPEC>;
#[doc = "System OFF register"]
pub mod systemoff;
#[doc = "EXTPOFCON (rw) register accessor: an alias for `Reg<EXTPOFCON_SPEC>`"]
pub type EXTPOFCON = crate::Reg<extpofcon::EXTPOFCON_SPEC>;
#[doc = "External power failure warning configuration"]
pub mod extpofcon;
#[doc = "DCDCEN (rw) register accessor: an alias for `Reg<DCDCEN_SPEC>`"]
pub type DCDCEN = crate::Reg<dcdcen::DCDCEN_SPEC>;
#[doc = "Enable DC/DC mode of the main voltage regulator."]
pub mod dcdcen;
