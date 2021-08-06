#[doc = "Register `TASKS_ACTIVATE` writer"]
pub struct W(crate::W<TASKS_ACTIVATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_ACTIVATE_SPEC>;
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
impl From<crate::W<TASKS_ACTIVATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_ACTIVATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Activate NFCT peripheral for incoming and outgoing frames, change state to activated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_ACTIVATE_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_ACTIVATE_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_ACTIVATE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_ACTIVATE` writer - Activate NFCT peripheral for incoming and outgoing frames, change state to activated"]
pub struct TASKS_ACTIVATE_W<'a> {
    w: &'a mut W,
}
impl<'a> TASKS_ACTIVATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASKS_ACTIVATE_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_ACTIVATE_AW::TRIGGER)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Activate NFCT peripheral for incoming and outgoing frames, change state to activated"]
    #[inline(always)]
    pub fn tasks_activate(&mut self) -> TASKS_ACTIVATE_W {
        TASKS_ACTIVATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Activate NFCT peripheral for incoming and outgoing frames, change state to activated\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_activate](index.html) module"]
pub struct TASKS_ACTIVATE_SPEC;
impl crate::RegisterSpec for TASKS_ACTIVATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_activate::W](W) writer structure"]
impl crate::Writable for TASKS_ACTIVATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TASKS_ACTIVATE to value 0"]
impl crate::Resettable for TASKS_ACTIVATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
