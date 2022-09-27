#[doc = r"Register block"]
#[repr(C)]
pub struct SUBSCRIBE_CHG {
    #[doc = "0x00 - Description cluster: Subscribe configuration for task CHG\\[n\\].EN"]
    pub en: EN,
    #[doc = "0x04 - Description cluster: Subscribe configuration for task CHG\\[n\\].DIS"]
    pub dis: DIS,
}
#[doc = "EN (rw) register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "Description cluster: Subscribe configuration for task CHG\\[n\\].EN"]
pub mod en;
#[doc = "DIS (rw) register accessor: an alias for `Reg<DIS_SPEC>`"]
pub type DIS = crate::Reg<dis::DIS_SPEC>;
#[doc = "Description cluster: Subscribe configuration for task CHG\\[n\\].DIS"]
pub mod dis;
