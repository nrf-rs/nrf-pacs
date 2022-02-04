#[doc = "Register `HFCLKRUN` reader"]
pub struct R(crate::R<HFCLKRUN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFCLKRUN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFCLKRUN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFCLKRUN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "HFCLKSTART task triggered or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUS_A {
    #[doc = "0: Task not triggered"]
    NOTTRIGGERED = 0,
    #[doc = "1: Task triggered"]
    TRIGGERED = 1,
}
impl From<STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - HFCLKSTART task triggered or not"]
pub struct STATUS_R(crate::FieldReader<bool, STATUS_A>);
impl STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATUS_A {
        match self.bits {
            false => STATUS_A::NOTTRIGGERED,
            true => STATUS_A::TRIGGERED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTTRIGGERED`"]
    #[inline(always)]
    pub fn is_not_triggered(&self) -> bool {
        **self == STATUS_A::NOTTRIGGERED
    }
    #[doc = "Checks if the value of the field is `TRIGGERED`"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        **self == STATUS_A::TRIGGERED
    }
}
impl core::ops::Deref for STATUS_R {
    type Target = crate::FieldReader<bool, STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - HFCLKSTART task triggered or not"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Status indicating that HFCLKSTART task has been triggered\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfclkrun](index.html) module"]
pub struct HFCLKRUN_SPEC;
impl crate::RegisterSpec for HFCLKRUN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfclkrun::R](R) reader structure"]
impl crate::Readable for HFCLKRUN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HFCLKRUN to value 0"]
impl crate::Resettable for HFCLKRUN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
