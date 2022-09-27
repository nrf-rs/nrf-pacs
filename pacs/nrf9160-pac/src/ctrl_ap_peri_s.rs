#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    #[doc = "0x400..0x488 - Unspecified"]
    pub mailbox: MAILBOX,
    _reserved1: [u8; 0x78],
    #[doc = "0x500..0x508 - Unspecified"]
    pub eraseprotect: ERASEPROTECT,
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
