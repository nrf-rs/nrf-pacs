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
#[doc = "Register `AMOUNT` writer"]
pub struct W(crate::W<AMOUNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMOUNT_SPEC>;
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
impl From<crate::W<AMOUNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMOUNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDATABITS` reader - Number of bits in the last or first byte read from RAM that shall be included in the frame (excluding parity bit)."]
pub struct TXDATABITS_R(crate::FieldReader<u8, u8>);
impl TXDATABITS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXDATABITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDATABITS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDATABITS` writer - Number of bits in the last or first byte read from RAM that shall be included in the frame (excluding parity bit)."]
pub struct TXDATABITS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATABITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `TXDATABYTES` reader - Number of complete bytes that shall be included in the frame, excluding CRC, parity and framing"]
pub struct TXDATABYTES_R(crate::FieldReader<u16, u16>);
impl TXDATABYTES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TXDATABYTES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDATABYTES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDATABYTES` writer - Number of complete bytes that shall be included in the frame, excluding CRC, parity and framing"]
pub struct TXDATABYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATABYTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 3)) | ((value as u32 & 0x01ff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Number of bits in the last or first byte read from RAM that shall be included in the frame (excluding parity bit)."]
    #[inline(always)]
    pub fn txdatabits(&self) -> TXDATABITS_R {
        TXDATABITS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:11 - Number of complete bytes that shall be included in the frame, excluding CRC, parity and framing"]
    #[inline(always)]
    pub fn txdatabytes(&self) -> TXDATABYTES_R {
        TXDATABYTES_R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Number of bits in the last or first byte read from RAM that shall be included in the frame (excluding parity bit)."]
    #[inline(always)]
    pub fn txdatabits(&mut self) -> TXDATABITS_W {
        TXDATABITS_W { w: self }
    }
    #[doc = "Bits 3:11 - Number of complete bytes that shall be included in the frame, excluding CRC, parity and framing"]
    #[inline(always)]
    pub fn txdatabytes(&mut self) -> TXDATABYTES_W {
        TXDATABYTES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Size of outgoing frame\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amount](index.html) module"]
pub struct AMOUNT_SPEC;
impl crate::RegisterSpec for AMOUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [amount::R](R) reader structure"]
impl crate::Readable for AMOUNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [amount::W](W) writer structure"]
impl crate::Writable for AMOUNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AMOUNT to value 0"]
impl crate::Resettable for AMOUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
