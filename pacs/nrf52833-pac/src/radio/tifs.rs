#[doc = "Register `TIFS` reader"]
pub struct R(crate::R<TIFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIFS` writer"]
pub struct W(crate::W<TIFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIFS_SPEC>;
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
impl From<crate::W<TIFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIFS` reader - Interframe spacing in us"]
pub struct TIFS_R(crate::FieldReader<u16, u16>);
impl TIFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TIFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIFS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIFS` writer - Interframe spacing in us"]
pub struct TIFS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Interframe spacing in us"]
    #[inline(always)]
    pub fn tifs(&self) -> TIFS_R {
        TIFS_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Interframe spacing in us"]
    #[inline(always)]
    pub fn tifs(&mut self) -> TIFS_W {
        TIFS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interframe spacing in us\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tifs](index.html) module"]
pub struct TIFS_SPEC;
impl crate::RegisterSpec for TIFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tifs::R](R) reader structure"]
impl crate::Readable for TIFS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tifs::W](W) writer structure"]
impl crate::Writable for TIFS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIFS to value 0"]
impl crate::Resettable for TIFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
