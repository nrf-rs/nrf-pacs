#[doc = "Register `OVERRIDE2` reader"]
pub struct R(crate::R<OVERRIDE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OVERRIDE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OVERRIDE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OVERRIDE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OVERRIDE2` writer"]
pub struct W(crate::W<OVERRIDE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OVERRIDE2_SPEC>;
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
impl From<crate::W<OVERRIDE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OVERRIDE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVERRIDE2` reader - Trim value override 2."]
pub struct OVERRIDE2_R(crate::FieldReader<u32, u32>);
impl OVERRIDE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        OVERRIDE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRIDE2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRIDE2` writer - Trim value override 2."]
pub struct OVERRIDE2_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Trim value override 2."]
    #[inline(always)]
    pub fn override2(&self) -> OVERRIDE2_R {
        OVERRIDE2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Trim value override 2."]
    #[inline(always)]
    pub fn override2(&mut self) -> OVERRIDE2_W {
        OVERRIDE2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trim value override register 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [override2](index.html) module"]
pub struct OVERRIDE2_SPEC;
impl crate::RegisterSpec for OVERRIDE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [override2::R](R) reader structure"]
impl crate::Readable for OVERRIDE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [override2::W](W) writer structure"]
impl crate::Writable for OVERRIDE2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OVERRIDE2 to value 0"]
impl crate::Resettable for OVERRIDE2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
