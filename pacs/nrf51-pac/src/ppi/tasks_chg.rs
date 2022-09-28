#[doc = r"Register block"]
#[repr(C)]
pub struct TASKS_CHG {
    #[doc = "0x00 - Enable channel group."]
    pub en: EN,
    #[doc = "0x04 - Disable channel group."]
    pub dis: DIS,
}
#[doc = "EN (w) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "Enable channel group."]
pub mod en;
#[doc = "DIS (w) register accessor: an alias for `Reg<DIS_SPEC>`"]
pub type DIS = crate::Reg<dis::DIS_SPEC>;
#[doc = "Disable channel group."]
pub mod dis;
