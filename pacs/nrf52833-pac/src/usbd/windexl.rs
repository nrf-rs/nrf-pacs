#[doc = "Register `WINDEXL` reader"]
pub struct R(crate::R<WINDEXL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WINDEXL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WINDEXL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WINDEXL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WINDEXL` reader - SETUP data, byte 4, LSB of wIndex"]
pub struct WINDEXL_R(crate::FieldReader<u8, u8>);
impl WINDEXL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WINDEXL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WINDEXL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 4, LSB of wIndex"]
    #[inline(always)]
    pub fn windexl(&self) -> WINDEXL_R {
        WINDEXL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SETUP data, byte 4, LSB of wIndex\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [windexl](index.html) module"]
pub struct WINDEXL_SPEC;
impl crate::RegisterSpec for WINDEXL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [windexl::R](R) reader structure"]
impl crate::Readable for WINDEXL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WINDEXL to value 0"]
impl crate::Resettable for WINDEXL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
