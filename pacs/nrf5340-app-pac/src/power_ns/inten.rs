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
#[doc = "Field `POFWARN` reader - Enable or disable interrupt for event POFWARN"]
pub type POFWARN_R = crate::BitReader<POFWARN_A>;
#[doc = "Enable or disable interrupt for event POFWARN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POFWARN_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<POFWARN_A> for bool {
    #[inline(always)]
    fn from(variant: POFWARN_A) -> Self {
        variant as u8 != 0
    }
}
impl POFWARN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POFWARN_A {
        match self.bits {
            false => POFWARN_A::DISABLED,
            true => POFWARN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == POFWARN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == POFWARN_A::ENABLED
    }
}
#[doc = "Field `POFWARN` writer - Enable or disable interrupt for event POFWARN"]
pub type POFWARN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, POFWARN_A, O>;
impl<'a, const O: u8> POFWARN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(POFWARN_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(POFWARN_A::ENABLED)
    }
}
#[doc = "Field `SLEEPENTER` reader - Enable or disable interrupt for event SLEEPENTER"]
pub type SLEEPENTER_R = crate::BitReader<SLEEPENTER_A>;
#[doc = "Enable or disable interrupt for event SLEEPENTER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPENTER_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<SLEEPENTER_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPENTER_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEEPENTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPENTER_A {
        match self.bits {
            false => SLEEPENTER_A::DISABLED,
            true => SLEEPENTER_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLEEPENTER_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLEEPENTER_A::ENABLED
    }
}
#[doc = "Field `SLEEPENTER` writer - Enable or disable interrupt for event SLEEPENTER"]
pub type SLEEPENTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, SLEEPENTER_A, O>;
impl<'a, const O: u8> SLEEPENTER_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLEEPENTER_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLEEPENTER_A::ENABLED)
    }
}
#[doc = "Field `SLEEPEXIT` reader - Enable or disable interrupt for event SLEEPEXIT"]
pub type SLEEPEXIT_R = crate::BitReader<SLEEPEXIT_A>;
#[doc = "Enable or disable interrupt for event SLEEPEXIT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPEXIT_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<SLEEPEXIT_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPEXIT_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEEPEXIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPEXIT_A {
        match self.bits {
            false => SLEEPEXIT_A::DISABLED,
            true => SLEEPEXIT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLEEPEXIT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLEEPEXIT_A::ENABLED
    }
}
#[doc = "Field `SLEEPEXIT` writer - Enable or disable interrupt for event SLEEPEXIT"]
pub type SLEEPEXIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, SLEEPEXIT_A, O>;
impl<'a, const O: u8> SLEEPEXIT_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLEEPEXIT_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLEEPEXIT_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 2 - Enable or disable interrupt for event POFWARN"]
    #[inline(always)]
    pub fn pofwarn(&self) -> POFWARN_R {
        POFWARN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event SLEEPENTER"]
    #[inline(always)]
    pub fn sleepenter(&self) -> SLEEPENTER_R {
        SLEEPENTER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event SLEEPEXIT"]
    #[inline(always)]
    pub fn sleepexit(&self) -> SLEEPEXIT_R {
        SLEEPEXIT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Enable or disable interrupt for event POFWARN"]
    #[inline(always)]
    pub fn pofwarn(&mut self) -> POFWARN_W<2> {
        POFWARN_W::new(self)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event SLEEPENTER"]
    #[inline(always)]
    pub fn sleepenter(&mut self) -> SLEEPENTER_W<5> {
        SLEEPENTER_W::new(self)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event SLEEPEXIT"]
    #[inline(always)]
    pub fn sleepexit(&mut self) -> SLEEPEXIT_W<6> {
        SLEEPEXIT_W::new(self)
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
