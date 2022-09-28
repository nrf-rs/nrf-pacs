#[doc = r"Register block"]
#[repr(C)]
pub struct VREGRADIO {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - DC/DC enable register for VREGRADIO"]
    pub dcdcen: DCDCEN,
}
#[doc = "DCDCEN (rw) register accessor: an alias for `Reg<DCDCEN_SPEC>`"]
pub type DCDCEN = crate::Reg<dcdcen::DCDCEN_SPEC>;
#[doc = "DC/DC enable register for VREGRADIO"]
pub mod dcdcen;
