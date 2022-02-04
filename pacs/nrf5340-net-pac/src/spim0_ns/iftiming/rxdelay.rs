#[doc = "Register `RXDELAY` reader"]
pub struct R(crate::R<RXDELAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDELAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDELAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDELAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXDELAY` writer"]
pub struct W(crate::W<RXDELAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXDELAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RXDELAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXDELAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXDELAY` reader - Sample delay for input serial data on MISO. The value specifies the number of 64 MHz clock cycles (15.625 ns) delay from the the sampling edge of SCK (leading edge for CONFIG.CPHA = 0, trailing edge for CONFIG.CPHA = 1) until the input serial data is sampled. As en example, if RXDELAY = 0 and CONFIG.CPHA = 0, the input serial data is sampled on the rising edge of SCK."]
pub struct RXDELAY_R(crate::FieldReader<u8, u8>);
impl RXDELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXDELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDELAY` writer - Sample delay for input serial data on MISO. The value specifies the number of 64 MHz clock cycles (15.625 ns) delay from the the sampling edge of SCK (leading edge for CONFIG.CPHA = 0, trailing edge for CONFIG.CPHA = 1) until the input serial data is sampled. As en example, if RXDELAY = 0 and CONFIG.CPHA = 0, the input serial data is sampled on the rising edge of SCK."]
pub struct RXDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Sample delay for input serial data on MISO. The value specifies the number of 64 MHz clock cycles (15.625 ns) delay from the the sampling edge of SCK (leading edge for CONFIG.CPHA = 0, trailing edge for CONFIG.CPHA = 1) until the input serial data is sampled. As en example, if RXDELAY = 0 and CONFIG.CPHA = 0, the input serial data is sampled on the rising edge of SCK."]
    #[inline(always)]
    pub fn rxdelay(&self) -> RXDELAY_R {
        RXDELAY_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sample delay for input serial data on MISO. The value specifies the number of 64 MHz clock cycles (15.625 ns) delay from the the sampling edge of SCK (leading edge for CONFIG.CPHA = 0, trailing edge for CONFIG.CPHA = 1) until the input serial data is sampled. As en example, if RXDELAY = 0 and CONFIG.CPHA = 0, the input serial data is sampled on the rising edge of SCK."]
    #[inline(always)]
    pub fn rxdelay(&mut self) -> RXDELAY_W {
        RXDELAY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sample delay for input serial data on MISO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdelay](index.html) module"]
pub struct RXDELAY_SPEC;
impl crate::RegisterSpec for RXDELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdelay::R](R) reader structure"]
impl crate::Readable for RXDELAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxdelay::W](W) writer structure"]
impl crate::Writable for RXDELAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXDELAY to value 0x02"]
impl crate::Resettable for RXDELAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
