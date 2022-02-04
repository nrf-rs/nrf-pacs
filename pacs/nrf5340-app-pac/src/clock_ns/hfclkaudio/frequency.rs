#[doc = "Register `FREQUENCY` reader"]
pub struct R(crate::R<FREQUENCY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQUENCY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQUENCY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQUENCY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREQUENCY` writer"]
pub struct W(crate::W<FREQUENCY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQUENCY_SPEC>;
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
impl From<crate::W<FREQUENCY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREQUENCY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FREQUENCY` reader - Frequency 0: 10.666 MHz 65535: 13.333 MHz"]
pub struct FREQUENCY_R(crate::FieldReader<u16, u16>);
impl FREQUENCY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FREQUENCY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREQUENCY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQUENCY` writer - Frequency 0: 10.666 MHz 65535: 13.333 MHz"]
pub struct FREQUENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQUENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Frequency 0: 10.666 MHz 65535: 13.333 MHz"]
    #[inline(always)]
    pub fn frequency(&self) -> FREQUENCY_R {
        FREQUENCY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frequency 0: 10.666 MHz 65535: 13.333 MHz"]
    #[inline(always)]
    pub fn frequency(&mut self) -> FREQUENCY_W {
        FREQUENCY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Audio PLL frequency in 11.176 MHz - 11.402 MHz or 12.165 MHz - 12.411 MHz frequency bands\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frequency](index.html) module"]
pub struct FREQUENCY_SPEC;
impl crate::RegisterSpec for FREQUENCY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frequency::R](R) reader structure"]
impl crate::Readable for FREQUENCY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frequency::W](W) writer structure"]
impl crate::Writable for FREQUENCY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FREQUENCY to value 0x9bae"]
impl crate::Resettable for FREQUENCY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x9bae
    }
}
