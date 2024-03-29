#[doc = "Register `SLEEPSTATE` reader"]
pub struct R(crate::R<SLEEPSTATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLEEPSTATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLEEPSTATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLEEPSTATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLEEPSTATE` reader - Reflects the sleep state during automatic collision resolution. Set to IDLE by a GOIDLE task. Set to SLEEP_A when a valid SLEEP_REQ frame is received or by a GOSLEEP task."]
pub type SLEEPSTATE_R = crate::BitReader<SLEEPSTATE_A>;
#[doc = "Reflects the sleep state during automatic collision resolution. Set to IDLE by a GOIDLE task. Set to SLEEP_A when a valid SLEEP_REQ frame is received or by a GOSLEEP task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPSTATE_A {
    #[doc = "0: State is IDLE."]
    IDLE = 0,
    #[doc = "1: State is SLEEP_A."]
    SLEEP_A = 1,
}
impl From<SLEEPSTATE_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPSTATE_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEEPSTATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPSTATE_A {
        match self.bits {
            false => SLEEPSTATE_A::IDLE,
            true => SLEEPSTATE_A::SLEEP_A,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SLEEPSTATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `SLEEP_A`"]
    #[inline(always)]
    pub fn is_sleep_a(&self) -> bool {
        *self == SLEEPSTATE_A::SLEEP_A
    }
}
impl R {
    #[doc = "Bit 0 - Reflects the sleep state during automatic collision resolution. Set to IDLE by a GOIDLE task. Set to SLEEP_A when a valid SLEEP_REQ frame is received or by a GOSLEEP task."]
    #[inline(always)]
    pub fn sleepstate(&self) -> SLEEPSTATE_R {
        SLEEPSTATE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Sleep state during automatic collision resolution\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleepstate](index.html) module"]
pub struct SLEEPSTATE_SPEC;
impl crate::RegisterSpec for SLEEPSTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sleepstate::R](R) reader structure"]
impl crate::Readable for SLEEPSTATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SLEEPSTATE to value 0"]
impl crate::Resettable for SLEEPSTATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
