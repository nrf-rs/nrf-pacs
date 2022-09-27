#[doc = r"Register block"]
#[repr(C)]
pub struct TASKS_CHG {
    #[doc = "0x00 - Description cluster\\[0\\]: Enable channel group 0"]
    pub en: EN,
    #[doc = "0x04 - Description cluster\\[0\\]: Disable channel group 0"]
    pub dis: DIS,
}
#[doc = "EN (w) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "Description cluster\\[0\\]: Enable channel group 0"]
pub mod en;
#[doc = "DIS (w) register accessor: an alias for `Reg<DIS_SPEC>`"]
pub type DIS = crate::Reg<dis::DIS_SPEC>;
#[doc = "Description cluster\\[0\\]: Disable channel group 0"]
pub mod dis;
