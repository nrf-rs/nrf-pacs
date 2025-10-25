#[doc = "Register `AES_BUSY` reader"]
pub type R = crate::R<AesBusySpec>;
#[doc = "AES engine status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: AES engine is idle"]
    Idle = 0,
    #[doc = "1: AES engine is busy"]
    Busy = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - AES engine status."]
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
    #[doc = "AES engine is idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Status::Idle
    }
    #[doc = "AES engine is busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Status::Busy
    }
}
impl R {
    #[doc = "Bit 0 - AES engine status."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status register for AES engine activity.\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_busy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesBusySpec;
impl crate::RegisterSpec for AesBusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_busy::R`](R) reader structure"]
impl crate::Readable for AesBusySpec {}
#[doc = "`reset()` method sets AES_BUSY to value 0"]
impl crate::Resettable for AesBusySpec {}
