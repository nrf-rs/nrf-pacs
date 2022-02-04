#[doc = "Register `EDSAMPLE` reader"]
pub struct R(crate::R<EDSAMPLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDSAMPLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDSAMPLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDSAMPLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EDLVL` reader - IEEE 802.15.4 energy detect level"]
pub struct EDLVL_R(crate::FieldReader<u8, u8>);
impl EDLVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EDLVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDLVL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - IEEE 802.15.4 energy detect level"]
    #[inline(always)]
    pub fn edlvl(&self) -> EDLVL_R {
        EDLVL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "IEEE 802.15.4 energy detect level\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edsample](index.html) module"]
pub struct EDSAMPLE_SPEC;
impl crate::RegisterSpec for EDSAMPLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edsample::R](R) reader structure"]
impl crate::Readable for EDSAMPLE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EDSAMPLE to value 0"]
impl crate::Resettable for EDSAMPLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
