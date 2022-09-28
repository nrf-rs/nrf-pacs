#[doc = "Register `TASKS_ERASESTART` writer"]
pub struct W(crate::W<TASKS_ERASESTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_ERASESTART_SPEC>;
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
impl From<crate::W<TASKS_ERASESTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_ERASESTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Start external flash memory erase operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_ERASESTART_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_ERASESTART_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_ERASESTART_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_ERASESTART` writer - Start external flash memory erase operation"]
pub type TASKS_ERASESTART_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TASKS_ERASESTART_SPEC, TASKS_ERASESTART_AW, O>;
impl<'a, const O: u8> TASKS_ERASESTART_W<'a, O> {
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_ERASESTART_AW::TRIGGER)
    }
}
impl W {
    #[doc = "Bit 0 - Start external flash memory erase operation"]
    #[inline(always)]
    pub fn tasks_erasestart(&mut self) -> TASKS_ERASESTART_W<0> {
        TASKS_ERASESTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start external flash memory erase operation\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_erasestart](index.html) module"]
pub struct TASKS_ERASESTART_SPEC;
impl crate::RegisterSpec for TASKS_ERASESTART_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_erasestart::W](W) writer structure"]
impl crate::Writable for TASKS_ERASESTART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TASKS_ERASESTART to value 0"]
impl crate::Resettable for TASKS_ERASESTART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
