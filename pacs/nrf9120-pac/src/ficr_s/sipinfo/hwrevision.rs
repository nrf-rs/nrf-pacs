#[doc = "Register `HWREVISION[%s]` reader"]
pub struct R(crate::R<HWREVISION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWREVISION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWREVISION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWREVISION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HWREVISION` reader - "]
pub type HWREVISION_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn hwrevision(&self) -> HWREVISION_R {
        HWREVISION_R::new(self.bits)
    }
}
#[doc = "Description collection: SIP hardware revision, encoded in ASCII, ex B0A or B1A\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwrevision](index.html) module"]
pub struct HWREVISION_SPEC;
impl crate::RegisterSpec for HWREVISION_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hwrevision::R](R) reader structure"]
impl crate::Readable for HWREVISION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWREVISION[%s]
to value 0xff"]
impl crate::Resettable for HWREVISION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
