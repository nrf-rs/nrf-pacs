#[doc = "Register `DOUT_DMA_SRAM_BUSY` reader"]
pub type R = crate::R<DoutDmaSramBusySpec>;
#[doc = "DOUT RNG SRAM DMA engine status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: DOUT RNG SRAM DMA engine is idle"]
    Idle = 0,
    #[doc = "1: DOUT RNG SRAM DMA engine is busy"]
    Busy = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - DOUT RNG SRAM DMA engine status."]
pub type StatusR = crate::BitReader<Status>;
impl StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Status {
        match self.bits {
            false => Status::Idle,
            true => Status::Busy,
        }
    }
    #[doc = "DOUT RNG SRAM DMA engine is idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Status::Idle
    }
    #[doc = "DOUT RNG SRAM DMA engine is busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Status::Busy
    }
}
impl R {
    #[doc = "Bit 0 - DOUT RNG SRAM DMA engine status."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status register for DOUT DMA engine activity when accessing RNG SRAM.\n\nYou can [`read`](crate::Reg::read) this register and get [`dout_dma_sram_busy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoutDmaSramBusySpec;
impl crate::RegisterSpec for DoutDmaSramBusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout_dma_sram_busy::R`](R) reader structure"]
impl crate::Readable for DoutDmaSramBusySpec {}
#[doc = "`reset()` method sets DOUT_DMA_SRAM_BUSY to value 0"]
impl crate::Resettable for DoutDmaSramBusySpec {}
