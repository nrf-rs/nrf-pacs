#[doc = "Register `TASKS_READCLRACC` writer"]
pub struct W(crate::W<TASKS_READCLRACC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_READCLRACC_SPEC>;
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
impl From<crate::W<TASKS_READCLRACC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_READCLRACC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Read and clear ACC and ACCDBL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_READCLRACC_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_READCLRACC_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_READCLRACC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_READCLRACC` writer - Read and clear ACC and ACCDBL"]
pub type TASKS_READCLRACC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TASKS_READCLRACC_SPEC, TASKS_READCLRACC_AW, O>;
impl<'a, const O: u8> TASKS_READCLRACC_W<'a, O> {
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_READCLRACC_AW::TRIGGER)
    }
}
impl W {
    #[doc = "Bit 0 - Read and clear ACC and ACCDBL"]
    #[inline(always)]
    pub fn tasks_readclracc(&mut self) -> TASKS_READCLRACC_W<0> {
        TASKS_READCLRACC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read and clear ACC and ACCDBL\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_readclracc](index.html) module"]
pub struct TASKS_READCLRACC_SPEC;
impl crate::RegisterSpec for TASKS_READCLRACC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_readclracc::W](W) writer structure"]
impl crate::Writable for TASKS_READCLRACC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TASKS_READCLRACC to value 0"]
impl crate::Resettable for TASKS_READCLRACC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
