#[doc = "Register `SRC` reader"]
pub struct R(crate::R<SRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRC` writer"]
pub struct W(crate::W<SRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRC_SPEC>;
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
impl From<crate::W<SRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC` reader - Word-aligned RAM source address."]
pub struct SRC_R(crate::FieldReader<u32, u32>);
impl SRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC` writer - Word-aligned RAM source address."]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Word-aligned RAM source address."]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Word-aligned RAM source address."]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM source address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src](index.html) module"]
pub struct SRC_SPEC;
impl crate::RegisterSpec for SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [src::R](R) reader structure"]
impl crate::Readable for SRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [src::W](W) writer structure"]
impl crate::Writable for SRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRC to value 0"]
impl crate::Resettable for SRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
