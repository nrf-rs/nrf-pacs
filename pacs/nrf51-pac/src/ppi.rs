#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - Channel group tasks."]
    pub tasks_chg: [TASKS_CHG; 4],
    _reserved1: [u8; 0x04e0],
    #[doc = "0x500 - Channel enable."]
    pub chen: crate::Reg<chen::CHEN_SPEC>,
    #[doc = "0x504 - Channel enable set."]
    pub chenset: crate::Reg<chenset::CHENSET_SPEC>,
    #[doc = "0x508 - Channel enable clear."]
    pub chenclr: crate::Reg<chenclr::CHENCLR_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x510..0x590 - PPI Channel."]
    pub ch: [CH; 16],
    _reserved5: [u8; 0x0270],
    #[doc = "0x800..0x810 - Channel group configuration."]
    pub chg: [crate::Reg<chg::CHG_SPEC>; 4],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct TASKS_CHG {
    #[doc = "0x00 - Enable channel group."]
    pub en: crate::Reg<self::tasks_chg::en::EN_SPEC>,
    #[doc = "0x04 - Disable channel group."]
    pub dis: crate::Reg<self::tasks_chg::dis::DIS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Channel group tasks."]
pub mod tasks_chg;
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Channel event end-point."]
    pub eep: crate::Reg<self::ch::eep::EEP_SPEC>,
    #[doc = "0x04 - Channel task end-point."]
    pub tep: crate::Reg<self::ch::tep::TEP_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PPI Channel."]
pub mod ch;
#[doc = "CHEN register accessor: an alias for `Reg<CHEN_SPEC>`"]
pub type CHEN = crate::Reg<chen::CHEN_SPEC>;
#[doc = "Channel enable."]
pub mod chen;
#[doc = "CHENSET register accessor: an alias for `Reg<CHENSET_SPEC>`"]
pub type CHENSET = crate::Reg<chenset::CHENSET_SPEC>;
#[doc = "Channel enable set."]
pub mod chenset;
#[doc = "CHENCLR register accessor: an alias for `Reg<CHENCLR_SPEC>`"]
pub type CHENCLR = crate::Reg<chenclr::CHENCLR_SPEC>;
#[doc = "Channel enable clear."]
pub mod chenclr;
#[doc = "CHG register accessor: an alias for `Reg<CHG_SPEC>`"]
pub type CHG = crate::Reg<chg::CHG_SPEC>;
#[doc = "Channel group configuration."]
pub mod chg;
