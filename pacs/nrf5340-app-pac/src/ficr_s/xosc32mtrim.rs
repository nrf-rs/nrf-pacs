#[doc = "Register `XOSC32MTRIM` reader"]
pub struct R(crate::R<XOSC32MTRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XOSC32MTRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XOSC32MTRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XOSC32MTRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLOPE` reader - Slope trim factor on twos complement form"]
pub struct SLOPE_R(crate::FieldReader<u8, u8>);
impl SLOPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SLOPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET` reader - Offset trim factor on integer form"]
pub struct OFFSET_R(crate::FieldReader<u8, u8>);
impl OFFSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - Slope trim factor on twos complement form"]
    #[inline(always)]
    pub fn slope(&self) -> SLOPE_R {
        SLOPE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Offset trim factor on integer form"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
}
#[doc = "XOSC32M capacitor selection trim values\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xosc32mtrim](index.html) module"]
pub struct XOSC32MTRIM_SPEC;
impl crate::RegisterSpec for XOSC32MTRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xosc32mtrim::R](R) reader structure"]
impl crate::Readable for XOSC32MTRIM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets XOSC32MTRIM to value 0xffff_ffff"]
impl crate::Resettable for XOSC32MTRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
