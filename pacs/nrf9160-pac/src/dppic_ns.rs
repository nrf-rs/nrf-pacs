#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x30 - Channel group tasks"]
    pub tasks_chg: [TASKS_CHG; 6],
    _reserved1: [u8; 0x50],
    #[doc = "0x80..0xb0 - Subscribe configuration for tasks"]
    pub subscribe_chg: [SUBSCRIBE_CHG; 6],
    _reserved2: [u8; 0x0450],
    #[doc = "0x500 - Channel enable register"]
    pub chen: crate::Reg<chen::CHEN_SPEC>,
    #[doc = "0x504 - Channel enable set register"]
    pub chenset: crate::Reg<chenset::CHENSET_SPEC>,
    #[doc = "0x508 - Channel enable clear register"]
    pub chenclr: crate::Reg<chenclr::CHENCLR_SPEC>,
    _reserved5: [u8; 0x02f4],
    #[doc = "0x800..0x818 - Description collection: Channel group n Note: Writes to this register is ignored if either SUBSCRIBE_CHG\\[n\\].EN/DIS are enabled."]
    pub chg: [crate::Reg<chg::CHG_SPEC>; 6],
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
pub struct SUBSCRIBE_CHG {
    #[doc = "0x00 - Description cluster: Subscribe configuration for task CHG\\[n\\].EN"]
    pub en: crate::Reg<self::subscribe_chg::en::EN_SPEC>,
    #[doc = "0x04 - Description cluster: Subscribe configuration for task CHG\\[n\\].DIS"]
    pub dis: crate::Reg<self::subscribe_chg::dis::DIS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Subscribe configuration for tasks"]
pub mod subscribe_chg;
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
#[doc = "Description collection: Channel group n Note: Writes to this register is ignored if either SUBSCRIBE_CHG\\[n\\].EN/DIS are enabled."]
pub mod chg;
