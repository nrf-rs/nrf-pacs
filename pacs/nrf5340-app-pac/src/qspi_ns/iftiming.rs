#[doc = "Register `IFTIMING` reader"]
pub struct R(crate::R<IFTIMING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFTIMING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFTIMING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFTIMING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFTIMING` writer"]
pub struct W(crate::W<IFTIMING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFTIMING_SPEC>;
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
impl From<crate::W<IFTIMING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFTIMING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXDELAY` reader - Timing related to sampling of the input serial data. The value of RXDELAY specifies the number of prescaled 192 MHz cycles delay from the the rising edge of the SPI Clock (SCK) until the input serial data is sampled. For example, if RXDELAY is set to 0, the input serial data is sampled on the rising edge of SCK."]
pub struct RXDELAY_R(crate::FieldReader<u8, u8>);
impl RXDELAY_R {
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
#[doc = "Field `RXDELAY` writer - Timing related to sampling of the input serial data. The value of RXDELAY specifies the number of prescaled 192 MHz cycles delay from the the rising edge of the SPI Clock (SCK) until the input serial data is sampled. For example, if RXDELAY is set to 0, the input serial data is sampled on the rising edge of SCK."]
pub struct RXDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:10 - Timing related to sampling of the input serial data. The value of RXDELAY specifies the number of prescaled 192 MHz cycles delay from the the rising edge of the SPI Clock (SCK) until the input serial data is sampled. For example, if RXDELAY is set to 0, the input serial data is sampled on the rising edge of SCK."]
    #[inline(always)]
    pub fn rxdelay(&self) -> RXDELAY_R {
        RXDELAY_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - Timing related to sampling of the input serial data. The value of RXDELAY specifies the number of prescaled 192 MHz cycles delay from the the rising edge of the SPI Clock (SCK) until the input serial data is sampled. For example, if RXDELAY is set to 0, the input serial data is sampled on the rising edge of SCK."]
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
#[doc = "SPI interface timing.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iftiming](index.html) module"]
pub struct IFTIMING_SPEC;
impl crate::RegisterSpec for IFTIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iftiming::R](R) reader structure"]
impl crate::Readable for IFTIMING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iftiming::W](W) writer structure"]
impl crate::Writable for IFTIMING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFTIMING to value 0x0200"]
impl crate::Resettable for IFTIMING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
