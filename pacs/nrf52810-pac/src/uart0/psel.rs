#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "PSEL")]
pub struct Psel {
    rts: Rts,
    txd: Txd,
    cts: Cts,
    rxd: Rxd,
}
impl Psel {
    #[doc = "0x00 - Pin select for RTS"]
    #[inline(always)]
    pub const fn rts(&self) -> &Rts {
        &self.rts
    }
    #[doc = "0x04 - Pin select for TXD"]
    #[inline(always)]
    pub const fn txd(&self) -> &Txd {
        &self.txd
    }
    #[doc = "0x08 - Pin select for CTS"]
    #[inline(always)]
    pub const fn cts(&self) -> &Cts {
        &self.cts
    }
    #[doc = "0x0c - Pin select for RXD"]
    #[inline(always)]
    pub const fn rxd(&self) -> &Rxd {
        &self.rxd
    }
}
#[doc = "RTS (rw) register accessor: Pin select for RTS\n\nYou can [`read`](crate::Reg::read) this register and get [`rts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rts`] module"]
#[doc(alias = "RTS")]
pub type Rts = crate::Reg<rts::RtsSpec>;
#[doc = "Pin select for RTS"]
pub mod rts;
#[doc = "TXD (rw) register accessor: Pin select for TXD\n\nYou can [`read`](crate::Reg::read) this register and get [`txd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txd`] module"]
#[doc(alias = "TXD")]
pub type Txd = crate::Reg<txd::TxdSpec>;
#[doc = "Pin select for TXD"]
pub mod txd;
#[doc = "CTS (rw) register accessor: Pin select for CTS\n\nYou can [`read`](crate::Reg::read) this register and get [`cts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cts`] module"]
#[doc(alias = "CTS")]
pub type Cts = crate::Reg<cts::CtsSpec>;
#[doc = "Pin select for CTS"]
pub mod cts;
#[doc = "RXD (rw) register accessor: Pin select for RXD\n\nYou can [`read`](crate::Reg::read) this register and get [`rxd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxd`] module"]
#[doc(alias = "RXD")]
pub type Rxd = crate::Reg<rxd::RxdSpec>;
#[doc = "Pin select for RXD"]
pub mod rxd;
