#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "DPPI")]
pub struct Dppi {
    perm: Perm,
    lock: Lock,
}
impl Dppi {
    #[doc = "0x00 - Description cluster: Select between secure and non-secure attribute for the DPPI channels."]
    #[inline(always)]
    pub const fn perm(&self) -> &Perm {
        &self.perm
    }
    #[doc = "0x04 - Description cluster: Prevent further modification of the corresponding PERM register"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
}
#[doc = "PERM (rw) register accessor: Description cluster: Select between secure and non-secure attribute for the DPPI channels.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perm`]
module"]
#[doc(alias = "PERM")]
pub type Perm = crate::Reg<perm::PermSpec>;
#[doc = "Description cluster: Select between secure and non-secure attribute for the DPPI channels."]
pub mod perm;
#[doc = "LOCK (rw) register accessor: Description cluster: Prevent further modification of the corresponding PERM register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "Description cluster: Prevent further modification of the corresponding PERM register"]
pub mod lock;
