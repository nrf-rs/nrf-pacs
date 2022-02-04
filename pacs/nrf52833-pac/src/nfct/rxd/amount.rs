#[doc = "Register `AMOUNT` reader"]
pub struct R(crate::R<AMOUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMOUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMOUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMOUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATABITS` reader - Number of bits in the last byte in the frame, if less than 8 (including CRC, but excluding parity and SoF/EoF framing)."]
pub struct RXDATABITS_R(crate::FieldReader<u8, u8>);
impl RXDATABITS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXDATABITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDATABITS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDATABYTES` reader - Number of complete bytes received in the frame (including CRC, but excluding parity and SoF/EoF framing)"]
pub struct RXDATABYTES_R(crate::FieldReader<u16, u16>);
impl RXDATABYTES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RXDATABYTES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDATABYTES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Number of bits in the last byte in the frame, if less than 8 (including CRC, but excluding parity and SoF/EoF framing)."]
    #[inline(always)]
    pub fn rxdatabits(&self) -> RXDATABITS_R {
        RXDATABITS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:11 - Number of complete bytes received in the frame (including CRC, but excluding parity and SoF/EoF framing)"]
    #[inline(always)]
    pub fn rxdatabytes(&self) -> RXDATABYTES_R {
        RXDATABYTES_R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
}
#[doc = "Size of last incoming frame\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amount](index.html) module"]
pub struct AMOUNT_SPEC;
impl crate::RegisterSpec for AMOUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [amount::R](R) reader structure"]
impl crate::Readable for AMOUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AMOUNT to value 0"]
impl crate::Resettable for AMOUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
