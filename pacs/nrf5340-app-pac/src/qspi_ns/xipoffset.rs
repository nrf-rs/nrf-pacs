#[doc = "Register `XIPOFFSET` reader"]
pub struct R(crate::R<XIPOFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XIPOFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XIPOFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XIPOFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XIPOFFSET` writer"]
pub struct W(crate::W<XIPOFFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XIPOFFSET_SPEC>;
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
impl From<crate::W<XIPOFFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XIPOFFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XIPOFFSET` reader - Address offset into the external memory for Execute in Place operation. Value must be a multiple of 4."]
pub type XIPOFFSET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `XIPOFFSET` writer - Address offset into the external memory for Execute in Place operation. Value must be a multiple of 4."]
pub type XIPOFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XIPOFFSET_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Address offset into the external memory for Execute in Place operation. Value must be a multiple of 4."]
    #[inline(always)]
    pub fn xipoffset(&self) -> XIPOFFSET_R {
        XIPOFFSET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address offset into the external memory for Execute in Place operation. Value must be a multiple of 4."]
    #[inline(always)]
    pub fn xipoffset(&mut self) -> XIPOFFSET_W<0> {
        XIPOFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Address offset into the external memory for Execute in Place operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xipoffset](index.html) module"]
pub struct XIPOFFSET_SPEC;
impl crate::RegisterSpec for XIPOFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xipoffset::R](R) reader structure"]
impl crate::Readable for XIPOFFSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xipoffset::W](W) writer structure"]
impl crate::Writable for XIPOFFSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XIPOFFSET to value 0"]
impl crate::Resettable for XIPOFFSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
