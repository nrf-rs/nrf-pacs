#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "DMA_ENC")]
pub struct DmaEnc {
    key0: Key0,
    key1: Key1,
    key2: Key2,
    key3: Key3,
    nonce0: Nonce0,
    nonce1: Nonce1,
    nonce2: Nonce2,
    enable: Enable,
}
impl DmaEnc {
    #[doc = "0x00 - Bits 31:0 of DMA AES KEY"]
    #[inline(always)]
    pub const fn key0(&self) -> &Key0 {
        &self.key0
    }
    #[doc = "0x04 - Bits 63:32 of DMA AES KEY"]
    #[inline(always)]
    pub const fn key1(&self) -> &Key1 {
        &self.key1
    }
    #[doc = "0x08 - Bits 95:64 of DMA AES KEY"]
    #[inline(always)]
    pub const fn key2(&self) -> &Key2 {
        &self.key2
    }
    #[doc = "0x0c - Bits 127:96 of DMA AES KEY"]
    #[inline(always)]
    pub const fn key3(&self) -> &Key3 {
        &self.key3
    }
    #[doc = "0x10 - Bits 31:0 of DMA NONCE"]
    #[inline(always)]
    pub const fn nonce0(&self) -> &Nonce0 {
        &self.nonce0
    }
    #[doc = "0x14 - Bits 63:32 of DMA NONCE"]
    #[inline(always)]
    pub const fn nonce1(&self) -> &Nonce1 {
        &self.nonce1
    }
    #[doc = "0x18 - Bits 95:64 of DMA NONCE"]
    #[inline(always)]
    pub const fn nonce2(&self) -> &Nonce2 {
        &self.nonce2
    }
    #[doc = "0x1c - Enable stream cipher for EasyDMA"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
}
#[doc = "KEY0 (w) register accessor: Bits 31:0 of DMA AES KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key0`] module"]
#[doc(alias = "KEY0")]
pub type Key0 = crate::Reg<key0::Key0Spec>;
#[doc = "Bits 31:0 of DMA AES KEY"]
pub mod key0;
#[doc = "KEY1 (w) register accessor: Bits 63:32 of DMA AES KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key1`] module"]
#[doc(alias = "KEY1")]
pub type Key1 = crate::Reg<key1::Key1Spec>;
#[doc = "Bits 63:32 of DMA AES KEY"]
pub mod key1;
#[doc = "KEY2 (w) register accessor: Bits 95:64 of DMA AES KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key2`] module"]
#[doc(alias = "KEY2")]
pub type Key2 = crate::Reg<key2::Key2Spec>;
#[doc = "Bits 95:64 of DMA AES KEY"]
pub mod key2;
#[doc = "KEY3 (w) register accessor: Bits 127:96 of DMA AES KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key3`] module"]
#[doc(alias = "KEY3")]
pub type Key3 = crate::Reg<key3::Key3Spec>;
#[doc = "Bits 127:96 of DMA AES KEY"]
pub mod key3;
#[doc = "NONCE0 (w) register accessor: Bits 31:0 of DMA NONCE\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nonce0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nonce0`] module"]
#[doc(alias = "NONCE0")]
pub type Nonce0 = crate::Reg<nonce0::Nonce0Spec>;
#[doc = "Bits 31:0 of DMA NONCE"]
pub mod nonce0;
#[doc = "NONCE1 (w) register accessor: Bits 63:32 of DMA NONCE\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nonce1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nonce1`] module"]
#[doc(alias = "NONCE1")]
pub type Nonce1 = crate::Reg<nonce1::Nonce1Spec>;
#[doc = "Bits 63:32 of DMA NONCE"]
pub mod nonce1;
#[doc = "NONCE2 (w) register accessor: Bits 95:64 of DMA NONCE\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nonce2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nonce2`] module"]
#[doc(alias = "NONCE2")]
pub type Nonce2 = crate::Reg<nonce2::Nonce2Spec>;
#[doc = "Bits 95:64 of DMA NONCE"]
pub mod nonce2;
#[doc = "ENABLE (rw) register accessor: Enable stream cipher for EasyDMA\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable stream cipher for EasyDMA"]
pub mod enable;
