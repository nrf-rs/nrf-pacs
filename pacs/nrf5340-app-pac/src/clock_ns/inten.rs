#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HFCLKSTARTED` reader - Enable or disable interrupt for event HFCLKSTARTED"]
pub type HFCLKSTARTED_R = crate::BitReader<HFCLKSTARTED_A>;
#[doc = "Enable or disable interrupt for event HFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFCLKSTARTED_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<HFCLKSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: HFCLKSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
impl HFCLKSTARTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFCLKSTARTED_A {
        match self.bits {
            false => HFCLKSTARTED_A::DISABLED,
            true => HFCLKSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HFCLKSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HFCLKSTARTED_A::ENABLED
    }
}
#[doc = "Field `HFCLKSTARTED` writer - Enable or disable interrupt for event HFCLKSTARTED"]
pub type HFCLKSTARTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, HFCLKSTARTED_A, O>;
impl<'a, const O: u8> HFCLKSTARTED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HFCLKSTARTED_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HFCLKSTARTED_A::ENABLED)
    }
}
#[doc = "Field `LFCLKSTARTED` reader - Enable or disable interrupt for event LFCLKSTARTED"]
pub type LFCLKSTARTED_R = crate::BitReader<LFCLKSTARTED_A>;
#[doc = "Enable or disable interrupt for event LFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFCLKSTARTED_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<LFCLKSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: LFCLKSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
impl LFCLKSTARTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFCLKSTARTED_A {
        match self.bits {
            false => LFCLKSTARTED_A::DISABLED,
            true => LFCLKSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFCLKSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LFCLKSTARTED_A::ENABLED
    }
}
#[doc = "Field `LFCLKSTARTED` writer - Enable or disable interrupt for event LFCLKSTARTED"]
pub type LFCLKSTARTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, LFCLKSTARTED_A, O>;
impl<'a, const O: u8> LFCLKSTARTED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFCLKSTARTED_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LFCLKSTARTED_A::ENABLED)
    }
}
#[doc = "Field `DONE` reader - Enable or disable interrupt for event DONE"]
pub type DONE_R = crate::BitReader<DONE_A>;
#[doc = "Enable or disable interrupt for event DONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DONE_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE_A {
        match self.bits {
            false => DONE_A::DISABLED,
            true => DONE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DONE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DONE_A::ENABLED
    }
}
#[doc = "Field `DONE` writer - Enable or disable interrupt for event DONE"]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, DONE_A, O>;
impl<'a, const O: u8> DONE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DONE_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DONE_A::ENABLED)
    }
}
#[doc = "Field `HFCLKAUDIOSTARTED` reader - Enable or disable interrupt for event HFCLKAUDIOSTARTED"]
pub type HFCLKAUDIOSTARTED_R = crate::BitReader<HFCLKAUDIOSTARTED_A>;
#[doc = "Enable or disable interrupt for event HFCLKAUDIOSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFCLKAUDIOSTARTED_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<HFCLKAUDIOSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: HFCLKAUDIOSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
impl HFCLKAUDIOSTARTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFCLKAUDIOSTARTED_A {
        match self.bits {
            false => HFCLKAUDIOSTARTED_A::DISABLED,
            true => HFCLKAUDIOSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HFCLKAUDIOSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HFCLKAUDIOSTARTED_A::ENABLED
    }
}
#[doc = "Field `HFCLKAUDIOSTARTED` writer - Enable or disable interrupt for event HFCLKAUDIOSTARTED"]
pub type HFCLKAUDIOSTARTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTEN_SPEC, HFCLKAUDIOSTARTED_A, O>;
impl<'a, const O: u8> HFCLKAUDIOSTARTED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HFCLKAUDIOSTARTED_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HFCLKAUDIOSTARTED_A::ENABLED)
    }
}
#[doc = "Field `HFCLK192MSTARTED` reader - Enable or disable interrupt for event HFCLK192MSTARTED"]
pub type HFCLK192MSTARTED_R = crate::BitReader<HFCLK192MSTARTED_A>;
#[doc = "Enable or disable interrupt for event HFCLK192MSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFCLK192MSTARTED_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<HFCLK192MSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: HFCLK192MSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
impl HFCLK192MSTARTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFCLK192MSTARTED_A {
        match self.bits {
            false => HFCLK192MSTARTED_A::DISABLED,
            true => HFCLK192MSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HFCLK192MSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HFCLK192MSTARTED_A::ENABLED
    }
}
#[doc = "Field `HFCLK192MSTARTED` writer - Enable or disable interrupt for event HFCLK192MSTARTED"]
pub type HFCLK192MSTARTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTEN_SPEC, HFCLK192MSTARTED_A, O>;
impl<'a, const O: u8> HFCLK192MSTARTED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HFCLK192MSTARTED_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HFCLK192MSTARTED_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event HFCLKSTARTED"]
    #[inline(always)]
    pub fn hfclkstarted(&self) -> HFCLKSTARTED_R {
        HFCLKSTARTED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn lfclkstarted(&self) -> LFCLKSTARTED_R {
        LFCLKSTARTED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event DONE"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for event HFCLKAUDIOSTARTED"]
    #[inline(always)]
    pub fn hfclkaudiostarted(&self) -> HFCLKAUDIOSTARTED_R {
        HFCLKAUDIOSTARTED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event HFCLK192MSTARTED"]
    #[inline(always)]
    pub fn hfclk192mstarted(&self) -> HFCLK192MSTARTED_R {
        HFCLK192MSTARTED_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event HFCLKSTARTED"]
    #[inline(always)]
    pub fn hfclkstarted(&mut self) -> HFCLKSTARTED_W<0> {
        HFCLKSTARTED_W::new(self)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn lfclkstarted(&mut self) -> LFCLKSTARTED_W<1> {
        LFCLKSTARTED_W::new(self)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event DONE"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W<7> {
        DONE_W::new(self)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for event HFCLKAUDIOSTARTED"]
    #[inline(always)]
    pub fn hfclkaudiostarted(&mut self) -> HFCLKAUDIOSTARTED_W<8> {
        HFCLKAUDIOSTARTED_W::new(self)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event HFCLK192MSTARTED"]
    #[inline(always)]
    pub fn hfclk192mstarted(&mut self) -> HFCLK192MSTARTED_W<9> {
        HFCLK192MSTARTED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable or disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
