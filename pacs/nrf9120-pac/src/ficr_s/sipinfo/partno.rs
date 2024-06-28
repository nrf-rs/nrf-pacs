#[doc = "Register `PARTNO` reader"]
pub struct R(crate::R<PARTNO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARTNO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARTNO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARTNO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PARTNO` reader - "]
pub type PARTNO_R = crate::FieldReader<u32, PARTNO_A>;
#[doc = "\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PARTNO_A {
    #[doc = "37216: Device is an nRF9160 sip"]
    _9160 = 37216,
}
impl From<PARTNO_A> for u32 {
    #[inline(always)]
    fn from(variant: PARTNO_A) -> Self {
        variant as _
    }
}
impl PARTNO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PARTNO_A> {
        match self.bits {
            37216 => Some(PARTNO_A::_9160),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_9160`"]
    #[inline(always)]
    pub fn is_9160(&self) -> bool {
        *self == PARTNO_A::_9160
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn partno(&self) -> PARTNO_R {
        PARTNO_R::new(self.bits)
    }
}
#[doc = "SIP part number\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [partno](index.html) module"]
pub struct PARTNO_SPEC;
impl crate::RegisterSpec for PARTNO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [partno::R](R) reader structure"]
impl crate::Readable for PARTNO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PARTNO to value 0xffff_ffff"]
impl crate::Resettable for PARTNO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
