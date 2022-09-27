#[doc = "Register `ERASEALL` writer"]
pub struct W(crate::W<ERASEALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERASEALL_SPEC>;
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
impl From<crate::W<ERASEALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERASEALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Erase all non-volatile memory including UICR registers. Note that erasing must be enabled by setting CONFIG.WEN = Een before the non-volatile memory can be erased.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERASEALL_AW {
    #[doc = "0: No operation"]
    NO_OPERATION = 0,
    #[doc = "1: Start chip erase"]
    ERASE = 1,
}
impl From<ERASEALL_AW> for bool {
    #[inline(always)]
    fn from(variant: ERASEALL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERASEALL` writer - Erase all non-volatile memory including UICR registers. Note that erasing must be enabled by setting CONFIG.WEN = Een before the non-volatile memory can be erased."]
pub type ERASEALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERASEALL_SPEC, ERASEALL_AW, O>;
impl<'a, const O: u8> ERASEALL_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(ERASEALL_AW::NO_OPERATION)
    }
    #[doc = "Start chip erase"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(ERASEALL_AW::ERASE)
    }
}
impl W {
    #[doc = "Bit 0 - Erase all non-volatile memory including UICR registers. Note that erasing must be enabled by setting CONFIG.WEN = Een before the non-volatile memory can be erased."]
    #[inline(always)]
    pub fn eraseall(&mut self) -> ERASEALL_W<0> {
        ERASEALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for erasing all non-volatile user memory\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eraseall](index.html) module"]
pub struct ERASEALL_SPEC;
impl crate::RegisterSpec for ERASEALL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [eraseall::W](W) writer structure"]
impl crate::Writable for ERASEALL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERASEALL to value 0"]
impl crate::Resettable for ERASEALL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
