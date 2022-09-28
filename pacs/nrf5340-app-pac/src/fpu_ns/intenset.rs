#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INVALIDOPERATION` reader - Write '1' to enable interrupt for event INVALIDOPERATION"]
pub type INVALIDOPERATION_R = crate::BitReader<INVALIDOPERATION_A>;
#[doc = "Write '1' to enable interrupt for event INVALIDOPERATION\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVALIDOPERATION_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<INVALIDOPERATION_A> for bool {
    #[inline(always)]
    fn from(variant: INVALIDOPERATION_A) -> Self {
        variant as u8 != 0
    }
}
impl INVALIDOPERATION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVALIDOPERATION_A {
        match self.bits {
            false => INVALIDOPERATION_A::DISABLED,
            true => INVALIDOPERATION_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INVALIDOPERATION_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INVALIDOPERATION_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event INVALIDOPERATION\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVALIDOPERATION_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<INVALIDOPERATION_AW> for bool {
    #[inline(always)]
    fn from(variant: INVALIDOPERATION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVALIDOPERATION` writer - Write '1' to enable interrupt for event INVALIDOPERATION"]
pub type INVALIDOPERATION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENSET_SPEC, INVALIDOPERATION_AW, O>;
impl<'a, const O: u8> INVALIDOPERATION_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(INVALIDOPERATION_AW::SET)
    }
}
#[doc = "Field `DIVIDEBYZERO` reader - Write '1' to enable interrupt for event DIVIDEBYZERO"]
pub type DIVIDEBYZERO_R = crate::BitReader<DIVIDEBYZERO_A>;
#[doc = "Write '1' to enable interrupt for event DIVIDEBYZERO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVIDEBYZERO_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<DIVIDEBYZERO_A> for bool {
    #[inline(always)]
    fn from(variant: DIVIDEBYZERO_A) -> Self {
        variant as u8 != 0
    }
}
impl DIVIDEBYZERO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVIDEBYZERO_A {
        match self.bits {
            false => DIVIDEBYZERO_A::DISABLED,
            true => DIVIDEBYZERO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIVIDEBYZERO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIVIDEBYZERO_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event DIVIDEBYZERO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVIDEBYZERO_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<DIVIDEBYZERO_AW> for bool {
    #[inline(always)]
    fn from(variant: DIVIDEBYZERO_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIVIDEBYZERO` writer - Write '1' to enable interrupt for event DIVIDEBYZERO"]
pub type DIVIDEBYZERO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENSET_SPEC, DIVIDEBYZERO_AW, O>;
impl<'a, const O: u8> DIVIDEBYZERO_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DIVIDEBYZERO_AW::SET)
    }
}
#[doc = "Field `OVERFLOW` reader - Write '1' to enable interrupt for event OVERFLOW"]
pub type OVERFLOW_R = crate::BitReader<OVERFLOW_A>;
#[doc = "Write '1' to enable interrupt for event OVERFLOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOW_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<OVERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: OVERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl OVERFLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERFLOW_A {
        match self.bits {
            false => OVERFLOW_A::DISABLED,
            true => OVERFLOW_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVERFLOW_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVERFLOW_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event OVERFLOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOW_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<OVERFLOW_AW> for bool {
    #[inline(always)]
    fn from(variant: OVERFLOW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERFLOW` writer - Write '1' to enable interrupt for event OVERFLOW"]
pub type OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, OVERFLOW_AW, O>;
impl<'a, const O: u8> OVERFLOW_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OVERFLOW_AW::SET)
    }
}
#[doc = "Field `UNDERFLOW` reader - Write '1' to enable interrupt for event UNDERFLOW"]
pub type UNDERFLOW_R = crate::BitReader<UNDERFLOW_A>;
#[doc = "Write '1' to enable interrupt for event UNDERFLOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNDERFLOW_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<UNDERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: UNDERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl UNDERFLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNDERFLOW_A {
        match self.bits {
            false => UNDERFLOW_A::DISABLED,
            true => UNDERFLOW_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UNDERFLOW_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UNDERFLOW_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event UNDERFLOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNDERFLOW_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<UNDERFLOW_AW> for bool {
    #[inline(always)]
    fn from(variant: UNDERFLOW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNDERFLOW` writer - Write '1' to enable interrupt for event UNDERFLOW"]
pub type UNDERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, UNDERFLOW_AW, O>;
impl<'a, const O: u8> UNDERFLOW_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(UNDERFLOW_AW::SET)
    }
}
#[doc = "Field `INEXACT` reader - Write '1' to enable interrupt for event INEXACT"]
pub type INEXACT_R = crate::BitReader<INEXACT_A>;
#[doc = "Write '1' to enable interrupt for event INEXACT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INEXACT_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<INEXACT_A> for bool {
    #[inline(always)]
    fn from(variant: INEXACT_A) -> Self {
        variant as u8 != 0
    }
}
impl INEXACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INEXACT_A {
        match self.bits {
            false => INEXACT_A::DISABLED,
            true => INEXACT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INEXACT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INEXACT_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event INEXACT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INEXACT_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<INEXACT_AW> for bool {
    #[inline(always)]
    fn from(variant: INEXACT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INEXACT` writer - Write '1' to enable interrupt for event INEXACT"]
pub type INEXACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, INEXACT_AW, O>;
impl<'a, const O: u8> INEXACT_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(INEXACT_AW::SET)
    }
}
#[doc = "Field `DENORMALINPUT` reader - Write '1' to enable interrupt for event DENORMALINPUT"]
pub type DENORMALINPUT_R = crate::BitReader<DENORMALINPUT_A>;
#[doc = "Write '1' to enable interrupt for event DENORMALINPUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DENORMALINPUT_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<DENORMALINPUT_A> for bool {
    #[inline(always)]
    fn from(variant: DENORMALINPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl DENORMALINPUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DENORMALINPUT_A {
        match self.bits {
            false => DENORMALINPUT_A::DISABLED,
            true => DENORMALINPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DENORMALINPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DENORMALINPUT_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event DENORMALINPUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DENORMALINPUT_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<DENORMALINPUT_AW> for bool {
    #[inline(always)]
    fn from(variant: DENORMALINPUT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DENORMALINPUT` writer - Write '1' to enable interrupt for event DENORMALINPUT"]
pub type DENORMALINPUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENSET_SPEC, DENORMALINPUT_AW, O>;
impl<'a, const O: u8> DENORMALINPUT_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DENORMALINPUT_AW::SET)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event INVALIDOPERATION"]
    #[inline(always)]
    pub fn invalidoperation(&self) -> INVALIDOPERATION_R {
        INVALIDOPERATION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event DIVIDEBYZERO"]
    #[inline(always)]
    pub fn dividebyzero(&self) -> DIVIDEBYZERO_R {
        DIVIDEBYZERO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event OVERFLOW"]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event UNDERFLOW"]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event INEXACT"]
    #[inline(always)]
    pub fn inexact(&self) -> INEXACT_R {
        INEXACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event DENORMALINPUT"]
    #[inline(always)]
    pub fn denormalinput(&self) -> DENORMALINPUT_R {
        DENORMALINPUT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event INVALIDOPERATION"]
    #[inline(always)]
    pub fn invalidoperation(&mut self) -> INVALIDOPERATION_W<0> {
        INVALIDOPERATION_W::new(self)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event DIVIDEBYZERO"]
    #[inline(always)]
    pub fn dividebyzero(&mut self) -> DIVIDEBYZERO_W<1> {
        DIVIDEBYZERO_W::new(self)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event OVERFLOW"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W<2> {
        OVERFLOW_W::new(self)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event UNDERFLOW"]
    #[inline(always)]
    pub fn underflow(&mut self) -> UNDERFLOW_W<3> {
        UNDERFLOW_W::new(self)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event INEXACT"]
    #[inline(always)]
    pub fn inexact(&mut self) -> INEXACT_W<4> {
        INEXACT_W::new(self)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event DENORMALINPUT"]
    #[inline(always)]
    pub fn denormalinput(&mut self) -> DENORMALINPUT_W<5> {
        DENORMALINPUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
