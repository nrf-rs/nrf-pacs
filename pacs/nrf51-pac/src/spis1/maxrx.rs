#[doc = "Register `MAXRX` reader"]
pub struct R(crate::R<MAXRX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAXRX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAXRX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAXRX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAXRX` writer"]
pub struct W(crate::W<MAXRX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAXRX_SPEC>;
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
impl From<crate::W<MAXRX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAXRX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAXRX` reader - Maximum number of bytes in the receive buffer."]
pub struct MAXRX_R(crate::FieldReader<u8, u8>);
impl MAXRX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAXRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAXRX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAXRX` writer - Maximum number of bytes in the receive buffer."]
pub struct MAXRX_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXRX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum number of bytes in the receive buffer."]
    #[inline(always)]
    pub fn maxrx(&self) -> MAXRX_R {
        MAXRX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum number of bytes in the receive buffer."]
    #[inline(always)]
    pub fn maxrx(&mut self) -> MAXRX_W {
        MAXRX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Maximum number of bytes in the receive buffer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxrx](index.html) module"]
pub struct MAXRX_SPEC;
impl crate::RegisterSpec for MAXRX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maxrx::R](R) reader structure"]
impl crate::Readable for MAXRX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maxrx::W](W) writer structure"]
impl crate::Writable for MAXRX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAXRX to value 0"]
impl crate::Resettable for MAXRX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
