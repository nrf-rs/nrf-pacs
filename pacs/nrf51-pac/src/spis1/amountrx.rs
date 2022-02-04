#[doc = "Register `AMOUNTRX` reader"]
pub struct R(crate::R<AMOUNTRX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMOUNTRX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMOUNTRX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMOUNTRX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AMOUNTRX` reader - Number of bytes received in last granted transaction."]
pub struct AMOUNTRX_R(crate::FieldReader<u8, u8>);
impl AMOUNTRX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AMOUNTRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMOUNTRX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of bytes received in last granted transaction."]
    #[inline(always)]
    pub fn amountrx(&self) -> AMOUNTRX_R {
        AMOUNTRX_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Number of bytes received in last granted transaction.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amountrx](index.html) module"]
pub struct AMOUNTRX_SPEC;
impl crate::RegisterSpec for AMOUNTRX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [amountrx::R](R) reader structure"]
impl crate::Readable for AMOUNTRX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AMOUNTRX to value 0"]
impl crate::Resettable for AMOUNTRX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
