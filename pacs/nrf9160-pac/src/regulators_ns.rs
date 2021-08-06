#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0500],
    #[doc = "0x500 - System OFF register"]
    pub systemoff: crate::Reg<systemoff::SYSTEMOFF_SPEC>,
    _reserved1: [u8; 0x74],
    #[doc = "0x578 - Enable DC/DC mode of the main voltage regulator."]
    pub dcdcen: crate::Reg<dcdcen::DCDCEN_SPEC>,
}
#[doc = "SYSTEMOFF register accessor: an alias for `Reg<SYSTEMOFF_SPEC>`"]
pub type SYSTEMOFF = crate::Reg<systemoff::SYSTEMOFF_SPEC>;
#[doc = "System OFF register"]
pub mod systemoff;
#[doc = "DCDCEN register accessor: an alias for `Reg<DCDCEN_SPEC>`"]
pub type DCDCEN = crate::Reg<dcdcen::DCDCEN_SPEC>;
#[doc = "Enable DC/DC mode of the main voltage regulator."]
pub mod dcdcen;
