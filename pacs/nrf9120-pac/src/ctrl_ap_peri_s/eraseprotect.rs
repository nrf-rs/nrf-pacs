#[doc = r"Register block"]
#[repr(C)]
pub struct ERASEPROTECT {
    #[doc = "0x00 - This register locks the ERASEPROTECT.DISABLE register from being written until next reset."]
    pub lock: LOCK,
    #[doc = "0x04 - This register disables the ERASEPROTECT register and performs an ERASEALL operation."]
    pub disable: DISABLE,
}
#[doc = "LOCK (rw) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "This register locks the ERASEPROTECT.DISABLE register from being written until next reset."]
pub mod lock;
#[doc = "DISABLE (rw) register accessor: an alias for `Reg<DISABLE_SPEC>`"]
pub type DISABLE = crate::Reg<disable::DISABLE_SPEC>;
#[doc = "This register disables the ERASEPROTECT register and performs an ERASEALL operation."]
pub mod disable;
