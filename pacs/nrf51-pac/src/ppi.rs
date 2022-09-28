#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - Channel group tasks."]
    pub tasks_chg: [TASKS_CHG; 4],
    _reserved1: [u8; 0x04e0],
    #[doc = "0x500 - Channel enable."]
    pub chen: CHEN,
    #[doc = "0x504 - Channel enable set."]
    pub chenset: CHENSET,
    #[doc = "0x508 - Channel enable clear."]
    pub chenclr: CHENCLR,
    _reserved4: [u8; 0x04],
    #[doc = "0x510..0x590 - PPI Channel."]
    pub ch: [CH; 16],
    _reserved5: [u8; 0x0270],
    #[doc = "0x800..0x810 - Channel group configuration."]
    pub chg: [CHG; 4],
}
#[doc = "Channel group tasks."]
pub use tasks_chg::TASKS_CHG;
#[doc = r"Cluster"]
#[doc = "Channel group tasks."]
pub mod tasks_chg;
#[doc = "CHEN (rw) register accessor: an alias for `Reg<CHEN_SPEC>`"]
pub type CHEN = crate::Reg<chen::CHEN_SPEC>;
#[doc = "Channel enable."]
pub mod chen;
#[doc = "CHENSET (rw) register accessor: an alias for `Reg<CHENSET_SPEC>`"]
pub type CHENSET = crate::Reg<chenset::CHENSET_SPEC>;
#[doc = "Channel enable set."]
pub mod chenset;
#[doc = "CHENCLR (rw) register accessor: an alias for `Reg<CHENCLR_SPEC>`"]
pub type CHENCLR = crate::Reg<chenclr::CHENCLR_SPEC>;
#[doc = "Channel enable clear."]
pub mod chenclr;
#[doc = "PPI Channel."]
pub use ch::CH;
#[doc = r"Cluster"]
#[doc = "PPI Channel."]
pub mod ch;
#[doc = "CHG (rw) register accessor: an alias for `Reg<CHG_SPEC>`"]
pub type CHG = crate::Reg<chg::CHG_SPEC>;
#[doc = "Channel group configuration."]
pub mod chg;
