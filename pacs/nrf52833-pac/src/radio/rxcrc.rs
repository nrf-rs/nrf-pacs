#[doc = "Register `RXCRC` reader"]
pub struct R(crate::R<RXCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXCRC` reader - CRC field of previously received packet"]
pub struct RXCRC_R(crate::FieldReader<u32, u32>);
impl RXCRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RXCRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCRC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23 - CRC field of previously received packet"]
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "CRC field of previously received packet\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcrc](index.html) module"]
pub struct RXCRC_SPEC;
impl crate::RegisterSpec for RXCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxcrc::R](R) reader structure"]
impl crate::Readable for RXCRC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXCRC to value 0"]
impl crate::Resettable for RXCRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
