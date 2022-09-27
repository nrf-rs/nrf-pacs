#[doc = "Register `VREQHREADY` reader"]
pub struct R(crate::R<VREQHREADY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREQHREADY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREQHREADY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREQHREADY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `READY` reader - RADIO is ready to operate on high voltage"]
pub type READY_R = crate::BitReader<READY_A>;
#[doc = "RADIO is ready to operate on high voltage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_A {
    #[doc = "0: Not ready"]
    NOT_READY = 0,
    #[doc = "1: Ready"]
    READY = 1,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
impl READY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            false => READY_A::NOT_READY,
            true => READY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == READY_A::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == READY_A::READY
    }
}
impl R {
    #[doc = "Bit 0 - RADIO is ready to operate on high voltage"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "High voltage on RADIO is ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vreqhready](index.html) module"]
pub struct VREQHREADY_SPEC;
impl crate::RegisterSpec for VREQHREADY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vreqhready::R](R) reader structure"]
impl crate::Readable for VREQHREADY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VREQHREADY to value 0"]
impl crate::Resettable for VREQHREADY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
