#[doc = "Register `GPMEM[%s]` reader"]
pub struct R(crate::R<GPMEM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPMEM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPMEM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPMEM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPMEM[%s]` writer"]
pub struct W(crate::W<GPMEM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPMEM_SPEC>;
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
impl From<crate::W<GPMEM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPMEM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPMEM` reader - General purpose memory"]
pub struct GPMEM_R(crate::FieldReader<u32, u32>);
impl GPMEM_R {
    pub(crate) fn new(bits: u32) -> Self {
        GPMEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPMEM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPMEM` writer - General purpose memory"]
pub struct GPMEM_W<'a> {
    w: &'a mut W,
}
impl<'a> GPMEM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - General purpose memory"]
    #[inline(always)]
    pub fn gpmem(&self) -> GPMEM_R {
        GPMEM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - General purpose memory"]
    #[inline(always)]
    pub fn gpmem(&mut self) -> GPMEM_W {
        GPMEM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: General purpose memory\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpmem](index.html) module"]
pub struct GPMEM_SPEC;
impl crate::RegisterSpec for GPMEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpmem::R](R) reader structure"]
impl crate::Readable for GPMEM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpmem::W](W) writer structure"]
impl crate::Writable for GPMEM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPMEM[%s]
to value 0"]
impl crate::Resettable for GPMEM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
