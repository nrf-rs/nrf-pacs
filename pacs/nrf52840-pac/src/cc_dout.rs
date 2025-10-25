#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c00],
    dout_buffer: DoutBuffer,
    _reserved1: [u8; 0x011c],
    dout_dma_mem_busy: DoutDmaMemBusy,
    _reserved2: [u8; 0x04],
    dst_mem_addr: DstMemAddr,
    dst_mem_size: DstMemSize,
    dst_sram_addr: DstSramAddr,
    dst_sram_size: DstSramSize,
    dout_dma_sram_busy: DoutDmaSramBusy,
    dout_dma_sram_endianness: DoutDmaSramEndianness,
    _reserved8: [u8; 0x04],
    dout_read_align: DoutReadAlign,
    _reserved9: [u8; 0x08],
    dout_fifo_empty: DoutFifoEmpty,
    _reserved10: [u8; 0x04],
    dout_sw_reset: DoutSwReset,
}
impl RegisterBlock {
    #[doc = "0xc00 - Cryptographic results directly accessible by the CPU."]
    #[inline(always)]
    pub const fn dout_buffer(&self) -> &DoutBuffer {
        &self.dout_buffer
    }
    #[doc = "0xd20 - Status register for DOUT DMA engine activity when accessing memory."]
    #[inline(always)]
    pub const fn dout_dma_mem_busy(&self) -> &DoutDmaMemBusy {
        &self.dout_dma_mem_busy
    }
    #[doc = "0xd28 - Data destination address in memory."]
    #[inline(always)]
    pub const fn dst_mem_addr(&self) -> &DstMemAddr {
        &self.dst_mem_addr
    }
    #[doc = "0xd2c - The number of bytes to be written to memory."]
    #[inline(always)]
    pub const fn dst_mem_size(&self) -> &DstMemSize {
        &self.dst_mem_size
    }
    #[doc = "0xd30 - Data destination address in RNG SRAM."]
    #[inline(always)]
    pub const fn dst_sram_addr(&self) -> &DstSramAddr {
        &self.dst_sram_addr
    }
    #[doc = "0xd34 - The number of bytes to be written to RNG SRAM."]
    #[inline(always)]
    pub const fn dst_sram_size(&self) -> &DstSramSize {
        &self.dst_sram_size
    }
    #[doc = "0xd38 - Status register for DOUT DMA engine activity when accessing RNG SRAM."]
    #[inline(always)]
    pub const fn dout_dma_sram_busy(&self) -> &DoutDmaSramBusy {
        &self.dout_dma_sram_busy
    }
    #[doc = "0xd3c - Configure the endianness of DOUT DMA transactions towards RNG SRAM."]
    #[inline(always)]
    pub const fn dout_dma_sram_endianness(&self) -> &DoutDmaSramEndianness {
        &self.dout_dma_sram_endianness
    }
    #[doc = "0xd44 - Indication that the next CPU read from the DOUT_BUFFER is the last in the sequence. This is needed only when the data size is NOT modulo 4 (e.g. HASH padding)."]
    #[inline(always)]
    pub const fn dout_read_align(&self) -> &DoutReadAlign {
        &self.dout_read_align
    }
    #[doc = "0xd50 - Register indicating if DOUT FIFO is empty or if more data will come."]
    #[inline(always)]
    pub const fn dout_fifo_empty(&self) -> &DoutFifoEmpty {
        &self.dout_fifo_empty
    }
    #[doc = "0xd58 - Reset the DOUT DMA engine."]
    #[inline(always)]
    pub const fn dout_sw_reset(&self) -> &DoutSwReset {
        &self.dout_sw_reset
    }
}
#[doc = "DOUT_BUFFER (r) register accessor: Cryptographic results directly accessible by the CPU.\n\nYou can [`read`](crate::Reg::read) this register and get [`dout_buffer::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout_buffer`] module"]
#[doc(alias = "DOUT_BUFFER")]
pub type DoutBuffer = crate::Reg<dout_buffer::DoutBufferSpec>;
#[doc = "Cryptographic results directly accessible by the CPU."]
pub mod dout_buffer;
#[doc = "DOUT_DMA_MEM_BUSY (r) register accessor: Status register for DOUT DMA engine activity when accessing memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`dout_dma_mem_busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout_dma_mem_busy`] module"]
#[doc(alias = "DOUT_DMA_MEM_BUSY")]
pub type DoutDmaMemBusy = crate::Reg<dout_dma_mem_busy::DoutDmaMemBusySpec>;
#[doc = "Status register for DOUT DMA engine activity when accessing memory."]
pub mod dout_dma_mem_busy;
#[doc = "DST_MEM_ADDR (w) register accessor: Data destination address in memory.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_mem_addr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_mem_addr`] module"]
#[doc(alias = "DST_MEM_ADDR")]
pub type DstMemAddr = crate::Reg<dst_mem_addr::DstMemAddrSpec>;
#[doc = "Data destination address in memory."]
pub mod dst_mem_addr;
#[doc = "DST_MEM_SIZE (w) register accessor: The number of bytes to be written to memory.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_mem_size::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_mem_size`] module"]
#[doc(alias = "DST_MEM_SIZE")]
pub type DstMemSize = crate::Reg<dst_mem_size::DstMemSizeSpec>;
#[doc = "The number of bytes to be written to memory."]
pub mod dst_mem_size;
#[doc = "DST_SRAM_ADDR (rw) register accessor: Data destination address in RNG SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`dst_sram_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_sram_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_sram_addr`] module"]
#[doc(alias = "DST_SRAM_ADDR")]
pub type DstSramAddr = crate::Reg<dst_sram_addr::DstSramAddrSpec>;
#[doc = "Data destination address in RNG SRAM."]
pub mod dst_sram_addr;
#[doc = "DST_SRAM_SIZE (w) register accessor: The number of bytes to be written to RNG SRAM.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_sram_size::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_sram_size`] module"]
#[doc(alias = "DST_SRAM_SIZE")]
pub type DstSramSize = crate::Reg<dst_sram_size::DstSramSizeSpec>;
#[doc = "The number of bytes to be written to RNG SRAM."]
pub mod dst_sram_size;
#[doc = "DOUT_DMA_SRAM_BUSY (r) register accessor: Status register for DOUT DMA engine activity when accessing RNG SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`dout_dma_sram_busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout_dma_sram_busy`] module"]
#[doc(alias = "DOUT_DMA_SRAM_BUSY")]
pub type DoutDmaSramBusy = crate::Reg<dout_dma_sram_busy::DoutDmaSramBusySpec>;
#[doc = "Status register for DOUT DMA engine activity when accessing RNG SRAM."]
pub mod dout_dma_sram_busy;
#[doc = "DOUT_DMA_SRAM_ENDIANNESS (rw) register accessor: Configure the endianness of DOUT DMA transactions towards RNG SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`dout_dma_sram_endianness::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout_dma_sram_endianness::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout_dma_sram_endianness`] module"]
#[doc(alias = "DOUT_DMA_SRAM_ENDIANNESS")]
pub type DoutDmaSramEndianness = crate::Reg<dout_dma_sram_endianness::DoutDmaSramEndiannessSpec>;
#[doc = "Configure the endianness of DOUT DMA transactions towards RNG SRAM."]
pub mod dout_dma_sram_endianness;
#[doc = "DOUT_READ_ALIGN (w) register accessor: Indication that the next CPU read from the DOUT_BUFFER is the last in the sequence. This is needed only when the data size is NOT modulo 4 (e.g. HASH padding).\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout_read_align::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout_read_align`] module"]
#[doc(alias = "DOUT_READ_ALIGN")]
pub type DoutReadAlign = crate::Reg<dout_read_align::DoutReadAlignSpec>;
#[doc = "Indication that the next CPU read from the DOUT_BUFFER is the last in the sequence. This is needed only when the data size is NOT modulo 4 (e.g. HASH padding)."]
pub mod dout_read_align;
#[doc = "DOUT_FIFO_EMPTY (r) register accessor: Register indicating if DOUT FIFO is empty or if more data will come.\n\nYou can [`read`](crate::Reg::read) this register and get [`dout_fifo_empty::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout_fifo_empty`] module"]
#[doc(alias = "DOUT_FIFO_EMPTY")]
pub type DoutFifoEmpty = crate::Reg<dout_fifo_empty::DoutFifoEmptySpec>;
#[doc = "Register indicating if DOUT FIFO is empty or if more data will come."]
pub mod dout_fifo_empty;
#[doc = "DOUT_SW_RESET (w) register accessor: Reset the DOUT DMA engine.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout_sw_reset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout_sw_reset`] module"]
#[doc(alias = "DOUT_SW_RESET")]
pub type DoutSwReset = crate::Reg<dout_sw_reset::DoutSwResetSpec>;
#[doc = "Reset the DOUT DMA engine."]
pub mod dout_sw_reset;
