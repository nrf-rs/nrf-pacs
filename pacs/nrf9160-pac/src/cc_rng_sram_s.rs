#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0f00],
    sram_data: SramData,
    sram_addr: SramAddr,
    sram_data_ready: SramDataReady,
}
impl RegisterBlock {
    #[doc = "0xf00 - Read/Write data from RNG SRAM"]
    #[inline(always)]
    pub const fn sram_data(&self) -> &SramData {
        &self.sram_data
    }
    #[doc = "0xf04 - First address given to RNG SRAM DMA for read/write transactions from/to RNG SRAM."]
    #[inline(always)]
    pub const fn sram_addr(&self) -> &SramAddr {
        &self.sram_addr
    }
    #[doc = "0xf08 - RNG SRAM DMA engine is ready to read/write from/to RNG SRAM."]
    #[inline(always)]
    pub const fn sram_data_ready(&self) -> &SramDataReady {
        &self.sram_data_ready
    }
}
#[doc = "SRAM_DATA (rw) register accessor: Read/Write data from RNG SRAM\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_data`] module"]
#[doc(alias = "SRAM_DATA")]
pub type SramData = crate::Reg<sram_data::SramDataSpec>;
#[doc = "Read/Write data from RNG SRAM"]
pub mod sram_data;
#[doc = "SRAM_ADDR (w) register accessor: First address given to RNG SRAM DMA for read/write transactions from/to RNG SRAM.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_addr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_addr`] module"]
#[doc(alias = "SRAM_ADDR")]
pub type SramAddr = crate::Reg<sram_addr::SramAddrSpec>;
#[doc = "First address given to RNG SRAM DMA for read/write transactions from/to RNG SRAM."]
pub mod sram_addr;
#[doc = "SRAM_DATA_READY (r) register accessor: RNG SRAM DMA engine is ready to read/write from/to RNG SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_data_ready::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_data_ready`] module"]
#[doc(alias = "SRAM_DATA_READY")]
pub type SramDataReady = crate::Reg<sram_data_ready::SramDataReadySpec>;
#[doc = "RNG SRAM DMA engine is ready to read/write from/to RNG SRAM."]
pub mod sram_data_ready;
