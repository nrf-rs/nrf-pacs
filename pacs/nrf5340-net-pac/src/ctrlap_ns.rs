#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    mailbox: Mailbox,
    _reserved1: [u8; 0x78],
    eraseprotect: Eraseprotect,
    _reserved2: [u8; 0x38],
    approtect: Approtect,
    _reserved3: [u8; 0xb8],
    status: Status,
}
impl RegisterBlock {
    #[doc = "0x400..0x488 - Unspecified"]
    #[inline(always)]
    pub const fn mailbox(&self) -> &Mailbox {
        &self.mailbox
    }
    #[doc = "0x500..0x508 - Unspecified"]
    #[inline(always)]
    pub const fn eraseprotect(&self) -> &Eraseprotect {
        &self.eraseprotect
    }
    #[doc = "0x540..0x548 - Unspecified"]
    #[inline(always)]
    pub const fn approtect(&self) -> &Approtect {
        &self.approtect
    }
    #[doc = "0x600 - Status bits for CTRL-AP peripheral."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
}
#[doc = "Unspecified"]
pub use self::mailbox::Mailbox;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod mailbox;
#[doc = "Unspecified"]
pub use self::eraseprotect::Eraseprotect;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod eraseprotect;
#[doc = "Unspecified"]
pub use self::approtect::Approtect;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod approtect;
#[doc = "STATUS (r) register accessor: Status bits for CTRL-AP peripheral.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status bits for CTRL-AP peripheral."]
pub mod status;
