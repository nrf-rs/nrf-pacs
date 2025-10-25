#[doc = "Register `DOUT_DMA_MEM_BUSY` reader"]
pub type R = crate::R<DoutDmaMemBusySpec>;
#[doc = "DOUT memory DMA engine status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: DOUT memory DMA engine is idle"]
    Idle = 0,
    #[doc = "1: DOUT memory DMA engine is busy"]
    Busy = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - DOUT memory DMA engine status."]
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
    #[doc = "DOUT memory DMA engine is idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Status::Idle
    }
    #[doc = "DOUT memory DMA engine is busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Status::Busy
    }
}
impl R {
    #[doc = "Bit 0 - DOUT memory DMA engine status."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status register for DOUT DMA engine activity when accessing memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`dout_dma_mem_busy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoutDmaMemBusySpec;
impl crate::RegisterSpec for DoutDmaMemBusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout_dma_mem_busy::R`](R) reader structure"]
impl crate::Readable for DoutDmaMemBusySpec {}
#[doc = "`reset()` method sets DOUT_DMA_MEM_BUSY to value 0"]
impl crate::Resettable for DoutDmaMemBusySpec {}
