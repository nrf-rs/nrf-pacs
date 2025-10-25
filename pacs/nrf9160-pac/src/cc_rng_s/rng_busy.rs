#[doc = "Register `RNG_BUSY` reader"]
pub type R = crate::R<RngBusySpec>;
#[doc = "RNG engine status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: RNG engine is idle"]
    Idle = 0,
    #[doc = "1: RNG engine is busy"]
    Busy = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - RNG engine status."]
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
    #[doc = "RNG engine is idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Status::Idle
    }
    #[doc = "RNG engine is busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Status::Busy
    }
}
#[doc = "TRNG status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrngStatus {
    #[doc = "0: TRNG is idle"]
    Idle = 0,
    #[doc = "1: TRNG is busy"]
    Busy = 1,
}
impl From<TrngStatus> for bool {
    #[inline(always)]
    fn from(variant: TrngStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRNG_STATUS` reader - TRNG status."]
pub type TrngStatusR = crate::BitReader<TrngStatus>;
impl TrngStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TrngStatus {
        match self.bits {
            false => TrngStatus::Idle,
            true => TrngStatus::Busy,
        }
    }
    #[doc = "TRNG is idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TrngStatus::Idle
    }
    #[doc = "TRNG is busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == TrngStatus::Busy
    }
}
impl R {
    #[doc = "Bit 0 - RNG engine status."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TRNG status."]
    #[inline(always)]
    pub fn trng_status(&self) -> TrngStatusR {
        TrngStatusR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Status register for RNG engine activity.\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_busy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngBusySpec;
impl crate::RegisterSpec for RngBusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_busy::R`](R) reader structure"]
impl crate::Readable for RngBusySpec {}
#[doc = "`reset()` method sets RNG_BUSY to value 0"]
impl crate::Resettable for RngBusySpec {}
