#[doc = "Register `TASKS_BCSTART` writer"]
pub struct W(crate::W<TASKS_BCSTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_BCSTART_SPEC>;
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
impl From<crate::W<TASKS_BCSTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_BCSTART_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start the bit counter.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_bcstart](index.html) module"]
pub struct TASKS_BCSTART_SPEC;
impl crate::RegisterSpec for TASKS_BCSTART_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_bcstart::W](W) writer structure"]
impl crate::Writable for TASKS_BCSTART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TASKS_BCSTART to value 0"]
impl crate::Resettable for TASKS_BCSTART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
