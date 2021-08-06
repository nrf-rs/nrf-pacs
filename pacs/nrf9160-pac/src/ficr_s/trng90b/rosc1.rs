#[doc = "Register `ROSC1` reader"]
pub struct R(crate::R<ROSC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROSC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROSC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROSC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ROSC1` reader - Sample count for ring oscillator 1"]
pub struct ROSC1_R(crate::FieldReader<u32, u32>);
impl ROSC1_R {
    pub(crate) fn new(bits: u32) -> Self {
        ROSC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSC1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Sample count for ring oscillator 1"]
    #[inline(always)]
    pub fn rosc1(&self) -> ROSC1_R {
        ROSC1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Sample count for ring oscillator 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rosc1](index.html) module"]
pub struct ROSC1_SPEC;
impl crate::RegisterSpec for ROSC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rosc1::R](R) reader structure"]
impl crate::Readable for ROSC1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ROSC1 to value 0xffff_ffff"]
impl crate::Resettable for ROSC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
