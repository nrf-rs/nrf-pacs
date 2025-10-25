#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    rng_imr: RngImr,
    rng_isr: RngIsr,
    rng_icr: RngIcr,
    trng_config: TrngConfig,
    trng_valid: TrngValid,
    ehr_data: [EhrData; 6],
    noise_source: NoiseSource,
    sample_cnt: SampleCnt,
    autocorr_statistic: AutocorrStatistic,
    trng_debug: TrngDebug,
    _reserved10: [u8; 0x04],
    rng_sw_reset: RngSwReset,
    _reserved11: [u8; 0x74],
    rng_busy: RngBusy,
    trng_reset: TrngReset,
    rng_hw_flags: RngHwFlags,
    rng_clk: RngClk,
    rng_dma: RngDma,
    rng_dma_rosc_len: RngDmaRoscLen,
    rng_dma_sram_addr: RngDmaSramAddr,
    rng_dma_samples_num: RngDmaSamplesNum,
    rng_watchdog_val: RngWatchdogVal,
    rng_dma_busy: RngDmaBusy,
}
impl RegisterBlock {
    #[doc = "0x100 - Interrupt mask register. Each bit of this register holds the mask of a single interrupt source."]
    #[inline(always)]
    pub const fn rng_imr(&self) -> &RngImr {
        &self.rng_imr
    }
    #[doc = "0x104 - Interrupt status register. Each bit of this register holds the interrupt status of a single interrupt source. If corresponding RNG_IMR bit is unmasked, an interrupt is generated."]
    #[inline(always)]
    pub const fn rng_isr(&self) -> &RngIsr {
        &self.rng_isr
    }
    #[doc = "0x108 - Interrupt clear register. Writing a 1 bit into a field in this register will clear the corresponding bit in RNG_ISR."]
    #[inline(always)]
    pub const fn rng_icr(&self) -> &RngIcr {
        &self.rng_icr
    }
    #[doc = "0x10c - TRNG ring oscillator length configuration"]
    #[inline(always)]
    pub const fn trng_config(&self) -> &TrngConfig {
        &self.trng_config
    }
    #[doc = "0x110 - This register indicates if TRNG entropy collection is valid."]
    #[inline(always)]
    pub const fn trng_valid(&self) -> &TrngValid {
        &self.trng_valid
    }
    #[doc = "0x114..0x12c - Description collection: The entropy holding registers (EHR) hold 192-bits random data collected by the TRNG. The initial EHR_DATA\\[0\\] register holds the least significant bits \\[31:0\\] of the random data value."]
    #[inline(always)]
    pub const fn ehr_data(&self, n: usize) -> &EhrData {
        &self.ehr_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x114..0x12c - Description collection: The entropy holding registers (EHR) hold 192-bits random data collected by the TRNG. The initial EHR_DATA\\[0\\] register holds the least significant bits \\[31:0\\] of the random data value."]
    #[inline(always)]
    pub fn ehr_data_iter(&self) -> impl Iterator<Item = &EhrData> {
        self.ehr_data.iter()
    }
    #[doc = "0x12c - This register controls the ring oscillator circuit used as a noise source."]
    #[inline(always)]
    pub const fn noise_source(&self) -> &NoiseSource {
        &self.noise_source
    }
    #[doc = "0x130 - Sample count defining the number of CPU clock cycles between two consecutive noise source samples."]
    #[inline(always)]
    pub const fn sample_cnt(&self) -> &SampleCnt {
        &self.sample_cnt
    }
    #[doc = "0x134 - Statistics counter for autocorrelation test activations. Statistics collection is stopped if one of the counters reach its limit of all ones."]
    #[inline(always)]
    pub const fn autocorr_statistic(&self) -> &AutocorrStatistic {
        &self.autocorr_statistic
    }
    #[doc = "0x138 - Debug register for the TRNG. This register is used to bypass TRNG tests in hardware."]
    #[inline(always)]
    pub const fn trng_debug(&self) -> &TrngDebug {
        &self.trng_debug
    }
    #[doc = "0x140 - Reset the RNG engine."]
    #[inline(always)]
    pub const fn rng_sw_reset(&self) -> &RngSwReset {
        &self.rng_sw_reset
    }
    #[doc = "0x1b8 - Status register for RNG engine activity."]
    #[inline(always)]
    pub const fn rng_busy(&self) -> &RngBusy {
        &self.rng_busy
    }
    #[doc = "0x1bc - Reset the TRNG, including internal counter of collected bits and registers EHR_DATA and TRNG_VALID."]
    #[inline(always)]
    pub const fn trng_reset(&self) -> &TrngReset {
        &self.trng_reset
    }
    #[doc = "0x1c0 - Hardware configuration of RNG engine. Reset value holds the supported features."]
    #[inline(always)]
    pub const fn rng_hw_flags(&self) -> &RngHwFlags {
        &self.rng_hw_flags
    }
    #[doc = "0x1c4 - Control clock for the RNG engine."]
    #[inline(always)]
    pub const fn rng_clk(&self) -> &RngClk {
        &self.rng_clk
    }
    #[doc = "0x1c8 - Writing to this register enables the RNG DMA engine."]
    #[inline(always)]
    pub const fn rng_dma(&self) -> &RngDma {
        &self.rng_dma
    }
    #[doc = "0x1cc - This register defines which ring oscillator length configuration should be used when using the RNG DMA engine."]
    #[inline(always)]
    pub const fn rng_dma_rosc_len(&self) -> &RngDmaRoscLen {
        &self.rng_dma_rosc_len
    }
    #[doc = "0x1d0 - This register defines the start address in TRNG SRAM for the TRNG data to be collected by the RNG DMA engine."]
    #[inline(always)]
    pub const fn rng_dma_sram_addr(&self) -> &RngDmaSramAddr {
        &self.rng_dma_sram_addr
    }
    #[doc = "0x1d4 - This register defines the number of 192-bits samples that the RNG DMA engine collects per run."]
    #[inline(always)]
    pub const fn rng_dma_samples_num(&self) -> &RngDmaSamplesNum {
        &self.rng_dma_samples_num
    }
    #[doc = "0x1d8 - This register defines the maximum number of CPU clock cycles per TRNG collection of 192-bits samples. If the number of cycles for a collection exceeds this threshold the WATCHDOG interrupt is triggered."]
    #[inline(always)]
    pub const fn rng_watchdog_val(&self) -> &RngWatchdogVal {
        &self.rng_watchdog_val
    }
    #[doc = "0x1dc - Status register for RNG DMA engine activity."]
    #[inline(always)]
    pub const fn rng_dma_busy(&self) -> &RngDmaBusy {
        &self.rng_dma_busy
    }
}
#[doc = "RNG_IMR (rw) register accessor: Interrupt mask register. Each bit of this register holds the mask of a single interrupt source.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_imr`] module"]
#[doc(alias = "RNG_IMR")]
pub type RngImr = crate::Reg<rng_imr::RngImrSpec>;
#[doc = "Interrupt mask register. Each bit of this register holds the mask of a single interrupt source."]
pub mod rng_imr;
#[doc = "RNG_ISR (r) register accessor: Interrupt status register. Each bit of this register holds the interrupt status of a single interrupt source. If corresponding RNG_IMR bit is unmasked, an interrupt is generated.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_isr`] module"]
#[doc(alias = "RNG_ISR")]
pub type RngIsr = crate::Reg<rng_isr::RngIsrSpec>;
#[doc = "Interrupt status register. Each bit of this register holds the interrupt status of a single interrupt source. If corresponding RNG_IMR bit is unmasked, an interrupt is generated."]
pub mod rng_isr;
#[doc = "RNG_ICR (w) register accessor: Interrupt clear register. Writing a 1 bit into a field in this register will clear the corresponding bit in RNG_ISR.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_icr`] module"]
#[doc(alias = "RNG_ICR")]
pub type RngIcr = crate::Reg<rng_icr::RngIcrSpec>;
#[doc = "Interrupt clear register. Writing a 1 bit into a field in this register will clear the corresponding bit in RNG_ISR."]
pub mod rng_icr;
#[doc = "TRNG_CONFIG (rw) register accessor: TRNG ring oscillator length configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_config`] module"]
#[doc(alias = "TRNG_CONFIG")]
pub type TrngConfig = crate::Reg<trng_config::TrngConfigSpec>;
#[doc = "TRNG ring oscillator length configuration"]
pub mod trng_config;
#[doc = "TRNG_VALID (r) register accessor: This register indicates if TRNG entropy collection is valid.\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_valid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_valid`] module"]
#[doc(alias = "TRNG_VALID")]
pub type TrngValid = crate::Reg<trng_valid::TrngValidSpec>;
#[doc = "This register indicates if TRNG entropy collection is valid."]
pub mod trng_valid;
#[doc = "EHR_DATA (r) register accessor: Description collection: The entropy holding registers (EHR) hold 192-bits random data collected by the TRNG. The initial EHR_DATA\\[0\\] register holds the least significant bits \\[31:0\\] of the random data value.\n\nYou can [`read`](crate::Reg::read) this register and get [`ehr_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehr_data`] module"]
#[doc(alias = "EHR_DATA")]
pub type EhrData = crate::Reg<ehr_data::EhrDataSpec>;
#[doc = "Description collection: The entropy holding registers (EHR) hold 192-bits random data collected by the TRNG. The initial EHR_DATA\\[0\\] register holds the least significant bits \\[31:0\\] of the random data value."]
pub mod ehr_data;
#[doc = "NOISE_SOURCE (rw) register accessor: This register controls the ring oscillator circuit used as a noise source.\n\nYou can [`read`](crate::Reg::read) this register and get [`noise_source::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`noise_source::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@noise_source`] module"]
#[doc(alias = "NOISE_SOURCE")]
pub type NoiseSource = crate::Reg<noise_source::NoiseSourceSpec>;
#[doc = "This register controls the ring oscillator circuit used as a noise source."]
pub mod noise_source;
#[doc = "SAMPLE_CNT (rw) register accessor: Sample count defining the number of CPU clock cycles between two consecutive noise source samples.\n\nYou can [`read`](crate::Reg::read) this register and get [`sample_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sample_cnt`] module"]
#[doc(alias = "SAMPLE_CNT")]
pub type SampleCnt = crate::Reg<sample_cnt::SampleCntSpec>;
#[doc = "Sample count defining the number of CPU clock cycles between two consecutive noise source samples."]
pub mod sample_cnt;
#[doc = "AUTOCORR_STATISTIC (rw) register accessor: Statistics counter for autocorrelation test activations. Statistics collection is stopped if one of the counters reach its limit of all ones.\n\nYou can [`read`](crate::Reg::read) this register and get [`autocorr_statistic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocorr_statistic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autocorr_statistic`] module"]
#[doc(alias = "AUTOCORR_STATISTIC")]
pub type AutocorrStatistic = crate::Reg<autocorr_statistic::AutocorrStatisticSpec>;
#[doc = "Statistics counter for autocorrelation test activations. Statistics collection is stopped if one of the counters reach its limit of all ones."]
pub mod autocorr_statistic;
#[doc = "TRNG_DEBUG (rw) register accessor: Debug register for the TRNG. This register is used to bypass TRNG tests in hardware.\n\nYou can [`read`](crate::Reg::read) this register and get [`trng_debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_debug`] module"]
#[doc(alias = "TRNG_DEBUG")]
pub type TrngDebug = crate::Reg<trng_debug::TrngDebugSpec>;
#[doc = "Debug register for the TRNG. This register is used to bypass TRNG tests in hardware."]
pub mod trng_debug;
#[doc = "RNG_SW_RESET (w) register accessor: Reset the RNG engine.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_sw_reset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_sw_reset`] module"]
#[doc(alias = "RNG_SW_RESET")]
pub type RngSwReset = crate::Reg<rng_sw_reset::RngSwResetSpec>;
#[doc = "Reset the RNG engine."]
pub mod rng_sw_reset;
#[doc = "RNG_BUSY (r) register accessor: Status register for RNG engine activity.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_busy`] module"]
#[doc(alias = "RNG_BUSY")]
pub type RngBusy = crate::Reg<rng_busy::RngBusySpec>;
#[doc = "Status register for RNG engine activity."]
pub mod rng_busy;
#[doc = "TRNG_RESET (w) register accessor: Reset the TRNG, including internal counter of collected bits and registers EHR_DATA and TRNG_VALID.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trng_reset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trng_reset`] module"]
#[doc(alias = "TRNG_RESET")]
pub type TrngReset = crate::Reg<trng_reset::TrngResetSpec>;
#[doc = "Reset the TRNG, including internal counter of collected bits and registers EHR_DATA and TRNG_VALID."]
pub mod trng_reset;
#[doc = "RNG_HW_FLAGS (r) register accessor: Hardware configuration of RNG engine. Reset value holds the supported features.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_hw_flags::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_hw_flags`] module"]
#[doc(alias = "RNG_HW_FLAGS")]
pub type RngHwFlags = crate::Reg<rng_hw_flags::RngHwFlagsSpec>;
#[doc = "Hardware configuration of RNG engine. Reset value holds the supported features."]
pub mod rng_hw_flags;
#[doc = "RNG_CLK (w) register accessor: Control clock for the RNG engine.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_clk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_clk`] module"]
#[doc(alias = "RNG_CLK")]
pub type RngClk = crate::Reg<rng_clk::RngClkSpec>;
#[doc = "Control clock for the RNG engine."]
pub mod rng_clk;
#[doc = "RNG_DMA (rw) register accessor: Writing to this register enables the RNG DMA engine.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_dma`] module"]
#[doc(alias = "RNG_DMA")]
pub type RngDma = crate::Reg<rng_dma::RngDmaSpec>;
#[doc = "Writing to this register enables the RNG DMA engine."]
pub mod rng_dma;
#[doc = "RNG_DMA_ROSC_LEN (rw) register accessor: This register defines which ring oscillator length configuration should be used when using the RNG DMA engine.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_dma_rosc_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_dma_rosc_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_dma_rosc_len`] module"]
#[doc(alias = "RNG_DMA_ROSC_LEN")]
pub type RngDmaRoscLen = crate::Reg<rng_dma_rosc_len::RngDmaRoscLenSpec>;
#[doc = "This register defines which ring oscillator length configuration should be used when using the RNG DMA engine."]
pub mod rng_dma_rosc_len;
#[doc = "RNG_DMA_SRAM_ADDR (rw) register accessor: This register defines the start address in TRNG SRAM for the TRNG data to be collected by the RNG DMA engine.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_dma_sram_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_dma_sram_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_dma_sram_addr`] module"]
#[doc(alias = "RNG_DMA_SRAM_ADDR")]
pub type RngDmaSramAddr = crate::Reg<rng_dma_sram_addr::RngDmaSramAddrSpec>;
#[doc = "This register defines the start address in TRNG SRAM for the TRNG data to be collected by the RNG DMA engine."]
pub mod rng_dma_sram_addr;
#[doc = "RNG_DMA_SAMPLES_NUM (rw) register accessor: This register defines the number of 192-bits samples that the RNG DMA engine collects per run.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_dma_samples_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_dma_samples_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_dma_samples_num`] module"]
#[doc(alias = "RNG_DMA_SAMPLES_NUM")]
pub type RngDmaSamplesNum = crate::Reg<rng_dma_samples_num::RngDmaSamplesNumSpec>;
#[doc = "This register defines the number of 192-bits samples that the RNG DMA engine collects per run."]
pub mod rng_dma_samples_num;
#[doc = "RNG_WATCHDOG_VAL (rw) register accessor: This register defines the maximum number of CPU clock cycles per TRNG collection of 192-bits samples. If the number of cycles for a collection exceeds this threshold the WATCHDOG interrupt is triggered.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_watchdog_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_watchdog_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_watchdog_val`] module"]
#[doc(alias = "RNG_WATCHDOG_VAL")]
pub type RngWatchdogVal = crate::Reg<rng_watchdog_val::RngWatchdogValSpec>;
#[doc = "This register defines the maximum number of CPU clock cycles per TRNG collection of 192-bits samples. If the number of cycles for a collection exceeds this threshold the WATCHDOG interrupt is triggered."]
pub mod rng_watchdog_val;
#[doc = "RNG_DMA_BUSY (r) register accessor: Status register for RNG DMA engine activity.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_dma_busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_dma_busy`] module"]
#[doc(alias = "RNG_DMA_BUSY")]
pub type RngDmaBusy = crate::Reg<rng_dma_busy::RngDmaBusySpec>;
#[doc = "Status register for RNG DMA engine activity."]
pub mod rng_dma_busy;
