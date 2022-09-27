#[doc = "Register `TASKS_STARTISOIN` writer"]
pub struct W(crate::W<TASKS_STARTISOIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_STARTISOIN_SPEC>;
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
impl From<crate::W<TASKS_STARTISOIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_STARTISOIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_STARTISOIN_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_STARTISOIN_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_STARTISOIN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_STARTISOIN` writer - Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint"]
pub type TASKS_STARTISOIN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TASKS_STARTISOIN_SPEC, TASKS_STARTISOIN_AW, O>;
impl<'a, const O: u8> TASKS_STARTISOIN_W<'a, O> {
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_STARTISOIN_AW::TRIGGER)
    }
}
impl W {
    #[doc = "Bit 0 - Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint"]
    #[inline(always)]
    pub fn tasks_startisoin(&mut self) -> TASKS_STARTISOIN_W<0> {
        TASKS_STARTISOIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_startisoin](index.html) module"]
pub struct TASKS_STARTISOIN_SPEC;
impl crate::RegisterSpec for TASKS_STARTISOIN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_startisoin::W](W) writer structure"]
impl crate::Writable for TASKS_STARTISOIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TASKS_STARTISOIN to value 0"]
impl crate::Resettable for TASKS_STARTISOIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
