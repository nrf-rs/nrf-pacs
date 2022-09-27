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
#[doc = "Field `RXDELAY` reader - Timing related to sampling of the input serial data. The value of RXDELAY specifies the number of 64 MHz cycles (15.625 ns) delay from the the rising edge of the SPI Clock (SCK) until the input serial data is sampled. As en example, if set to 0 the input serial data is sampled on the rising edge of SCK."]
pub type RXDELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXDELAY` writer - Timing related to sampling of the input serial data. The value of RXDELAY specifies the number of 64 MHz cycles (15.625 ns) delay from the the rising edge of the SPI Clock (SCK) until the input serial data is sampled. As en example, if set to 0 the input serial data is sampled on the rising edge of SCK."]
pub type RXDELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IFTIMING_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 8:10 - Timing related to sampling of the input serial data. The value of RXDELAY specifies the number of 64 MHz cycles (15.625 ns) delay from the the rising edge of the SPI Clock (SCK) until the input serial data is sampled. As en example, if set to 0 the input serial data is sampled on the rising edge of SCK."]
    #[inline(always)]
    pub fn rxdelay(&self) -> RXDELAY_R {
        RXDELAY_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - Timing related to sampling of the input serial data. The value of RXDELAY specifies the number of 64 MHz cycles (15.625 ns) delay from the the rising edge of the SPI Clock (SCK) until the input serial data is sampled. As en example, if set to 0 the input serial data is sampled on the rising edge of SCK."]
    #[inline(always)]
    pub fn rxdelay(&mut self) -> RXDELAY_W<8> {
        RXDELAY_W::new(self)
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
