#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "TXD")]
pub struct Txd {
    ptr: Ptr,
}
impl Txd {
    #[doc = "0x00 - Transmit buffer RAM start address"]
    #[inline(always)]
    pub const fn ptr(&self) -> &Ptr {
        &self.ptr
    }
}
#[doc = "PTR (rw) register accessor: Transmit buffer RAM start address\n\nYou can [`read`](crate::Reg::read) this register and get [`ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptr`] module"]
#[doc(alias = "PTR")]
pub type Ptr = crate::Reg<ptr::PtrSpec>;
#[doc = "Transmit buffer RAM start address"]
pub mod ptr;
