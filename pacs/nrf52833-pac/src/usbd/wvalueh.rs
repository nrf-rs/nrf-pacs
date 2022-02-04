#[doc = "Register `WVALUEH` reader"]
pub struct R(crate::R<WVALUEH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WVALUEH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WVALUEH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WVALUEH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WVALUEH` reader - SETUP data, byte 3, MSB of wValue"]
pub struct WVALUEH_R(crate::FieldReader<u8, u8>);
impl WVALUEH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WVALUEH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WVALUEH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 3, MSB of wValue"]
    #[inline(always)]
    pub fn wvalueh(&self) -> WVALUEH_R {
        WVALUEH_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SETUP data, byte 3, MSB of wValue\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wvalueh](index.html) module"]
pub struct WVALUEH_SPEC;
impl crate::RegisterSpec for WVALUEH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wvalueh::R](R) reader structure"]
impl crate::Readable for WVALUEH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WVALUEH to value 0"]
impl crate::Resettable for WVALUEH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
