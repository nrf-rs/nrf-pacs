#[doc = "Register `PROFILINGCLEAR` writer"]
pub struct W(crate::W<PROFILINGCLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROFILINGCLEAR_SPEC>;
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
impl From<crate::W<PROFILINGCLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROFILINGCLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clearing the profiling counters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLEAR_AW {
    #[doc = "1: Clear the profiling counters"]
    CLEAR = 1,
}
impl From<CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLEAR` writer - Clearing the profiling counters"]
pub type CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROFILINGCLEAR_SPEC, CLEAR_AW, O>;
impl<'a, const O: u8> CLEAR_W<'a, O> {
    #[doc = "Clear the profiling counters"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CLEAR_AW::CLEAR)
    }
}
impl W {
    #[doc = "Bit 0 - Clearing the profiling counters"]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W<0> {
        CLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear the profiling counters.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [profilingclear](index.html) module"]
pub struct PROFILINGCLEAR_SPEC;
impl crate::RegisterSpec for PROFILINGCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [profilingclear::W](W) writer structure"]
impl crate::Writable for PROFILINGCLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PROFILINGCLEAR to value 0"]
impl crate::Resettable for PROFILINGCLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
