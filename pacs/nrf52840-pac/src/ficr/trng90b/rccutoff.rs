#[doc = "Register `RCCUTOFF` reader"]
pub struct R(crate::R<RCCUTOFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCCUTOFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCCUTOFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCCUTOFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RCCUTOFF` reader - Repetition counter cutoff"]
pub struct RCCUTOFF_R(crate::FieldReader<u32, u32>);
impl RCCUTOFF_R {
    pub(crate) fn new(bits: u32) -> Self {
        RCCUTOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCCUTOFF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Repetition counter cutoff"]
    #[inline(always)]
    pub fn rccutoff(&self) -> RCCUTOFF_R {
        RCCUTOFF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Repetition counter cutoff\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rccutoff](index.html) module"]
pub struct RCCUTOFF_SPEC;
impl crate::RegisterSpec for RCCUTOFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rccutoff::R](R) reader structure"]
impl crate::Readable for RCCUTOFF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RCCUTOFF to value 0xffff_ffff"]
impl crate::Resettable for RCCUTOFF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
