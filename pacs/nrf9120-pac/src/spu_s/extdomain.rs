#[doc = r"Register block"]
#[repr(C)]
pub struct EXTDOMAIN {
    #[doc = "0x00 - Description cluster: Access for bus access generated from the external domain n List capabilities of the external domain n"]
    pub perm: PERM,
}
#[doc = "PERM (rw) register accessor: an alias for `Reg<PERM_SPEC>`"]
pub type PERM = crate::Reg<perm::PERM_SPEC>;
#[doc = "Description cluster: Access for bus access generated from the external domain n List capabilities of the external domain n"]
pub mod perm;
