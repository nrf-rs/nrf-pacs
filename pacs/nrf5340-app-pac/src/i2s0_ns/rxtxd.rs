#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "RXTXD")]
pub struct Rxtxd {
    maxcnt: Maxcnt,
}
impl Rxtxd {
    #[doc = "0x00 - Size of RXD and TXD buffers"]
    #[inline(always)]
    pub const fn maxcnt(&self) -> &Maxcnt {
        &self.maxcnt
    }
}
#[doc = "MAXCNT (rw) register accessor: Size of RXD and TXD buffers\n\nYou can [`read`](crate::Reg::read) this register and get [`maxcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxcnt`] module"]
#[doc(alias = "MAXCNT")]
pub type Maxcnt = crate::Reg<maxcnt::MaxcntSpec>;
#[doc = "Size of RXD and TXD buffers"]
pub mod maxcnt;
