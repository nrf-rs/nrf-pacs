#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Description cluster: Configure the word-aligned start address of region n to protect"]
pub mod addr;
#[doc = "SIZE register accessor: an alias for `Reg<SIZE_SPEC>`"]
pub type SIZE = crate::Reg<size::SIZE_SPEC>;
#[doc = "Description cluster: Size of region to protect counting from address ACL\\[n\\].ADDR. Write '0' as no effect."]
pub mod size;
#[doc = "PERM register accessor: an alias for `Reg<PERM_SPEC>`"]
pub type PERM = crate::Reg<perm::PERM_SPEC>;
#[doc = "Description cluster: Access permissions for region n as defined by start address ACL\\[n\\].ADDR and size ACL\\[n\\].SIZE"]
pub mod perm;
