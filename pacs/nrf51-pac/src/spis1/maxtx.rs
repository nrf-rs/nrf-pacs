#[doc = "Register `MAXTX` reader"]
pub struct R(crate::R<MAXTX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAXTX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAXTX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAXTX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAXTX` writer"]
pub struct W(crate::W<MAXTX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAXTX_SPEC>;
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
impl From<crate::W<MAXTX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAXTX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAXTX` reader - Maximum number of bytes in the transmit buffer."]
pub struct MAXTX_R(crate::FieldReader<u8, u8>);
impl MAXTX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAXTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAXTX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAXTX` writer - Maximum number of bytes in the transmit buffer."]
pub struct MAXTX_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXTX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum number of bytes in the transmit buffer."]
    #[inline(always)]
    pub fn maxtx(&self) -> MAXTX_R {
        MAXTX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum number of bytes in the transmit buffer."]
    #[inline(always)]
    pub fn maxtx(&mut self) -> MAXTX_W {
        MAXTX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Maximum number of bytes in the transmit buffer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxtx](index.html) module"]
pub struct MAXTX_SPEC;
impl crate::RegisterSpec for MAXTX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maxtx::R](R) reader structure"]
impl crate::Readable for MAXTX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maxtx::W](W) writer structure"]
impl crate::Writable for MAXTX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAXTX to value 0"]
impl crate::Resettable for MAXTX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
