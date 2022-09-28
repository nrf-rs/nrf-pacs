#[doc = r"Register block"]
#[repr(C)]
pub struct SECUREAPPROTECT {
    #[doc = "0x00 - This register locks the SECUREAPPROTECT.DISABLE register from being written until next reset."]
    pub lock: LOCK,
    #[doc = "0x04 - This register disables the SECUREAPPROTECT register and enables debug access to secure mode."]
    pub disable: DISABLE,
}
#[doc = "LOCK (rw) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "This register locks the SECUREAPPROTECT.DISABLE register from being written until next reset."]
pub mod lock;
#[doc = "DISABLE (rw) register accessor: an alias for `Reg<DISABLE_SPEC>`"]
pub type DISABLE = crate::Reg<disable::DISABLE_SPEC>;
#[doc = "This register disables the SECUREAPPROTECT register and enables debug access to secure mode."]
pub mod disable;
