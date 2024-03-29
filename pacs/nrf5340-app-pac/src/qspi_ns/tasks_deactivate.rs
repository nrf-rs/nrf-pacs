#[doc = "Register `TASKS_DEACTIVATE` writer"]
pub struct W(crate::W<TASKS_DEACTIVATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_DEACTIVATE_SPEC>;
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
impl From<crate::W<TASKS_DEACTIVATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_DEACTIVATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Deactivate QSPI interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_DEACTIVATE_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_DEACTIVATE_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_DEACTIVATE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_DEACTIVATE` writer - Deactivate QSPI interface"]
pub type TASKS_DEACTIVATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TASKS_DEACTIVATE_SPEC, TASKS_DEACTIVATE_AW, O>;
impl<'a, const O: u8> TASKS_DEACTIVATE_W<'a, O> {
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_DEACTIVATE_AW::TRIGGER)
    }
}
impl W {
    #[doc = "Bit 0 - Deactivate QSPI interface"]
    #[inline(always)]
    pub fn tasks_deactivate(&mut self) -> TASKS_DEACTIVATE_W<0> {
        TASKS_DEACTIVATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deactivate QSPI interface\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_deactivate](index.html) module"]
pub struct TASKS_DEACTIVATE_SPEC;
impl crate::RegisterSpec for TASKS_DEACTIVATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_deactivate::W](W) writer structure"]
impl crate::Writable for TASKS_DEACTIVATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TASKS_DEACTIVATE to value 0"]
impl crate::Resettable for TASKS_DEACTIVATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
