#[doc = "Register `ERASEPAGEPARTIAL` reader"]
pub struct R(crate::R<ERASEPAGEPARTIAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERASEPAGEPARTIAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERASEPAGEPARTIAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERASEPAGEPARTIAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERASEPAGEPARTIAL` writer"]
pub struct W(crate::W<ERASEPAGEPARTIAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERASEPAGEPARTIAL_SPEC>;
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
impl From<crate::W<ERASEPAGEPARTIAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERASEPAGEPARTIAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERASEPAGEPARTIAL` reader - Register for starting partial erase of a page in code area"]
pub struct ERASEPAGEPARTIAL_R(crate::FieldReader<u32, u32>);
impl ERASEPAGEPARTIAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ERASEPAGEPARTIAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERASEPAGEPARTIAL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERASEPAGEPARTIAL` writer - Register for starting partial erase of a page in code area"]
pub struct ERASEPAGEPARTIAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASEPAGEPARTIAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Register for starting partial erase of a page in code area"]
    #[inline(always)]
    pub fn erasepagepartial(&self) -> ERASEPAGEPARTIAL_R {
        ERASEPAGEPARTIAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register for starting partial erase of a page in code area"]
    #[inline(always)]
    pub fn erasepagepartial(&mut self) -> ERASEPAGEPARTIAL_W {
        ERASEPAGEPARTIAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for partial erase of a page in code area\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erasepagepartial](index.html) module"]
pub struct ERASEPAGEPARTIAL_SPEC;
impl crate::RegisterSpec for ERASEPAGEPARTIAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [erasepagepartial::R](R) reader structure"]
impl crate::Readable for ERASEPAGEPARTIAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [erasepagepartial::W](W) writer structure"]
impl crate::Writable for ERASEPAGEPARTIAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERASEPAGEPARTIAL to value 0"]
impl crate::Resettable for ERASEPAGEPARTIAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
