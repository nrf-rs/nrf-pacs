#[doc = "Register `ACCDBL` reader"]
pub struct R(crate::R<ACCDBL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACCDBL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACCDBL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACCDBL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACCDBL` reader - Accumulated double (error) transitions."]
pub type ACCDBL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Accumulated double (error) transitions."]
    #[inline(always)]
    pub fn accdbl(&self) -> ACCDBL_R {
        ACCDBL_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Accumulated double (error) transitions register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [accdbl](index.html) module"]
pub struct ACCDBL_SPEC;
impl crate::RegisterSpec for ACCDBL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [accdbl::R](R) reader structure"]
impl crate::Readable for ACCDBL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACCDBL to value 0"]
impl crate::Resettable for ACCDBL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
