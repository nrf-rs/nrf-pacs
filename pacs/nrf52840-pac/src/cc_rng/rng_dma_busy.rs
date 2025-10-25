#[doc = "Register `RNG_DMA_BUSY` reader"]
pub type R = crate::R<RngDmaBusySpec>;
#[doc = "RNG DMA engine status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: RNG DMA engine is idle"]
    Idle = 0,
    #[doc = "1: RNG DMA engine is busy"]
    Busy = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - RNG DMA engine status."]
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
    #[doc = "RNG DMA engine is idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Status::Idle
    }
    #[doc = "RNG DMA engine is busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Status::Busy
    }
}
#[doc = "The active ring oscillator length configuration used by the RNG DMA engine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RoscLen {
    #[doc = "0: Shortest ROSC1 ring oscillator configuration used."]
    Rosc1 = 0,
    #[doc = "1: ROSC2 ring oscillator configuration used."]
    Rosc2 = 1,
    #[doc = "2: ROSC3 ring oscillator configuration used."]
    Rosc3 = 2,
    #[doc = "3: Longest ROSC4 ring oscillator configuration used."]
    Rosc4 = 3,
}
impl From<RoscLen> for u8 {
    #[inline(always)]
    fn from(variant: RoscLen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RoscLen {
    type Ux = u8;
}
impl crate::IsEnum for RoscLen {}
#[doc = "Field `ROSC_LEN` reader - The active ring oscillator length configuration used by the RNG DMA engine."]
pub type RoscLenR = crate::FieldReader<RoscLen>;
impl RoscLenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RoscLen {
        match self.bits {
            0 => RoscLen::Rosc1,
            1 => RoscLen::Rosc2,
            2 => RoscLen::Rosc3,
            3 => RoscLen::Rosc4,
            _ => unreachable!(),
        }
    }
    #[doc = "Shortest ROSC1 ring oscillator configuration used."]
    #[inline(always)]
    pub fn is_rosc1(&self) -> bool {
        *self == RoscLen::Rosc1
    }
    #[doc = "ROSC2 ring oscillator configuration used."]
    #[inline(always)]
    pub fn is_rosc2(&self) -> bool {
        *self == RoscLen::Rosc2
    }
    #[doc = "ROSC3 ring oscillator configuration used."]
    #[inline(always)]
    pub fn is_rosc3(&self) -> bool {
        *self == RoscLen::Rosc3
    }
    #[doc = "Longest ROSC4 ring oscillator configuration used."]
    #[inline(always)]
    pub fn is_rosc4(&self) -> bool {
        *self == RoscLen::Rosc4
    }
}
#[doc = "Field `NUM_OF_SAMPLES` reader - Number of samples already collected using the current ring oscillator configuration."]
pub type NumOfSamplesR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - RNG DMA engine status."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - The active ring oscillator length configuration used by the RNG DMA engine."]
    #[inline(always)]
    pub fn rosc_len(&self) -> RoscLenR {
        RoscLenR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:10 - Number of samples already collected using the current ring oscillator configuration."]
    #[inline(always)]
    pub fn num_of_samples(&self) -> NumOfSamplesR {
        NumOfSamplesR::new(((self.bits >> 3) & 0xff) as u8)
    }
}
#[doc = "Status register for RNG DMA engine activity.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_dma_busy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngDmaBusySpec;
impl crate::RegisterSpec for RngDmaBusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_dma_busy::R`](R) reader structure"]
impl crate::Readable for RngDmaBusySpec {}
#[doc = "`reset()` method sets RNG_DMA_BUSY to value 0"]
impl crate::Resettable for RngDmaBusySpec {}
