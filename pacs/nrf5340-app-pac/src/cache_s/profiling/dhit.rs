#[doc = "Register `DHIT` reader"]
pub struct R(crate::R<DHIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DHIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DHIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DHIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HITS` reader - Number of data cache hits"]
pub type HITS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of data cache hits"]
    #[inline(always)]
    pub fn hits(&self) -> HITS_R {
        HITS_R::new(self.bits)
    }
}
#[doc = "Description cluster: Data fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhit](index.html) module"]
pub struct DHIT_SPEC;
impl crate::RegisterSpec for DHIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dhit::R](R) reader structure"]
impl crate::Readable for DHIT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DHIT to value 0"]
impl crate::Resettable for DHIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
