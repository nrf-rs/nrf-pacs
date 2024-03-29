#[doc = "Register `INVALIDATE` writer"]
pub struct W(crate::W<INVALIDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INVALIDATE_SPEC>;
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
impl From<crate::W<INVALIDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INVALIDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Invalidate the cache\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVALIDATE_AW {
    #[doc = "1: Invalidate the cache"]
    INVALIDATE = 1,
}
impl From<INVALIDATE_AW> for bool {
    #[inline(always)]
    fn from(variant: INVALIDATE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVALIDATE` writer - Invalidate the cache"]
pub type INVALIDATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INVALIDATE_SPEC, INVALIDATE_AW, O>;
impl<'a, const O: u8> INVALIDATE_W<'a, O> {
    #[doc = "Invalidate the cache"]
    #[inline(always)]
    pub fn invalidate(self) -> &'a mut W {
        self.variant(INVALIDATE_AW::INVALIDATE)
    }
}
impl W {
    #[doc = "Bit 0 - Invalidate the cache"]
    #[inline(always)]
    pub fn invalidate(&mut self) -> INVALIDATE_W<0> {
        INVALIDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Invalidate the cache.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [invalidate](index.html) module"]
pub struct INVALIDATE_SPEC;
impl crate::RegisterSpec for INVALIDATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [invalidate::W](W) writer structure"]
impl crate::Writable for INVALIDATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INVALIDATE to value 0"]
impl crate::Resettable for INVALIDATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
