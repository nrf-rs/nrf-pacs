#[doc = "Register `WLENGTHL` reader"]
pub struct R(crate::R<WLENGTHL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WLENGTHL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WLENGTHL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WLENGTHL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WLENGTHL` reader - SETUP data, byte 6, LSB of wLength"]
pub struct WLENGTHL_R(crate::FieldReader<u8, u8>);
impl WLENGTHL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WLENGTHL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WLENGTHL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 6, LSB of wLength"]
    #[inline(always)]
    pub fn wlengthl(&self) -> WLENGTHL_R {
        WLENGTHL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SETUP data, byte 6, LSB of wLength\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wlengthl](index.html) module"]
pub struct WLENGTHL_SPEC;
impl crate::RegisterSpec for WLENGTHL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wlengthl::R](R) reader structure"]
impl crate::Readable for WLENGTHL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WLENGTHL to value 0"]
impl crate::Resettable for WLENGTHL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
