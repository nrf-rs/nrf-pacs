#[doc = "Register `TASKS_PWMREQSTART` writer"]
pub struct W(crate::W<TASKS_PWMREQSTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_PWMREQSTART_SPEC>;
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
impl From<crate::W<TASKS_PWMREQSTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_PWMREQSTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Request forcing PWM mode in external DC/DC voltage regulator. (Drives FPWM_DCDC pin high or low depending on a setting in UICR).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_PWMREQSTART_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_PWMREQSTART_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_PWMREQSTART_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_PWMREQSTART` writer - Request forcing PWM mode in external DC/DC voltage regulator. (Drives FPWM_DCDC pin high or low depending on a setting in UICR)."]
pub type TASKS_PWMREQSTART_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TASKS_PWMREQSTART_SPEC, TASKS_PWMREQSTART_AW, O>;
impl<'a, const O: u8> TASKS_PWMREQSTART_W<'a, O> {
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_PWMREQSTART_AW::TRIGGER)
    }
}
impl W {
    #[doc = "Bit 0 - Request forcing PWM mode in external DC/DC voltage regulator. (Drives FPWM_DCDC pin high or low depending on a setting in UICR)."]
    #[inline(always)]
    pub fn tasks_pwmreqstart(&mut self) -> TASKS_PWMREQSTART_W<0> {
        TASKS_PWMREQSTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Request forcing PWM mode in external DC/DC voltage regulator. (Drives FPWM_DCDC pin high or low depending on a setting in UICR).\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_pwmreqstart](index.html) module"]
pub struct TASKS_PWMREQSTART_SPEC;
impl crate::RegisterSpec for TASKS_PWMREQSTART_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_pwmreqstart::W](W) writer structure"]
impl crate::Writable for TASKS_PWMREQSTART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TASKS_PWMREQSTART to value 0"]
impl crate::Resettable for TASKS_PWMREQSTART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
