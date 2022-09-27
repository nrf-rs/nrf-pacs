#[doc = "Register `ROSC3` reader"]
pub struct R(crate::R<ROSC3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROSC3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROSC3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROSC3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ROSC3` reader - Sample count for ring oscillator 3"]
pub type ROSC3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Sample count for ring oscillator 3"]
    #[inline(always)]
    pub fn rosc3(&self) -> ROSC3_R {
        ROSC3_R::new(self.bits)
    }
}
#[doc = "Sample count for ring oscillator 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rosc3](index.html) module"]
pub struct ROSC3_SPEC;
impl crate::RegisterSpec for ROSC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rosc3::R](R) reader structure"]
impl crate::Readable for ROSC3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ROSC3 to value 0xffff_ffff"]
impl crate::Resettable for ROSC3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
