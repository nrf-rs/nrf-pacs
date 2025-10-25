#[doc = "Register `RNG_ISR` reader"]
pub type R = crate::R<RngIsrSpec>;
#[doc = "Field `EHR_VALID_INT` reader - 192-bits have been collected and are ready to be read."]
pub type EhrValidIntR = crate::BitReader;
#[doc = "Field `AUTOCORR_ERR_INT` reader - Autocorrelation error. Failure occurs when autocorrelation test has failed four times in a row. Once set, the TRNG ceases to function until next reset."]
pub type AutocorrErrIntR = crate::BitReader;
#[doc = "Field `CRNGT_ERR_INT` reader - Continuous random number generator test error. Failure occurs when two consecutive blocks of 16 collected bits are equal."]
pub type CrngtErrIntR = crate::BitReader;
#[doc = "Field `VNC_ERR_INT` reader - von Neumann corrector error. Failure occurs if 32 consecutive collected bits are identical, ZERO, or ONE."]
pub type VncErrIntR = crate::BitReader;
#[doc = "Field `WATCHDOG_INT` reader - Maximum number of CPU clock cycles per sample have been exceeded. See RNG_WATCHDOG_VAL for more information."]
pub type WatchdogIntR = crate::BitReader;
#[doc = "Field `DMA_DONE_INT` reader - RNG DMA to SRAM is completed."]
pub type DmaDoneIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 192-bits have been collected and are ready to be read."]
    #[inline(always)]
    pub fn ehr_valid_int(&self) -> EhrValidIntR {
        EhrValidIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autocorrelation error. Failure occurs when autocorrelation test has failed four times in a row. Once set, the TRNG ceases to function until next reset."]
    #[inline(always)]
    pub fn autocorr_err_int(&self) -> AutocorrErrIntR {
        AutocorrErrIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Continuous random number generator test error. Failure occurs when two consecutive blocks of 16 collected bits are equal."]
    #[inline(always)]
    pub fn crngt_err_int(&self) -> CrngtErrIntR {
        CrngtErrIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - von Neumann corrector error. Failure occurs if 32 consecutive collected bits are identical, ZERO, or ONE."]
    #[inline(always)]
    pub fn vnc_err_int(&self) -> VncErrIntR {
        VncErrIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Maximum number of CPU clock cycles per sample have been exceeded. See RNG_WATCHDOG_VAL for more information."]
    #[inline(always)]
    pub fn watchdog_int(&self) -> WatchdogIntR {
        WatchdogIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RNG DMA to SRAM is completed."]
    #[inline(always)]
    pub fn dma_done_int(&self) -> DmaDoneIntR {
        DmaDoneIntR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt status register. Each bit of this register holds the interrupt status of a single interrupt source. If corresponding RNG_IMR bit is unmasked, an interrupt is generated.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngIsrSpec;
impl crate::RegisterSpec for RngIsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_isr::R`](R) reader structure"]
impl crate::Readable for RngIsrSpec {}
#[doc = "`reset()` method sets RNG_ISR to value 0"]
impl crate::Resettable for RngIsrSpec {}
