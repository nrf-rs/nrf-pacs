#[doc = "Register `AMOUNTTX` reader"]
pub struct R(crate::R<AMOUNTTX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMOUNTTX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMOUNTTX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMOUNTTX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AMOUNTTX` reader - Number of bytes transmitted in last granted transaction."]
pub struct AMOUNTTX_R(crate::FieldReader<u8, u8>);
impl AMOUNTTX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AMOUNTTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMOUNTTX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of bytes transmitted in last granted transaction."]
    #[inline(always)]
    pub fn amounttx(&self) -> AMOUNTTX_R {
        AMOUNTTX_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Number of bytes transmitted in last granted transaction.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amounttx](index.html) module"]
pub struct AMOUNTTX_SPEC;
impl crate::RegisterSpec for AMOUNTTX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [amounttx::R](R) reader structure"]
impl crate::Readable for AMOUNTTX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AMOUNTTX to value 0"]
impl crate::Resettable for AMOUNTTX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
