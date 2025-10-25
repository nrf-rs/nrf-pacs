#[doc = "Register `HASH_BUSY` reader"]
pub type R = crate::R<HashBusySpec>;
#[doc = "Hash engine status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: HASH engine is idle"]
    Idle = 0,
    #[doc = "1: HASH engine is busy"]
    Busy = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - Hash engine status."]
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
    #[doc = "HASH engine is idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Status::Idle
    }
    #[doc = "HASH engine is busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Status::Busy
    }
}
impl R {
    #[doc = "Bit 0 - Hash engine status."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status register for HASH engine activity.\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_busy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashBusySpec;
impl crate::RegisterSpec for HashBusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_busy::R`](R) reader structure"]
impl crate::Readable for HashBusySpec {}
#[doc = "`reset()` method sets HASH_BUSY to value 0"]
impl crate::Resettable for HashBusySpec {}
