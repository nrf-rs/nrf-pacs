#[doc = "Register `OVERRIDE3` reader"]
pub struct R(crate::R<OVERRIDE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OVERRIDE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OVERRIDE3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OVERRIDE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OVERRIDE3` writer"]
pub struct W(crate::W<OVERRIDE3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OVERRIDE3_SPEC>;
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
impl From<crate::W<OVERRIDE3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OVERRIDE3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVERRIDE3` reader - Trim value override 3."]
pub struct OVERRIDE3_R(crate::FieldReader<u32, u32>);
impl OVERRIDE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        OVERRIDE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRIDE3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRIDE3` writer - Trim value override 3."]
pub struct OVERRIDE3_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Trim value override 3."]
    #[inline(always)]
    pub fn override3(&self) -> OVERRIDE3_R {
        OVERRIDE3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Trim value override 3."]
    #[inline(always)]
    pub fn override3(&mut self) -> OVERRIDE3_W {
        OVERRIDE3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trim value override register 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [override3](index.html) module"]
pub struct OVERRIDE3_SPEC;
impl crate::RegisterSpec for OVERRIDE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [override3::R](R) reader structure"]
impl crate::Readable for OVERRIDE3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [override3::W](W) writer structure"]
impl crate::Writable for OVERRIDE3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OVERRIDE3 to value 0"]
impl crate::Resettable for OVERRIDE3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
