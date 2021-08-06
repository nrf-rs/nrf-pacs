#[doc = "Register `ERASEUICR` writer"]
pub struct W(crate::W<ERASEUICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERASEUICR_SPEC>;
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
impl From<crate::W<ERASEUICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERASEUICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Register starting erase of all user information configuration registers. The erase must be enabled using CONFIG.WEN before the UICR can be erased.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERASEUICR_AW {
    #[doc = "0: No operation"]
    NOOPERATION = 0,
    #[doc = "1: Start erase of UICR"]
    ERASE = 1,
}
impl From<ERASEUICR_AW> for bool {
    #[inline(always)]
    fn from(variant: ERASEUICR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERASEUICR` writer - Register starting erase of all user information configuration registers. The erase must be enabled using CONFIG.WEN before the UICR can be erased."]
pub struct ERASEUICR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASEUICR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERASEUICR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(ERASEUICR_AW::NOOPERATION)
    }
    #[doc = "Start erase of UICR"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(ERASEUICR_AW::ERASE)
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
    #[doc = "Bit 0 - Register starting erase of all user information configuration registers. The erase must be enabled using CONFIG.WEN before the UICR can be erased."]
    #[inline(always)]
    pub fn eraseuicr(&mut self) -> ERASEUICR_W {
        ERASEUICR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for erasing user information configuration registers\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eraseuicr](index.html) module"]
pub struct ERASEUICR_SPEC;
impl crate::RegisterSpec for ERASEUICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [eraseuicr::W](W) writer structure"]
impl crate::Writable for ERASEUICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERASEUICR to value 0"]
impl crate::Resettable for ERASEUICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
