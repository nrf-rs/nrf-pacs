#[doc = "Register `EPOUT[%s]` reader"]
pub struct R(crate::R<EPOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPOUT[%s]` writer"]
pub struct W(crate::W<EPOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPOUT_SPEC>;
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
impl From<crate::W<EPOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIZE` reader - Number of bytes received last in the data stage of this OUT endpoint"]
pub struct SIZE_R(crate::FieldReader<u8, u8>);
impl SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIZE` writer - Number of bytes received last in the data stage of this OUT endpoint"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Number of bytes received last in the data stage of this OUT endpoint"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Number of bytes received last in the data stage of this OUT endpoint"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Number of bytes received last in the data stage of this OUT endpoint\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epout](index.html) module"]
pub struct EPOUT_SPEC;
impl crate::RegisterSpec for EPOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epout::R](R) reader structure"]
impl crate::Readable for EPOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epout::W](W) writer structure"]
impl crate::Writable for EPOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPOUT[%s]
to value 0"]
impl crate::Resettable for EPOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
