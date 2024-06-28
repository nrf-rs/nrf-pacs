#[doc = r"Register block"]
#[repr(C)]
pub struct RAMREGION {
    #[doc = "0x00 - Description cluster: Access permissions for RAM region n"]
    pub perm: PERM,
}
#[doc = "PERM (rw) register accessor: an alias for `Reg<PERM_SPEC>`"]
pub type PERM = crate::Reg<perm::PERM_SPEC>;
#[doc = "Description cluster: Access permissions for RAM region n"]
pub mod perm;
