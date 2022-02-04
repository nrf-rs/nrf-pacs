#[doc = "Register `ROSC2` reader"]
pub struct R(crate::R<ROSC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROSC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROSC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROSC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ROSC2` reader - Sample count for ring oscillator 2"]
pub struct ROSC2_R(crate::FieldReader<u32, u32>);
impl ROSC2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ROSC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSC2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Sample count for ring oscillator 2"]
    #[inline(always)]
    pub fn rosc2(&self) -> ROSC2_R {
        ROSC2_R::new(self.bits)
    }
}
#[doc = "Sample count for ring oscillator 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rosc2](index.html) module"]
pub struct ROSC2_SPEC;
impl crate::RegisterSpec for ROSC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rosc2::R](R) reader structure"]
impl crate::Readable for ROSC2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ROSC2 to value 0xffff_ffff"]
impl crate::Resettable for ROSC2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
