#[doc = "Register `SWITCHPATTERN` reader"]
pub struct R(crate::R<SWITCHPATTERN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWITCHPATTERN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWITCHPATTERN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWITCHPATTERN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWITCHPATTERN` writer"]
pub struct W(crate::W<SWITCHPATTERN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWITCHPATTERN_SPEC>;
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
impl From<crate::W<SWITCHPATTERN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWITCHPATTERN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWITCHPATTERN` reader - Fill array of GPIO patterns for antenna control"]
pub struct SWITCHPATTERN_R(crate::FieldReader<u8, u8>);
impl SWITCHPATTERN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SWITCHPATTERN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWITCHPATTERN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWITCHPATTERN` writer - Fill array of GPIO patterns for antenna control"]
pub struct SWITCHPATTERN_W<'a> {
    w: &'a mut W,
}
impl<'a> SWITCHPATTERN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Fill array of GPIO patterns for antenna control"]
    #[inline(always)]
    pub fn switchpattern(&self) -> SWITCHPATTERN_R {
        SWITCHPATTERN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fill array of GPIO patterns for antenna control"]
    #[inline(always)]
    pub fn switchpattern(&mut self) -> SWITCHPATTERN_W {
        SWITCHPATTERN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO patterns to be used for each antenna\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [switchpattern](index.html) module"]
pub struct SWITCHPATTERN_SPEC;
impl crate::RegisterSpec for SWITCHPATTERN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [switchpattern::R](R) reader structure"]
impl crate::Readable for SWITCHPATTERN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [switchpattern::W](W) writer structure"]
impl crate::Writable for SWITCHPATTERN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWITCHPATTERN to value 0"]
impl crate::Resettable for SWITCHPATTERN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
