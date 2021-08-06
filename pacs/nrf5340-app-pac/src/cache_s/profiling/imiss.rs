#[doc = "Register `IMISS` reader"]
pub struct R(crate::R<IMISS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMISS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMISS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMISS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MISSES` reader - Number of instruction cache misses"]
pub struct MISSES_R(crate::FieldReader<u32, u32>);
impl MISSES_R {
    pub(crate) fn new(bits: u32) -> Self {
        MISSES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISSES_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Number of instruction cache misses"]
    #[inline(always)]
    pub fn misses(&self) -> MISSES_R {
        MISSES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Description cluster: Instruction fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imiss](index.html) module"]
pub struct IMISS_SPEC;
impl crate::RegisterSpec for IMISS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imiss::R](R) reader structure"]
impl crate::Readable for IMISS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMISS to value 0"]
impl crate::Resettable for IMISS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
