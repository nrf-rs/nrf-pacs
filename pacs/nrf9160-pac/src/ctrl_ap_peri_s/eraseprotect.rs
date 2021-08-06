#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Lock register ERASEPROTECT.DISABLE from being written until next reset"]
pub mod lock;
#[doc = "DISABLE register accessor: an alias for `Reg<DISABLE_SPEC>`"]
pub type DISABLE = crate::Reg<disable::DISABLE_SPEC>;
#[doc = "Disable ERASEPROTECT and perform ERASEALL"]
pub mod disable;
