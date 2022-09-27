#[doc = "Register `TASKS_NEXTSTEP` writer"]
pub struct W(crate::W<TASKS_NEXTSTEP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_NEXTSTEP_SPEC>;
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
impl From<crate::W<TASKS_NEXTSTEP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_NEXTSTEP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TASKS_NEXTSTEP` writer - "]
pub type TASKS_NEXTSTEP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TASKS_NEXTSTEP_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tasks_nextstep(&mut self) -> TASKS_NEXTSTEP_W<0> {
        TASKS_NEXTSTEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_nextstep](index.html) module"]
pub struct TASKS_NEXTSTEP_SPEC;
impl crate::RegisterSpec for TASKS_NEXTSTEP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_nextstep::W](W) writer structure"]
impl crate::Writable for TASKS_NEXTSTEP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TASKS_NEXTSTEP to value 0"]
impl crate::Resettable for TASKS_NEXTSTEP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
