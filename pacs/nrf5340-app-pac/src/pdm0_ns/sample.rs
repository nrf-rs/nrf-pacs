#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "SAMPLE")]
pub struct Sample {
    ptr: Ptr,
    maxcnt: Maxcnt,
}
impl Sample {
    #[doc = "0x00 - RAM address pointer to write samples to with EasyDMA"]
    #[inline(always)]
    pub const fn ptr(&self) -> &Ptr {
        &self.ptr
    }
    #[doc = "0x04 - Number of samples to allocate memory for in EasyDMA mode"]
    #[inline(always)]
    pub const fn maxcnt(&self) -> &Maxcnt {
        &self.maxcnt
    }
}
#[doc = "PTR (rw) register accessor: RAM address pointer to write samples to with EasyDMA\n\nYou can [`read`](crate::Reg::read) this register and get [`ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptr`] module"]
#[doc(alias = "PTR")]
pub type Ptr = crate::Reg<ptr::PtrSpec>;
#[doc = "RAM address pointer to write samples to with EasyDMA"]
pub mod ptr;
#[doc = "MAXCNT (rw) register accessor: Number of samples to allocate memory for in EasyDMA mode\n\nYou can [`read`](crate::Reg::read) this register and get [`maxcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxcnt`] module"]
#[doc(alias = "MAXCNT")]
pub type Maxcnt = crate::Reg<maxcnt::MaxcntSpec>;
#[doc = "Number of samples to allocate memory for in EasyDMA mode"]
pub mod maxcnt;
