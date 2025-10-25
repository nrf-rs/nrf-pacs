#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0810],
    aes_clk: AesClk,
    _reserved1: [u8; 0x04],
    hash_clk: HashClk,
    pka_clk: PkaClk,
    dma_clk: DmaClk,
    clk_status: ClkStatus,
    _reserved5: [u8; 0x30],
    chacha_clk: ChachaClk,
}
impl RegisterBlock {
    #[doc = "0x810 - Clock control for the AES engine."]
    #[inline(always)]
    pub const fn aes_clk(&self) -> &AesClk {
        &self.aes_clk
    }
    #[doc = "0x818 - Clock control for the HASH engine."]
    #[inline(always)]
    pub const fn hash_clk(&self) -> &HashClk {
        &self.hash_clk
    }
    #[doc = "0x81c - Clock control for the PKA engine."]
    #[inline(always)]
    pub const fn pka_clk(&self) -> &PkaClk {
        &self.pka_clk
    }
    #[doc = "0x820 - Clock control for the DMA engines."]
    #[inline(always)]
    pub const fn dma_clk(&self) -> &DmaClk {
        &self.dma_clk
    }
    #[doc = "0x824 - CRYPTOCELL clocks status register."]
    #[inline(always)]
    pub const fn clk_status(&self) -> &ClkStatus {
        &self.clk_status
    }
    #[doc = "0x858 - Clock control for the CHACHA engine."]
    #[inline(always)]
    pub const fn chacha_clk(&self) -> &ChachaClk {
        &self.chacha_clk
    }
}
#[doc = "AES_CLK (w) register accessor: Clock control for the AES engine.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_clk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_clk`] module"]
#[doc(alias = "AES_CLK")]
pub type AesClk = crate::Reg<aes_clk::AesClkSpec>;
#[doc = "Clock control for the AES engine."]
pub mod aes_clk;
#[doc = "HASH_CLK (w) register accessor: Clock control for the HASH engine.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_clk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_clk`] module"]
#[doc(alias = "HASH_CLK")]
pub type HashClk = crate::Reg<hash_clk::HashClkSpec>;
#[doc = "Clock control for the HASH engine."]
pub mod hash_clk;
#[doc = "PKA_CLK (w) register accessor: Clock control for the PKA engine.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pka_clk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pka_clk`] module"]
#[doc(alias = "PKA_CLK")]
pub type PkaClk = crate::Reg<pka_clk::PkaClkSpec>;
#[doc = "Clock control for the PKA engine."]
pub mod pka_clk;
#[doc = "DMA_CLK (w) register accessor: Clock control for the DMA engines.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_clk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_clk`] module"]
#[doc(alias = "DMA_CLK")]
pub type DmaClk = crate::Reg<dma_clk::DmaClkSpec>;
#[doc = "Clock control for the DMA engines."]
pub mod dma_clk;
#[doc = "CLK_STATUS (r) register accessor: CRYPTOCELL clocks status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_status`] module"]
#[doc(alias = "CLK_STATUS")]
pub type ClkStatus = crate::Reg<clk_status::ClkStatusSpec>;
#[doc = "CRYPTOCELL clocks status register."]
pub mod clk_status;
#[doc = "CHACHA_CLK (w) register accessor: Clock control for the CHACHA engine.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chacha_clk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chacha_clk`] module"]
#[doc(alias = "CHACHA_CLK")]
pub type ChachaClk = crate::Reg<chacha_clk::ChachaClkSpec>;
#[doc = "Clock control for the CHACHA engine."]
pub mod chacha_clk;
