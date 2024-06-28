#[repr(C)]
#[doc = "TXD EasyDMA channel"]
#[doc(alias = "TXD")]
pub struct Txd {
    ptr: Ptr,
    maxcnt: Maxcnt,
    amount: Amount,
}
impl Txd {
    #[doc = "0x00 - Data pointer"]
    #[inline(always)]
    pub const fn ptr(&self) -> &Ptr {
        &self.ptr
    }
    #[doc = "0x04 - Maximum number of bytes in transmit buffer"]
    #[inline(always)]
    pub const fn maxcnt(&self) -> &Maxcnt {
        &self.maxcnt
    }
    #[doc = "0x08 - Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub const fn amount(&self) -> &Amount {
        &self.amount
    }
}
#[doc = "PTR (rw) register accessor: Data pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptr`]
module"]
#[doc(alias = "PTR")]
pub type Ptr = crate::Reg<ptr::PtrSpec>;
#[doc = "Data pointer"]
pub mod ptr;
#[doc = "MAXCNT (rw) register accessor: Maximum number of bytes in transmit buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxcnt`]
module"]
#[doc(alias = "MAXCNT")]
pub type Maxcnt = crate::Reg<maxcnt::MaxcntSpec>;
#[doc = "Maximum number of bytes in transmit buffer"]
pub mod maxcnt;
#[doc = "AMOUNT (r) register accessor: Number of bytes transferred in the last transaction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amount::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amount`]
module"]
#[doc(alias = "AMOUNT")]
pub type Amount = crate::Reg<amount::AmountSpec>;
#[doc = "Number of bytes transferred in the last transaction"]
pub mod amount;
