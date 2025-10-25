#[doc = "Register `SRAM_DATA_READY` reader"]
pub type R = crate::R<SramDataReadySpec>;
#[doc = "RNG SRAM DMA status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SramReady {
    #[doc = "0: DMA is busy"]
    Busy = 0,
    #[doc = "1: DMA is idle"]
    Idle = 1,
}
impl From<SramReady> for bool {
    #[inline(always)]
    fn from(variant: SramReady) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_READY` reader - RNG SRAM DMA status."]
pub type SramReadyR = crate::BitReader<SramReady>;
impl SramReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SramReady {
        match self.bits {
            false => SramReady::Busy,
            true => SramReady::Idle,
        }
    }
    #[doc = "DMA is busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SramReady::Busy
    }
    #[doc = "DMA is idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SramReady::Idle
    }
}
impl R {
    #[doc = "Bit 0 - RNG SRAM DMA status."]
    #[inline(always)]
    pub fn sram_ready(&self) -> SramReadyR {
        SramReadyR::new((self.bits & 1) != 0)
    }
}
#[doc = "RNG SRAM DMA engine is ready to read/write from/to RNG SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_data_ready::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramDataReadySpec;
impl crate::RegisterSpec for SramDataReadySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_data_ready::R`](R) reader structure"]
impl crate::Readable for SramDataReadySpec {}
#[doc = "`reset()` method sets SRAM_DATA_READY to value 0x01"]
impl crate::Resettable for SramDataReadySpec {
    const RESET_VALUE: u32 = 0x01;
}
