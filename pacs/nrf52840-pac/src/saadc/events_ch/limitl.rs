#[doc = "Register `LIMITL` reader"]
pub struct R(crate::R<LIMITL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIMITL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIMITL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIMITL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LIMITL` writer"]
pub struct W(crate::W<LIMITL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIMITL_SPEC>;
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
impl From<crate::W<LIMITL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIMITL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LIMITL` reader - "]
pub type LIMITL_R = crate::BitReader<bool>;
#[doc = "Field `LIMITL` writer - "]
pub type LIMITL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIMITL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn limitl(&self) -> LIMITL_R {
        LIMITL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn limitl(&mut self) -> LIMITL_W<0> {
        LIMITL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster\\[n\\]: Last result is equal or below CH\\[n\\].LIMIT.LOW\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [limitl](index.html) module"]
pub struct LIMITL_SPEC;
impl crate::RegisterSpec for LIMITL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [limitl::R](R) reader structure"]
impl crate::Readable for LIMITL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [limitl::W](W) writer structure"]
impl crate::Writable for LIMITL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LIMITL to value 0"]
impl crate::Resettable for LIMITL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
