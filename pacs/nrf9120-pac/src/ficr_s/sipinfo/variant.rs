#[doc = "Register `VARIANT[%s]` reader"]
pub struct R(crate::R<VARIANT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VARIANT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VARIANT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VARIANT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VARIANT` reader - "]
pub type VARIANT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(self.bits)
    }
}
#[doc = "Description collection: SIP VARIANT, encoded in ASCII, ex SIAA, SIBA or SICA\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [variant](index.html) module"]
pub struct VARIANT_SPEC;
impl crate::RegisterSpec for VARIANT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [variant::R](R) reader structure"]
impl crate::Readable for VARIANT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VARIANT[%s]
to value 0xff"]
impl crate::Resettable for VARIANT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
