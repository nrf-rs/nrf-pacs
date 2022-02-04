#[doc = "Register `WLENGTHH` reader"]
pub struct R(crate::R<WLENGTHH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WLENGTHH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WLENGTHH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WLENGTHH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WLENGTHH` reader - SETUP data, byte 7, MSB of wLength"]
pub struct WLENGTHH_R(crate::FieldReader<u8, u8>);
impl WLENGTHH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WLENGTHH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WLENGTHH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 7, MSB of wLength"]
    #[inline(always)]
    pub fn wlengthh(&self) -> WLENGTHH_R {
        WLENGTHH_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SETUP data, byte 7, MSB of wLength\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wlengthh](index.html) module"]
pub struct WLENGTHH_SPEC;
impl crate::RegisterSpec for WLENGTHH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wlengthh::R](R) reader structure"]
impl crate::Readable for WLENGTHH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WLENGTHH to value 0"]
impl crate::Resettable for WLENGTHH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
