#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c00],
    din_buffer: DinBuffer,
    _reserved1: [u8; 0x1c],
    din_dma_mem_busy: DinDmaMemBusy,
    _reserved2: [u8; 0x04],
    src_mem_addr: SrcMemAddr,
    src_mem_size: SrcMemSize,
    src_sram_addr: SrcSramAddr,
    src_sram_size: SrcSramSize,
    din_dma_sram_busy: DinDmaSramBusy,
    din_dma_sram_endianness: DinDmaSramEndianness,
    _reserved8: [u8; 0x04],
    din_sw_reset: DinSwReset,
    din_cpu_data: DinCpuData,
    din_write_align: DinWriteAlign,
    din_fifo_empty: DinFifoEmpty,
    _reserved12: [u8; 0x04],
    din_fifo_reset: DinFifoReset,
}
impl RegisterBlock {
    #[doc = "0xc00 - Used by CPU to write data directly to the DIN buffer, which is then sent to the cryptographic engines for processing."]
    #[inline(always)]
    pub const fn din_buffer(&self) -> &DinBuffer {
        &self.din_buffer
    }
    #[doc = "0xc20 - Status register for DIN DMA engine activity when accessing memory."]
    #[inline(always)]
    pub const fn din_dma_mem_busy(&self) -> &DinDmaMemBusy {
        &self.din_dma_mem_busy
    }
    #[doc = "0xc28 - Data source address in memory."]
    #[inline(always)]
    pub const fn src_mem_addr(&self) -> &SrcMemAddr {
        &self.src_mem_addr
    }
    #[doc = "0xc2c - The number of bytes to be read from memory. Writing to this register triggers the DMA operation."]
    #[inline(always)]
    pub const fn src_mem_size(&self) -> &SrcMemSize {
        &self.src_mem_size
    }
    #[doc = "0xc30 - Data source address in RNG SRAM."]
    #[inline(always)]
    pub const fn src_sram_addr(&self) -> &SrcSramAddr {
        &self.src_sram_addr
    }
    #[doc = "0xc34 - The number of bytes to be read from RNG SRAM. Writing to this register triggers the DMA operation."]
    #[inline(always)]
    pub const fn src_sram_size(&self) -> &SrcSramSize {
        &self.src_sram_size
    }
    #[doc = "0xc38 - Status register for DIN DMA engine activity when accessing RNG SRAM."]
    #[inline(always)]
    pub const fn din_dma_sram_busy(&self) -> &DinDmaSramBusy {
        &self.din_dma_sram_busy
    }
    #[doc = "0xc3c - Configure the endianness of DIN DMA transactions towards RNG SRAM."]
    #[inline(always)]
    pub const fn din_dma_sram_endianness(&self) -> &DinDmaSramEndianness {
        &self.din_dma_sram_endianness
    }
    #[doc = "0xc44 - Reset the DIN DMA engine."]
    #[inline(always)]
    pub const fn din_sw_reset(&self) -> &DinSwReset {
        &self.din_sw_reset
    }
    #[doc = "0xc48 - Specifies the number of bytes the CPU will write to the DIN_BUFFER, ensuring the cryptographic engine processes the correct amount of data."]
    #[inline(always)]
    pub const fn din_cpu_data(&self) -> &DinCpuData {
        &self.din_cpu_data
    }
    #[doc = "0xc4c - Indicates that the next CPU write to the DIN_BUFFER is the last in the sequence. This is needed only when the data size is NOT modulo 4 (e.g. HASH padding)."]
    #[inline(always)]
    pub const fn din_write_align(&self) -> &DinWriteAlign {
        &self.din_write_align
    }
    #[doc = "0xc50 - Register indicating if DIN FIFO is empty and if more data can be accepted."]
    #[inline(always)]
    pub const fn din_fifo_empty(&self) -> &DinFifoEmpty {
        &self.din_fifo_empty
    }
    #[doc = "0xc58 - Reset the DIN FIFO, effectively clearing the FIFO for new data."]
    #[inline(always)]
    pub const fn din_fifo_reset(&self) -> &DinFifoReset {
        &self.din_fifo_reset
    }
}
#[doc = "DIN_BUFFER (w) register accessor: Used by CPU to write data directly to the DIN buffer, which is then sent to the cryptographic engines for processing.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_buffer::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_buffer`] module"]
#[doc(alias = "DIN_BUFFER")]
pub type DinBuffer = crate::Reg<din_buffer::DinBufferSpec>;
#[doc = "Used by CPU to write data directly to the DIN buffer, which is then sent to the cryptographic engines for processing."]
pub mod din_buffer;
#[doc = "DIN_DMA_MEM_BUSY (r) register accessor: Status register for DIN DMA engine activity when accessing memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`din_dma_mem_busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_dma_mem_busy`] module"]
#[doc(alias = "DIN_DMA_MEM_BUSY")]
pub type DinDmaMemBusy = crate::Reg<din_dma_mem_busy::DinDmaMemBusySpec>;
#[doc = "Status register for DIN DMA engine activity when accessing memory."]
pub mod din_dma_mem_busy;
#[doc = "SRC_MEM_ADDR (w) register accessor: Data source address in memory.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_mem_addr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_mem_addr`] module"]
#[doc(alias = "SRC_MEM_ADDR")]
pub type SrcMemAddr = crate::Reg<src_mem_addr::SrcMemAddrSpec>;
#[doc = "Data source address in memory."]
pub mod src_mem_addr;
#[doc = "SRC_MEM_SIZE (w) register accessor: The number of bytes to be read from memory. Writing to this register triggers the DMA operation.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_mem_size::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_mem_size`] module"]
#[doc(alias = "SRC_MEM_SIZE")]
pub type SrcMemSize = crate::Reg<src_mem_size::SrcMemSizeSpec>;
#[doc = "The number of bytes to be read from memory. Writing to this register triggers the DMA operation."]
pub mod src_mem_size;
#[doc = "SRC_SRAM_ADDR (rw) register accessor: Data source address in RNG SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`src_sram_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_sram_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_sram_addr`] module"]
#[doc(alias = "SRC_SRAM_ADDR")]
pub type SrcSramAddr = crate::Reg<src_sram_addr::SrcSramAddrSpec>;
#[doc = "Data source address in RNG SRAM."]
pub mod src_sram_addr;
#[doc = "SRC_SRAM_SIZE (w) register accessor: The number of bytes to be read from RNG SRAM. Writing to this register triggers the DMA operation.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_sram_size::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_sram_size`] module"]
#[doc(alias = "SRC_SRAM_SIZE")]
pub type SrcSramSize = crate::Reg<src_sram_size::SrcSramSizeSpec>;
#[doc = "The number of bytes to be read from RNG SRAM. Writing to this register triggers the DMA operation."]
pub mod src_sram_size;
#[doc = "DIN_DMA_SRAM_BUSY (r) register accessor: Status register for DIN DMA engine activity when accessing RNG SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`din_dma_sram_busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_dma_sram_busy`] module"]
#[doc(alias = "DIN_DMA_SRAM_BUSY")]
pub type DinDmaSramBusy = crate::Reg<din_dma_sram_busy::DinDmaSramBusySpec>;
#[doc = "Status register for DIN DMA engine activity when accessing RNG SRAM."]
pub mod din_dma_sram_busy;
#[doc = "DIN_DMA_SRAM_ENDIANNESS (rw) register accessor: Configure the endianness of DIN DMA transactions towards RNG SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`din_dma_sram_endianness::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_dma_sram_endianness::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_dma_sram_endianness`] module"]
#[doc(alias = "DIN_DMA_SRAM_ENDIANNESS")]
pub type DinDmaSramEndianness = crate::Reg<din_dma_sram_endianness::DinDmaSramEndiannessSpec>;
#[doc = "Configure the endianness of DIN DMA transactions towards RNG SRAM."]
pub mod din_dma_sram_endianness;
#[doc = "DIN_SW_RESET (w) register accessor: Reset the DIN DMA engine.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_sw_reset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_sw_reset`] module"]
#[doc(alias = "DIN_SW_RESET")]
pub type DinSwReset = crate::Reg<din_sw_reset::DinSwResetSpec>;
#[doc = "Reset the DIN DMA engine."]
pub mod din_sw_reset;
#[doc = "DIN_CPU_DATA (w) register accessor: Specifies the number of bytes the CPU will write to the DIN_BUFFER, ensuring the cryptographic engine processes the correct amount of data.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_cpu_data::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_cpu_data`] module"]
#[doc(alias = "DIN_CPU_DATA")]
pub type DinCpuData = crate::Reg<din_cpu_data::DinCpuDataSpec>;
#[doc = "Specifies the number of bytes the CPU will write to the DIN_BUFFER, ensuring the cryptographic engine processes the correct amount of data."]
pub mod din_cpu_data;
#[doc = "DIN_WRITE_ALIGN (w) register accessor: Indicates that the next CPU write to the DIN_BUFFER is the last in the sequence. This is needed only when the data size is NOT modulo 4 (e.g. HASH padding).\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_write_align::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_write_align`] module"]
#[doc(alias = "DIN_WRITE_ALIGN")]
pub type DinWriteAlign = crate::Reg<din_write_align::DinWriteAlignSpec>;
#[doc = "Indicates that the next CPU write to the DIN_BUFFER is the last in the sequence. This is needed only when the data size is NOT modulo 4 (e.g. HASH padding)."]
pub mod din_write_align;
#[doc = "DIN_FIFO_EMPTY (r) register accessor: Register indicating if DIN FIFO is empty and if more data can be accepted.\n\nYou can [`read`](crate::Reg::read) this register and get [`din_fifo_empty::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_fifo_empty`] module"]
#[doc(alias = "DIN_FIFO_EMPTY")]
pub type DinFifoEmpty = crate::Reg<din_fifo_empty::DinFifoEmptySpec>;
#[doc = "Register indicating if DIN FIFO is empty and if more data can be accepted."]
pub mod din_fifo_empty;
#[doc = "DIN_FIFO_RESET (w) register accessor: Reset the DIN FIFO, effectively clearing the FIFO for new data.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`din_fifo_reset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din_fifo_reset`] module"]
#[doc(alias = "DIN_FIFO_RESET")]
pub type DinFifoReset = crate::Reg<din_fifo_reset::DinFifoResetSpec>;
#[doc = "Reset the DIN FIFO, effectively clearing the FIFO for new data."]
pub mod din_fifo_reset;
