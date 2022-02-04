#[doc = "Register `ROSC4` reader"]
pub struct R(crate::R<ROSC4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROSC4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROSC4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROSC4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ROSC4` reader - Sample count for ring oscillator 4"]
pub struct ROSC4_R(crate::FieldReader<u32, u32>);
impl ROSC4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ROSC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSC4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Sample count for ring oscillator 4"]
    #[inline(always)]
    pub fn rosc4(&self) -> ROSC4_R {
        ROSC4_R::new(self.bits)
    }
}
#[doc = "Sample count for ring oscillator 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rosc4](index.html) module"]
pub struct ROSC4_SPEC;
impl crate::RegisterSpec for ROSC4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rosc4::R](R) reader structure"]
impl crate::Readable for ROSC4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ROSC4 to value 0xffff_ffff"]
impl crate::Resettable for ROSC4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
