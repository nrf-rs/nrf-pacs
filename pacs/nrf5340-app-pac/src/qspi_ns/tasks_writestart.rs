#[doc = "Register `TASKS_WRITESTART` writer"]
pub struct W(crate::W<TASKS_WRITESTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_WRITESTART_SPEC>;
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
impl From<crate::W<TASKS_WRITESTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_WRITESTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Start transfer from internal RAM to external flash memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_WRITESTART_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_WRITESTART_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_WRITESTART_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_WRITESTART` writer - Start transfer from internal RAM to external flash memory"]
pub type TASKS_WRITESTART_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TASKS_WRITESTART_SPEC, TASKS_WRITESTART_AW, O>;
impl<'a, const O: u8> TASKS_WRITESTART_W<'a, O> {
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_WRITESTART_AW::TRIGGER)
    }
}
impl W {
    #[doc = "Bit 0 - Start transfer from internal RAM to external flash memory"]
    #[inline(always)]
    pub fn tasks_writestart(&mut self) -> TASKS_WRITESTART_W<0> {
        TASKS_WRITESTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start transfer from internal RAM to external flash memory\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_writestart](index.html) module"]
pub struct TASKS_WRITESTART_SPEC;
impl crate::RegisterSpec for TASKS_WRITESTART_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_writestart::W](W) writer structure"]
impl crate::Writable for TASKS_WRITESTART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TASKS_WRITESTART to value 0"]
impl crate::Resettable for TASKS_WRITESTART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
