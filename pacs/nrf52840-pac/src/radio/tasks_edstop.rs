#[doc = "Register `TASKS_EDSTOP` writer"]
pub struct W(crate::W<TASKS_EDSTOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_EDSTOP_SPEC>;
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
impl From<crate::W<TASKS_EDSTOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_EDSTOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TASKS_EDSTOP` writer - "]
pub struct TASKS_EDSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TASKS_EDSTOP_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tasks_edstop(&mut self) -> TASKS_EDSTOP_W {
        TASKS_EDSTOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stop the energy detect measurement\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_edstop](index.html) module"]
pub struct TASKS_EDSTOP_SPEC;
impl crate::RegisterSpec for TASKS_EDSTOP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_edstop::W](W) writer structure"]
impl crate::Writable for TASKS_EDSTOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TASKS_EDSTOP to value 0"]
impl crate::Resettable for TASKS_EDSTOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
