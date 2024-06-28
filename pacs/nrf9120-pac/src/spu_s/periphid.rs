#[doc = r"Register block"]
#[repr(C)]
pub struct PERIPHID {
    #[doc = "0x00 - Description cluster: List capabilities and access permissions for the peripheral with ID n"]
    pub perm: PERM,
}
#[doc = "PERM (rw) register accessor: an alias for `Reg<PERM_SPEC>`"]
pub type PERM = crate::Reg<perm::PERM_SPEC>;
#[doc = "Description cluster: List capabilities and access permissions for the peripheral with ID n"]
pub mod perm;
