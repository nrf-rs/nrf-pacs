#[doc = "Register `IHIT` reader"]
pub struct R(crate::R<IHIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IHIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IHIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IHIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HITS` reader - Number of instruction cache hits"]
pub struct HITS_R(crate::FieldReader<u32, u32>);
impl HITS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        HITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HITS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Number of instruction cache hits"]
    #[inline(always)]
    pub fn hits(&self) -> HITS_R {
        HITS_R::new(self.bits)
    }
}
#[doc = "Description cluster: Instruction fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ihit](index.html) module"]
pub struct IHIT_SPEC;
impl crate::RegisterSpec for IHIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ihit::R](R) reader structure"]
impl crate::Readable for IHIT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IHIT to value 0"]
impl crate::Resettable for IHIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
