#[doc = "REGION register accessor: an alias for `Reg<REGION_SPEC>`"]
pub type REGION = crate::Reg<region::REGION_SPEC>;
#[doc = "Description cluster: Define which flash region can contain the non-secure callable (NSC) region n"]
pub mod region;
#[doc = "SIZE register accessor: an alias for `Reg<SIZE_SPEC>`"]
pub type SIZE = crate::Reg<size::SIZE_SPEC>;
#[doc = "Description cluster: Define the size of the non-secure callable (NSC) region n"]
pub mod size;
