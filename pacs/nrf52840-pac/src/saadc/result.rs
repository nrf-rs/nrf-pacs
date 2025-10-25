#[repr(C)]
#[doc = "RESULT EasyDMA channel"]
#[doc(alias = "RESULT")]
pub struct Result {
    ptr: Ptr,
    maxcnt: Maxcnt,
    amount: Amount,
}
impl Result {
    #[doc = "0x00 - Data pointer"]
    #[inline(always)]
    pub const fn ptr(&self) -> &Ptr {
        &self.ptr
    }
    #[doc = "0x04 - Maximum number of 16-bit samples to be written to output RAM buffer"]
    #[inline(always)]
    pub const fn maxcnt(&self) -> &Maxcnt {
        &self.maxcnt
    }
    #[doc = "0x08 - Number of 16-bit samples written to output RAM buffer since the previous START task"]
    #[inline(always)]
    pub const fn amount(&self) -> &Amount {
        &self.amount
    }
}
#[doc = "PTR (rw) register accessor: Data pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptr`] module"]
#[doc(alias = "PTR")]
pub type Ptr = crate::Reg<ptr::PtrSpec>;
#[doc = "Data pointer"]
pub mod ptr;
#[doc = "MAXCNT (rw) register accessor: Maximum number of 16-bit samples to be written to output RAM buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`maxcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxcnt`] module"]
#[doc(alias = "MAXCNT")]
pub type Maxcnt = crate::Reg<maxcnt::MaxcntSpec>;
#[doc = "Maximum number of 16-bit samples to be written to output RAM buffer"]
pub mod maxcnt;
#[doc = "AMOUNT (r) register accessor: Number of 16-bit samples written to output RAM buffer since the previous START task\n\nYou can [`read`](crate::Reg::read) this register and get [`amount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amount`] module"]
#[doc(alias = "AMOUNT")]
pub type Amount = crate::Reg<amount::AmountSpec>;
#[doc = "Number of 16-bit samples written to output RAM buffer since the previous START task"]
pub mod amount;
