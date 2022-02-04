#[doc = "Register `WINDEXH` reader"]
pub struct R(crate::R<WINDEXH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WINDEXH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WINDEXH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WINDEXH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WINDEXH` reader - SETUP data, byte 5, MSB of wIndex"]
pub struct WINDEXH_R(crate::FieldReader<u8, u8>);
impl WINDEXH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WINDEXH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINDEXH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 5, MSB of wIndex"]
    #[inline(always)]
    pub fn windexh(&self) -> WINDEXH_R {
        WINDEXH_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SETUP data, byte 5, MSB of wIndex\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [windexh](index.html) module"]
pub struct WINDEXH_SPEC;
impl crate::RegisterSpec for WINDEXH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [windexh::R](R) reader structure"]
impl crate::Readable for WINDEXH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WINDEXH to value 0"]
impl crate::Resettable for WINDEXH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
