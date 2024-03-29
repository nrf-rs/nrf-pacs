#[doc = "Register `RXD` reader"]
pub struct R(crate::R<RXD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXD` reader - RX data from previous transfer. Double buffered."]
pub type RXD_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RX data from previous transfer. Double buffered."]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RXD register. On read action the buffer pointer is displaced. Once read the character is consumed. If read when no character available, the UART will stop working.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxd](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct RXD_SPEC;
impl crate::RegisterSpec for RXD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxd::R](R) reader structure"]
impl crate::Readable for RXD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXD to value 0"]
impl crate::Resettable for RXD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
