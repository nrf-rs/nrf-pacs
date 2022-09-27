#[doc = r"Register block"]
#[repr(C)]
pub struct ACL {
    #[doc = "0x00 - Description cluster: Start address of region to protect. The start address must be word-aligned."]
    pub addr: ADDR,
    #[doc = "0x04 - Description cluster: Size of region to protect counting from address ACL\\[n\\].ADDR. Writing a '0' has no effect."]
    pub size: SIZE,
    #[doc = "0x08 - Description cluster: Access permissions for region n as defined by start address ACL\\[n\\].ADDR and size ACL\\[n\\].SIZE"]
    pub perm: PERM,
}
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Description cluster: Start address of region to protect. The start address must be word-aligned."]
pub mod addr;
#[doc = "SIZE (rw) register accessor: an alias for `Reg<SIZE_SPEC>`"]
pub type SIZE = crate::Reg<size::SIZE_SPEC>;
#[doc = "Description cluster: Size of region to protect counting from address ACL\\[n\\].ADDR. Writing a '0' has no effect."]
pub mod size;
#[doc = "PERM (rw) register accessor: an alias for `Reg<PERM_SPEC>`"]
pub type PERM = crate::Reg<perm::PERM_SPEC>;
#[doc = "Description cluster: Access permissions for region n as defined by start address ACL\\[n\\].ADDR and size ACL\\[n\\].SIZE"]
pub mod perm;
