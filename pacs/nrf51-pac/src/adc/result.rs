#[doc = "Register `RESULT` reader"]
pub struct R(crate::R<RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESULT` reader - Result of ADC conversion."]
pub type RESULT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Result of ADC conversion."]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Result of ADC conversion.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result](index.html) module"]
pub struct RESULT_SPEC;
impl crate::RegisterSpec for RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result::R](R) reader structure"]
impl crate::Readable for RESULT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT to value 0"]
impl crate::Resettable for RESULT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
