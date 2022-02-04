#[doc = "Register `WVALUEL` reader"]
pub struct R(crate::R<WVALUEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WVALUEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WVALUEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WVALUEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WVALUEL` reader - SETUP data, byte 2, LSB of wValue"]
pub struct WVALUEL_R(crate::FieldReader<u8, u8>);
impl WVALUEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WVALUEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WVALUEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 2, LSB of wValue"]
    #[inline(always)]
    pub fn wvaluel(&self) -> WVALUEL_R {
        WVALUEL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SETUP data, byte 2, LSB of wValue\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wvaluel](index.html) module"]
pub struct WVALUEL_SPEC;
impl crate::RegisterSpec for WVALUEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wvaluel::R](R) reader structure"]
impl crate::Readable for WVALUEL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WVALUEL to value 0"]
impl crate::Resettable for WVALUEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
