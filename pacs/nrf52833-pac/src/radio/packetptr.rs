#[doc = "Register `PACKETPTR` reader"]
pub struct R(crate::R<PACKETPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PACKETPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PACKETPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PACKETPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PACKETPTR` writer"]
pub struct W(crate::W<PACKETPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PACKETPTR_SPEC>;
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
impl From<crate::W<PACKETPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PACKETPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PACKETPTR` reader - Packet pointer"]
pub struct PACKETPTR_R(crate::FieldReader<u32, u32>);
impl PACKETPTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PACKETPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PACKETPTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PACKETPTR` writer - Packet pointer"]
pub struct PACKETPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PACKETPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Packet pointer"]
    #[inline(always)]
    pub fn packetptr(&self) -> PACKETPTR_R {
        PACKETPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Packet pointer"]
    #[inline(always)]
    pub fn packetptr(&mut self) -> PACKETPTR_W {
        PACKETPTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Packet pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packetptr](index.html) module"]
pub struct PACKETPTR_SPEC;
impl crate::RegisterSpec for PACKETPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [packetptr::R](R) reader structure"]
impl crate::Readable for PACKETPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [packetptr::W](W) writer structure"]
impl crate::Writable for PACKETPTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PACKETPTR to value 0"]
impl crate::Resettable for PACKETPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
