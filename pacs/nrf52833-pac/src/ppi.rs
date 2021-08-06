#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x30 - Channel group tasks"]
    pub tasks_chg: [TASKS_CHG; 6],
    _reserved1: [u8; 0x04d0],
    #[doc = "0x500 - Channel enable register"]
    pub chen: crate::Reg<chen::CHEN_SPEC>,
    #[doc = "0x504 - Channel enable set register"]
    pub chenset: crate::Reg<chenset::CHENSET_SPEC>,
    #[doc = "0x508 - Channel enable clear register"]
    pub chenclr: crate::Reg<chenclr::CHENCLR_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x510..0x5b0 - PPI Channel"]
    pub ch: [CH; 20],
    _reserved5: [u8; 0x0250],
    #[doc = "0x800..0x818 - Description collection: Channel group n"]
    pub chg: [crate::Reg<chg::CHG_SPEC>; 6],
    _reserved6: [u8; 0xf8],
    #[doc = "0x910..0x990 - Fork"]
    pub fork: [FORK; 32],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct TASKS_CHG {
    #[doc = "0x00 - Description cluster: Enable channel group n"]
    pub en: crate::Reg<self::tasks_chg::en::EN_SPEC>,
    #[doc = "0x04 - Description cluster: Disable channel group n"]
    pub dis: crate::Reg<self::tasks_chg::dis::DIS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Channel group tasks"]
pub mod tasks_chg;
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Description cluster: Channel n event end-point"]
    pub eep: crate::Reg<self::ch::eep::EEP_SPEC>,
    #[doc = "0x04 - Description cluster: Channel n task end-point"]
    pub tep: crate::Reg<self::ch::tep::TEP_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PPI Channel"]
pub mod ch;
#[doc = r"Register block"]
#[repr(C)]
pub struct FORK {
    #[doc = "0x00 - Description cluster: Channel n task end-point"]
    pub tep: crate::Reg<self::fork::tep::TEP_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Fork"]
pub mod fork;
#[doc = "CHEN register accessor: an alias for `Reg<CHEN_SPEC>`"]
pub type CHEN = crate::Reg<chen::CHEN_SPEC>;
#[doc = "Channel enable register"]
pub mod chen;
#[doc = "CHENSET register accessor: an alias for `Reg<CHENSET_SPEC>`"]
pub type CHENSET = crate::Reg<chenset::CHENSET_SPEC>;
#[doc = "Channel enable set register"]
pub mod chenset;
#[doc = "CHENCLR register accessor: an alias for `Reg<CHENCLR_SPEC>`"]
pub type CHENCLR = crate::Reg<chenclr::CHENCLR_SPEC>;
#[doc = "Channel enable clear register"]
pub mod chenclr;
#[doc = "CHG register accessor: an alias for `Reg<CHG_SPEC>`"]
pub type CHG = crate::Reg<chg::CHG_SPEC>;
#[doc = "Description collection: Channel group n"]
pub mod chg;
