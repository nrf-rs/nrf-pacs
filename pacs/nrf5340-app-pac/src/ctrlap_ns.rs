#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    #[doc = "0x400..0x488 - Unspecified"]
    pub mailbox: MAILBOX,
    _reserved1: [u8; 0x78],
    #[doc = "0x500..0x508 - Unspecified"]
    pub eraseprotect: ERASEPROTECT,
    _reserved2: [u8; 0x38],
    #[doc = "0x540..0x548 - Unspecified"]
    pub approtect: APPROTECT,
    #[doc = "0x548..0x550 - Unspecified"]
    pub secureapprotect: SECUREAPPROTECT,
    _reserved4: [u8; 0xb0],
    #[doc = "0x600 - Status bits for CTRL-AP peripheral."]
    pub status: STATUS,
}
#[doc = "Unspecified"]
pub use mailbox::MAILBOX;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod mailbox;
#[doc = "Unspecified"]
pub use eraseprotect::ERASEPROTECT;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod eraseprotect;
#[doc = "Unspecified"]
pub use approtect::APPROTECT;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod approtect;
#[doc = "Unspecified"]
pub use secureapprotect::SECUREAPPROTECT;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod secureapprotect;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status bits for CTRL-AP peripheral."]
pub mod status;
