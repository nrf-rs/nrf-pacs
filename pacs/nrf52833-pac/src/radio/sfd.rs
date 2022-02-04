#[doc = "Register `SFD` reader"]
pub struct R(crate::R<SFD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFD` writer"]
pub struct W(crate::W<SFD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFD_SPEC>;
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
impl From<crate::W<SFD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFD` reader - IEEE 802.15.4 start of frame delimiter"]
pub struct SFD_R(crate::FieldReader<u8, u8>);
impl SFD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SFD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFD` writer - IEEE 802.15.4 start of frame delimiter"]
pub struct SFD_W<'a> {
    w: &'a mut W,
}
impl<'a> SFD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - IEEE 802.15.4 start of frame delimiter"]
    #[inline(always)]
    pub fn sfd(&self) -> SFD_R {
        SFD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IEEE 802.15.4 start of frame delimiter"]
    #[inline(always)]
    pub fn sfd(&mut self) -> SFD_W {
        SFD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IEEE 802.15.4 start of frame delimiter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfd](index.html) module"]
pub struct SFD_SPEC;
impl crate::RegisterSpec for SFD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfd::R](R) reader structure"]
impl crate::Readable for SFD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfd::W](W) writer structure"]
impl crate::Writable for SFD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SFD to value 0xa7"]
impl crate::Resettable for SFD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa7
    }
}
