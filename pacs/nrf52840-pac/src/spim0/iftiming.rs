#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "IFTIMING")]
pub struct Iftiming {
    rxdelay: Rxdelay,
    csndur: Csndur,
}
impl Iftiming {
    #[doc = "0x00 - Sample delay for input serial data on MISO"]
    #[inline(always)]
    pub const fn rxdelay(&self) -> &Rxdelay {
        &self.rxdelay
    }
    #[doc = "0x04 - Minimum duration between edge of CSN and edge of SCK at the start and the end of a transaction, and minimum duration CSN will stay high between transactions if END-START shortcut is used"]
    #[inline(always)]
    pub const fn csndur(&self) -> &Csndur {
        &self.csndur
    }
}
#[doc = "RXDELAY (rw) register accessor: Sample delay for input serial data on MISO\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdelay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdelay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdelay`] module"]
#[doc(alias = "RXDELAY")]
pub type Rxdelay = crate::Reg<rxdelay::RxdelaySpec>;
#[doc = "Sample delay for input serial data on MISO"]
pub mod rxdelay;
#[doc = "CSNDUR (rw) register accessor: Minimum duration between edge of CSN and edge of SCK at the start and the end of a transaction, and minimum duration CSN will stay high between transactions if END-START shortcut is used\n\nYou can [`read`](crate::Reg::read) this register and get [`csndur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csndur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csndur`] module"]
#[doc(alias = "CSNDUR")]
pub type Csndur = crate::Reg<csndur::CsndurSpec>;
#[doc = "Minimum duration between edge of CSN and edge of SCK at the start and the end of a transaction, and minimum duration CSN will stay high between transactions if END-START shortcut is used"]
pub mod csndur;
