#[doc = "Register `HFCLKRUN` reader"]
pub type R = crate::R<HfclkrunSpec>;
#[doc = "HFCLKSTART task triggered or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: Task not triggered"]
    NotTriggered = 0,
    #[doc = "1: Task triggered"]
    Triggered = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - HFCLKSTART task triggered or not"]
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
    #[doc = "Task not triggered"]
    #[inline(always)]
    pub fn is_not_triggered(&self) -> bool {
        *self == Status::NotTriggered
    }
    #[doc = "Task triggered"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == Status::Triggered
    }
}
impl R {
    #[doc = "Bit 0 - HFCLKSTART task triggered or not"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status indicating that HFCLKSTART task has been triggered\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfclkrun::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfclkrunSpec;
impl crate::RegisterSpec for HfclkrunSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfclkrun::R`](R) reader structure"]
impl crate::Readable for HfclkrunSpec {}
#[doc = "`reset()` method sets HFCLKRUN to value 0"]
impl crate::Resettable for HfclkrunSpec {
    const RESET_VALUE: u32 = 0;
}
