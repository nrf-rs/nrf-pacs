#[doc = "Register `LFCLKRUN` reader"]
pub type R = crate::R<LfclkrunSpec>;
#[doc = "Task LFCLKSTART triggered status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: Task LFCLKSTART has not been triggered."]
    NotTriggered = 0,
    #[doc = "1: Task LFCLKSTART has been triggered."]
    Triggered = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - Task LFCLKSTART triggered status."]
pub type StatusR = crate::BitReader<Status>;
impl StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Status {
        match self.bits {
            false => Status::NotTriggered,
            true => Status::Triggered,
        }
    }
    #[doc = "Task LFCLKSTART has not been triggered."]
    #[inline(always)]
    pub fn is_not_triggered(&self) -> bool {
        *self == Status::NotTriggered
    }
    #[doc = "Task LFCLKSTART has been triggered."]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == Status::Triggered
    }
}
impl R {
    #[doc = "Bit 0 - Task LFCLKSTART triggered status."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "Task LFCLKSTART triggered status.\n\nYou can [`read`](crate::Reg::read) this register and get [`lfclkrun::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfclkrunSpec;
impl crate::RegisterSpec for LfclkrunSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfclkrun::R`](R) reader structure"]
impl crate::Readable for LfclkrunSpec {}
#[doc = "`reset()` method sets LFCLKRUN to value 0"]
impl crate::Resettable for LfclkrunSpec {}
