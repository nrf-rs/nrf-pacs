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
pub type SWITCHPATTERN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SWITCHPATTERN` writer - Fill array of GPIO patterns for antenna control"]
pub type SWITCHPATTERN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SWITCHPATTERN_SPEC, u8, u8, 8, O>;
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
    pub fn switchpattern(&mut self) -> SWITCHPATTERN_W<0> {
        SWITCHPATTERN_W::new(self)
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
