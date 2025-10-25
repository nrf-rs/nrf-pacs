#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "RXD")]
pub struct Rxd {
    frameconfig: Frameconfig,
    amount: Amount,
}
impl Rxd {
    #[doc = "0x00 - Configuration of incoming frames"]
    #[inline(always)]
    pub const fn frameconfig(&self) -> &Frameconfig {
        &self.frameconfig
    }
    #[doc = "0x04 - Size of last incoming frame"]
    #[inline(always)]
    pub const fn amount(&self) -> &Amount {
        &self.amount
    }
}
#[doc = "FRAMECONFIG (rw) register accessor: Configuration of incoming frames\n\nYou can [`read`](crate::Reg::read) this register and get [`frameconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frameconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frameconfig`] module"]
#[doc(alias = "FRAMECONFIG")]
pub type Frameconfig = crate::Reg<frameconfig::FrameconfigSpec>;
#[doc = "Configuration of incoming frames"]
pub mod frameconfig;
#[doc = "AMOUNT (r) register accessor: Size of last incoming frame\n\nYou can [`read`](crate::Reg::read) this register and get [`amount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amount`] module"]
#[doc(alias = "AMOUNT")]
pub type Amount = crate::Reg<amount::AmountSpec>;
#[doc = "Size of last incoming frame"]
pub mod amount;
