#[doc = "Register `MAXLEN` reader"]
pub struct R(crate::R<MAXLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAXLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAXLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAXLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAXLEN` writer"]
pub struct W(crate::W<MAXLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAXLEN_SPEC>;
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
impl From<crate::W<MAXLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAXLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAXLEN` reader - Size of the RAM buffer allocated to TXD and RXD data storage each"]
pub struct MAXLEN_R(crate::FieldReader<u16, u16>);
impl MAXLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MAXLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAXLEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAXLEN` writer - Size of the RAM buffer allocated to TXD and RXD data storage each"]
pub struct MAXLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Size of the RAM buffer allocated to TXD and RXD data storage each"]
    #[inline(always)]
    pub fn maxlen(&self) -> MAXLEN_R {
        MAXLEN_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Size of the RAM buffer allocated to TXD and RXD data storage each"]
    #[inline(always)]
    pub fn maxlen(&mut self) -> MAXLEN_W {
        MAXLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Size of the RAM buffer allocated to TXD and RXD data storage each\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxlen](index.html) module"]
pub struct MAXLEN_SPEC;
impl crate::RegisterSpec for MAXLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maxlen::R](R) reader structure"]
impl crate::Readable for MAXLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maxlen::W](W) writer structure"]
impl crate::Writable for MAXLEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAXLEN to value 0"]
impl crate::Resettable for MAXLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
