#[doc = r"Register block"]
#[repr(C)]
pub struct FLASHNSC {
    #[doc = "0x00 - Description cluster: Define which flash region can contain the non-secure callable (NSC) region n"]
    pub region: REGION,
    #[doc = "0x04 - Description cluster: Define the size of the non-secure callable (NSC) region n"]
    pub size: SIZE,
}
#[doc = "REGION (rw) register accessor: an alias for `Reg<REGION_SPEC>`"]
pub type REGION = crate::Reg<region::REGION_SPEC>;
#[doc = "Description cluster: Define which flash region can contain the non-secure callable (NSC) region n"]
pub mod region;
#[doc = "SIZE (rw) register accessor: an alias for `Reg<SIZE_SPEC>`"]
pub type SIZE = crate::Reg<size::SIZE_SPEC>;
#[doc = "Description cluster: Define the size of the non-secure callable (NSC) region n"]
pub mod size;
