#[doc = "Register `END` reader"]
pub struct R(crate::R<END_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<END_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<END_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<END_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `END` writer"]
pub struct W(crate::W<END_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<END_SPEC>;
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
impl From<crate::W<END_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<END_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `END` reader - End address of region."]
pub type END_R = crate::FieldReader<u32, u32>;
#[doc = "Field `END` writer - End address of region."]
pub type END_W<'a, const O: u8> = crate::FieldWriter<'a, u32, END_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - End address of region."]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - End address of region."]
    #[inline(always)]
    pub fn end(&mut self) -> END_W<0> {
        END_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster\\[0\\]: End address of region 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [end](index.html) module"]
pub struct END_SPEC;
impl crate::RegisterSpec for END_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [end::R](R) reader structure"]
impl crate::Readable for END_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [end::W](W) writer structure"]
impl crate::Writable for END_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets END to value 0"]
impl crate::Resettable for END_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
