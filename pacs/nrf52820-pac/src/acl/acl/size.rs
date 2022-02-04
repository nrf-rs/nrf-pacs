#[doc = "Register `SIZE` reader"]
pub struct R(crate::R<SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIZE` writer"]
pub struct W(crate::W<SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIZE_SPEC>;
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
impl From<crate::W<SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIZE` reader - Size of flash region n in bytes. Must be a multiple of the flash page size."]
pub struct SIZE_R(crate::FieldReader<u32, u32>);
impl SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIZE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIZE` writer - Size of flash region n in bytes. Must be a multiple of the flash page size."]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Size of flash region n in bytes. Must be a multiple of the flash page size."]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Size of flash region n in bytes. Must be a multiple of the flash page size."]
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
#[doc = "Description cluster: Size of region to protect counting from address ACL\\[n\\].ADDR. Write '0' as no effect.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [size](index.html) module"]
pub struct SIZE_SPEC;
impl crate::RegisterSpec for SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [size::R](R) reader structure"]
impl crate::Readable for SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [size::W](W) writer structure"]
impl crate::Writable for SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIZE to value 0"]
impl crate::Resettable for SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
