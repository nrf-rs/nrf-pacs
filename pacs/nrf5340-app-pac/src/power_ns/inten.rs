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
#[doc = "Field `POFWARN` reader - Enable or disable interrupt for event POFWARN"]
pub struct POFWARN_R(crate::FieldReader<bool, POFWARN_A>);
impl POFWARN_R {
    pub(crate) fn new(bits: bool) -> Self {
        POFWARN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == POFWARN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == POFWARN_A::ENABLED
    }
}
impl core::ops::Deref for POFWARN_R {
    type Target = crate::FieldReader<bool, POFWARN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POFWARN` writer - Enable or disable interrupt for event POFWARN"]
pub struct POFWARN_W<'a> {
    w: &'a mut W,
}
impl<'a> POFWARN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POFWARN_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
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
#[doc = "Field `SLEEPENTER` reader - Enable or disable interrupt for event SLEEPENTER"]
pub struct SLEEPENTER_R(crate::FieldReader<bool, SLEEPENTER_A>);
impl SLEEPENTER_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLEEPENTER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == SLEEPENTER_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SLEEPENTER_A::ENABLED
    }
}
impl core::ops::Deref for SLEEPENTER_R {
    type Target = crate::FieldReader<bool, SLEEPENTER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEPENTER` writer - Enable or disable interrupt for event SLEEPENTER"]
pub struct SLEEPENTER_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPENTER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPENTER_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
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
#[doc = "Field `SLEEPEXIT` reader - Enable or disable interrupt for event SLEEPEXIT"]
pub struct SLEEPEXIT_R(crate::FieldReader<bool, SLEEPEXIT_A>);
impl SLEEPEXIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLEEPEXIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == SLEEPEXIT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SLEEPEXIT_A::ENABLED
    }
}
impl core::ops::Deref for SLEEPEXIT_R {
    type Target = crate::FieldReader<bool, SLEEPEXIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEPEXIT` writer - Enable or disable interrupt for event SLEEPEXIT"]
pub struct SLEEPEXIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPEXIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPEXIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Enable or disable interrupt for event POFWARN"]
    #[inline(always)]
    pub fn pofwarn(&self) -> POFWARN_R {
        POFWARN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event SLEEPENTER"]
    #[inline(always)]
    pub fn sleepenter(&self) -> SLEEPENTER_R {
        SLEEPENTER_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event SLEEPEXIT"]
    #[inline(always)]
    pub fn sleepexit(&self) -> SLEEPEXIT_R {
        SLEEPEXIT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Enable or disable interrupt for event POFWARN"]
    #[inline(always)]
    pub fn pofwarn(&mut self) -> POFWARN_W {
        POFWARN_W { w: self }
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event SLEEPENTER"]
    #[inline(always)]
    pub fn sleepenter(&mut self) -> SLEEPENTER_W {
        SLEEPENTER_W { w: self }
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event SLEEPEXIT"]
    #[inline(always)]
    pub fn sleepexit(&mut self) -> SLEEPEXIT_W {
        SLEEPEXIT_W { w: self }
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
