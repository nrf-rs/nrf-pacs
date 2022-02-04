#[doc = "Register `OVERRIDE0` reader"]
pub struct R(crate::R<OVERRIDE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OVERRIDE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OVERRIDE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OVERRIDE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OVERRIDE0` writer"]
pub struct W(crate::W<OVERRIDE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OVERRIDE0_SPEC>;
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
impl From<crate::W<OVERRIDE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OVERRIDE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVERRIDE0` reader - Trim value override 0."]
pub struct OVERRIDE0_R(crate::FieldReader<u32, u32>);
impl OVERRIDE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        OVERRIDE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRIDE0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRIDE0` writer - Trim value override 0."]
pub struct OVERRIDE0_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Trim value override 0."]
    #[inline(always)]
    pub fn override0(&self) -> OVERRIDE0_R {
        OVERRIDE0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Trim value override 0."]
    #[inline(always)]
    pub fn override0(&mut self) -> OVERRIDE0_W {
        OVERRIDE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trim value override register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [override0](index.html) module"]
pub struct OVERRIDE0_SPEC;
impl crate::RegisterSpec for OVERRIDE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [override0::R](R) reader structure"]
impl crate::Readable for OVERRIDE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [override0::W](W) writer structure"]
impl crate::Writable for OVERRIDE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OVERRIDE0 to value 0"]
impl crate::Resettable for OVERRIDE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
