#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "ACL")]
pub struct Acl {
    addr: Addr,
    size: Size,
    perm: Perm,
}
impl Acl {
    #[doc = "0x00 - Description cluster: Start address of region to protect. The start address must be word-aligned."]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x04 - Description cluster: Size of region to protect counting from address ACL\\[n\\].ADDR. Writing a '0' has no effect."]
    #[inline(always)]
    pub const fn size(&self) -> &Size {
        &self.size
    }
    #[doc = "0x08 - Description cluster: Access permissions for region n as defined by start address ACL\\[n\\].ADDR and size ACL\\[n\\].SIZE"]
    #[inline(always)]
    pub const fn perm(&self) -> &Perm {
        &self.perm
    }
}
#[doc = "ADDR (rw) register accessor: Description cluster: Start address of region to protect. The start address must be word-aligned.\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`] module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Description cluster: Start address of region to protect. The start address must be word-aligned."]
pub mod addr;
#[doc = "SIZE (rw) register accessor: Description cluster: Size of region to protect counting from address ACL\\[n\\].ADDR. Writing a '0' has no effect.\n\nYou can [`read`](crate::Reg::read) this register and get [`size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@size`] module"]
#[doc(alias = "SIZE")]
pub type Size = crate::Reg<size::SizeSpec>;
#[doc = "Description cluster: Size of region to protect counting from address ACL\\[n\\].ADDR. Writing a '0' has no effect."]
pub mod size;
#[doc = "PERM (rw) register accessor: Description cluster: Access permissions for region n as defined by start address ACL\\[n\\].ADDR and size ACL\\[n\\].SIZE\n\nYou can [`read`](crate::Reg::read) this register and get [`perm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perm`] module"]
#[doc(alias = "PERM")]
pub type Perm = crate::Reg<perm::PermSpec>;
#[doc = "Description cluster: Access permissions for region n as defined by start address ACL\\[n\\].ADDR and size ACL\\[n\\].SIZE"]
pub mod perm;
