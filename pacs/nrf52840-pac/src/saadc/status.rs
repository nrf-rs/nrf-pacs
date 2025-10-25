#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: SAADC is ready. No on-going conversions."]
    Ready = 0,
    #[doc = "1: SAADC is busy. Conversion in progress."]
    Busy = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - Status"]
pub type StatusR = crate::BitReader<Status>;
impl StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Status {
        match self.bits {
            false => Status::Ready,
            true => Status::Busy,
        }
    }
    #[doc = "SAADC is ready. No on-going conversions."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Status::Ready
    }
    #[doc = "SAADC is busy. Conversion in progress."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Status::Busy
    }
}
impl R {
    #[doc = "Bit 0 - Status"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
