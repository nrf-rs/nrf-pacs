#[doc = "Register `OUTPTR` reader"]
pub struct R(crate::R<OUTPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTPTR` writer"]
pub struct W(crate::W<OUTPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTPTR_SPEC>;
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
impl From<crate::W<OUTPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTPTR` reader - Output pointer"]
pub struct OUTPTR_R(crate::FieldReader<u32, u32>);
impl OUTPTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        OUTPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTPTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTPTR` writer - Output pointer"]
pub struct OUTPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Output pointer"]
    #[inline(always)]
    pub fn outptr(&self) -> OUTPTR_R {
        OUTPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Output pointer"]
    #[inline(always)]
    pub fn outptr(&mut self) -> OUTPTR_W {
        OUTPTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outptr](index.html) module"]
pub struct OUTPTR_SPEC;
impl crate::RegisterSpec for OUTPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outptr::R](R) reader structure"]
impl crate::Readable for OUTPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outptr::W](W) writer structure"]
impl crate::Writable for OUTPTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTPTR to value 0"]
impl crate::Resettable for OUTPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
