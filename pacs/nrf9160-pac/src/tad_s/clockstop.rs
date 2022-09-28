#[doc = "Register `CLOCKSTOP` writer"]
pub struct W(crate::W<CLOCKSTOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCKSTOP_SPEC>;
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
impl From<crate::W<CLOCKSTOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCKSTOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_AW {
    #[doc = "1: Stop all trace and debug clocks."]
    STOP = 1,
}
impl From<STOP_AW> for bool {
    #[inline(always)]
    fn from(variant: STOP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` writer - "]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLOCKSTOP_SPEC, STOP_AW, O>;
impl<'a, const O: u8> STOP_W<'a, O> {
    #[doc = "Stop all trace and debug clocks."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(STOP_AW::STOP)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<0> {
        STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stop all trace and debug clocks.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clockstop](index.html) module"]
pub struct CLOCKSTOP_SPEC;
impl crate::RegisterSpec for CLOCKSTOP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clockstop::W](W) writer structure"]
impl crate::Writable for CLOCKSTOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLOCKSTOP to value 0"]
impl crate::Resettable for CLOCKSTOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
