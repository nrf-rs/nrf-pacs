#[doc = r"Register block"]
#[repr(C)]
pub struct LTEMODEM {
    #[doc = "0x00 - Start LTE modem"]
    pub startn: STARTN,
    #[doc = "0x04 - Force off LTE modem"]
    pub forceoff: FORCEOFF,
}
#[doc = "STARTN (rw) register accessor: an alias for `Reg<STARTN_SPEC>`"]
pub type STARTN = crate::Reg<startn::STARTN_SPEC>;
#[doc = "Start LTE modem"]
pub mod startn;
#[doc = "FORCEOFF (rw) register accessor: an alias for `Reg<FORCEOFF_SPEC>`"]
pub type FORCEOFF = crate::Reg<forceoff::FORCEOFF_SPEC>;
#[doc = "Force off LTE modem"]
pub mod forceoff;
