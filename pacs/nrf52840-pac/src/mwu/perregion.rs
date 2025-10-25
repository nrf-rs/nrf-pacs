#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "PERREGION")]
pub struct Perregion {
    substatwa: Substatwa,
    substatra: Substatra,
}
impl Perregion {
    #[doc = "0x00 - Description cluster: Source of event/interrupt in region n, write access detected while corresponding subregion was enabled for watching"]
    #[inline(always)]
    pub const fn substatwa(&self) -> &Substatwa {
        &self.substatwa
    }
    #[doc = "0x04 - Description cluster: Source of event/interrupt in region n, read access detected while corresponding subregion was enabled for watching"]
    #[inline(always)]
    pub const fn substatra(&self) -> &Substatra {
        &self.substatra
    }
}
#[doc = "SUBSTATWA (rw) register accessor: Description cluster: Source of event/interrupt in region n, write access detected while corresponding subregion was enabled for watching\n\nYou can [`read`](crate::Reg::read) this register and get [`substatwa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`substatwa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@substatwa`] module"]
#[doc(alias = "SUBSTATWA")]
pub type Substatwa = crate::Reg<substatwa::SubstatwaSpec>;
#[doc = "Description cluster: Source of event/interrupt in region n, write access detected while corresponding subregion was enabled for watching"]
pub mod substatwa;
#[doc = "SUBSTATRA (rw) register accessor: Description cluster: Source of event/interrupt in region n, read access detected while corresponding subregion was enabled for watching\n\nYou can [`read`](crate::Reg::read) this register and get [`substatra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`substatra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@substatra`] module"]
#[doc(alias = "SUBSTATRA")]
pub type Substatra = crate::Reg<substatra::SubstatraSpec>;
#[doc = "Description cluster: Source of event/interrupt in region n, read access detected while corresponding subregion was enabled for watching"]
pub mod substatra;
