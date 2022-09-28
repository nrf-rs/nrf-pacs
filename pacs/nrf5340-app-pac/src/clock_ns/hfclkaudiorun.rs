#[doc = "Register `HFCLKAUDIORUN` reader"]
pub struct R(crate::R<HFCLKAUDIORUN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFCLKAUDIORUN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFCLKAUDIORUN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFCLKAUDIORUN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STATUS` reader - HFCLKAUDIOSTART task triggered or not"]
pub type STATUS_R = crate::BitReader<STATUS_A>;
#[doc = "HFCLKAUDIOSTART task triggered or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUS_A {
    #[doc = "0: Task not triggered"]
    NOT_TRIGGERED = 0,
    #[doc = "1: Task triggered"]
    TRIGGERED = 1,
}
impl From<STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATUS_A {
        match self.bits {
            false => STATUS_A::NOT_TRIGGERED,
            true => STATUS_A::TRIGGERED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_TRIGGERED`"]
    #[inline(always)]
    pub fn is_not_triggered(&self) -> bool {
        *self == STATUS_A::NOT_TRIGGERED
    }
    #[doc = "Checks if the value of the field is `TRIGGERED`"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == STATUS_A::TRIGGERED
    }
}
impl R {
    #[doc = "Bit 0 - HFCLKAUDIOSTART task triggered or not"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Status indicating that HFCLKAUDIOSTART task has been triggered\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfclkaudiorun](index.html) module"]
pub struct HFCLKAUDIORUN_SPEC;
impl crate::RegisterSpec for HFCLKAUDIORUN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfclkaudiorun::R](R) reader structure"]
impl crate::Readable for HFCLKAUDIORUN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HFCLKAUDIORUN to value 0"]
impl crate::Resettable for HFCLKAUDIORUN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
