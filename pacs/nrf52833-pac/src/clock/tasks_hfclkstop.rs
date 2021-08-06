#[doc = "Register `TASKS_HFCLKSTOP` writer"]
pub struct W(crate::W<TASKS_HFCLKSTOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_HFCLKSTOP_SPEC>;
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
impl From<crate::W<TASKS_HFCLKSTOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_HFCLKSTOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Stop HFXO crystal oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_HFCLKSTOP_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_HFCLKSTOP_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_HFCLKSTOP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_HFCLKSTOP` writer - Stop HFXO crystal oscillator"]
pub struct TASKS_HFCLKSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TASKS_HFCLKSTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASKS_HFCLKSTOP_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_HFCLKSTOP_AW::TRIGGER)
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
    #[doc = "Bit 0 - Stop HFXO crystal oscillator"]
    #[inline(always)]
    pub fn tasks_hfclkstop(&mut self) -> TASKS_HFCLKSTOP_W {
        TASKS_HFCLKSTOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stop HFXO crystal oscillator\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_hfclkstop](index.html) module"]
pub struct TASKS_HFCLKSTOP_SPEC;
impl crate::RegisterSpec for TASKS_HFCLKSTOP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_hfclkstop::W](W) writer structure"]
impl crate::Writable for TASKS_HFCLKSTOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TASKS_HFCLKSTOP to value 0"]
impl crate::Resettable for TASKS_HFCLKSTOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
