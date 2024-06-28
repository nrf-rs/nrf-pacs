#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "EXTDOMAIN")]
pub struct Extdomain {
    perm: Perm,
}
impl Extdomain {
    #[doc = "0x00 - Description cluster: Access for bus access generated from the external domain n List capabilities of the external domain n"]
    #[inline(always)]
    pub const fn perm(&self) -> &Perm {
        &self.perm
    }
}
#[doc = "PERM (rw) register accessor: Description cluster: Access for bus access generated from the external domain n List capabilities of the external domain n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perm`]
module"]
#[doc(alias = "PERM")]
pub type Perm = crate::Reg<perm::PermSpec>;
#[doc = "Description cluster: Access for bus access generated from the external domain n List capabilities of the external domain n"]
pub mod perm;
