#[doc = "PERM register accessor: an alias for `Reg<PERM_SPEC>`"]
pub type PERM = crate::Reg<perm::PERM_SPEC>;
#[doc = "Description cluster: Select between secure and non-secure attribute for the DPPI channels"]
pub mod perm;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Description cluster: Prevent further modification of the corresponding PERM register"]
pub mod lock;
