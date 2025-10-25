#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "EPOUT")]
pub struct Epout {
    ptr: Ptr,
    maxcnt: Maxcnt,
    amount: Amount,
}
impl Epout {
    #[doc = "0x00 - Description cluster: Data pointer"]
    #[inline(always)]
    pub const fn ptr(&self) -> &Ptr {
        &self.ptr
    }
    #[doc = "0x04 - Description cluster: Maximum number of bytes to transfer"]
    #[inline(always)]
    pub const fn maxcnt(&self) -> &Maxcnt {
        &self.maxcnt
    }
    #[doc = "0x08 - Description cluster: Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub const fn amount(&self) -> &Amount {
        &self.amount
    }
}
#[doc = "PTR (rw) register accessor: Description cluster: Data pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptr`] module"]
#[doc(alias = "PTR")]
pub type Ptr = crate::Reg<ptr::PtrSpec>;
#[doc = "Description cluster: Data pointer"]
pub mod ptr;
#[doc = "MAXCNT (rw) register accessor: Description cluster: Maximum number of bytes to transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`maxcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxcnt`] module"]
#[doc(alias = "MAXCNT")]
pub type Maxcnt = crate::Reg<maxcnt::MaxcntSpec>;
#[doc = "Description cluster: Maximum number of bytes to transfer"]
pub mod maxcnt;
#[doc = "AMOUNT (r) register accessor: Description cluster: Number of bytes transferred in the last transaction\n\nYou can [`read`](crate::Reg::read) this register and get [`amount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amount`] module"]
#[doc(alias = "AMOUNT")]
pub type Amount = crate::Reg<amount::AmountSpec>;
#[doc = "Description cluster: Number of bytes transferred in the last transaction"]
pub mod amount;
