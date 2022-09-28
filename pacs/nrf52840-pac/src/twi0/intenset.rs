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
#[doc = "Field `STOPPED` reader - Write '1' to enable interrupt for STOPPED event"]
pub type STOPPED_R = crate::BitReader<STOPPED_A>;
#[doc = "Write '1' to enable interrupt for STOPPED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPPED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<STOPPED_A> for bool {
    #[inline(always)]
    fn from(variant: STOPPED_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPPED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPPED_A {
        match self.bits {
            false => STOPPED_A::DISABLED,
            true => STOPPED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STOPPED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STOPPED_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for STOPPED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPPED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<STOPPED_AW> for bool {
    #[inline(always)]
    fn from(variant: STOPPED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` writer - Write '1' to enable interrupt for STOPPED event"]
pub type STOPPED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, STOPPED_AW, O>;
impl<'a, const O: u8> STOPPED_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(STOPPED_AW::SET)
    }
}
#[doc = "Field `RXDREADY` reader - Write '1' to enable interrupt for RXDREADY event"]
pub type RXDREADY_R = crate::BitReader<RXDREADY_A>;
#[doc = "Write '1' to enable interrupt for RXDREADY event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDREADY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RXDREADY_A> for bool {
    #[inline(always)]
    fn from(variant: RXDREADY_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDREADY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDREADY_A {
        match self.bits {
            false => RXDREADY_A::DISABLED,
            true => RXDREADY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDREADY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDREADY_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for RXDREADY event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDREADY_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<RXDREADY_AW> for bool {
    #[inline(always)]
    fn from(variant: RXDREADY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDREADY` writer - Write '1' to enable interrupt for RXDREADY event"]
pub type RXDREADY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, RXDREADY_AW, O>;
impl<'a, const O: u8> RXDREADY_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RXDREADY_AW::SET)
    }
}
#[doc = "Field `TXDSENT` reader - Write '1' to enable interrupt for TXDSENT event"]
pub type TXDSENT_R = crate::BitReader<TXDSENT_A>;
#[doc = "Write '1' to enable interrupt for TXDSENT event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDSENT_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TXDSENT_A> for bool {
    #[inline(always)]
    fn from(variant: TXDSENT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXDSENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDSENT_A {
        match self.bits {
            false => TXDSENT_A::DISABLED,
            true => TXDSENT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDSENT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDSENT_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for TXDSENT event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDSENT_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TXDSENT_AW> for bool {
    #[inline(always)]
    fn from(variant: TXDSENT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDSENT` writer - Write '1' to enable interrupt for TXDSENT event"]
pub type TXDSENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, TXDSENT_AW, O>;
impl<'a, const O: u8> TXDSENT_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TXDSENT_AW::SET)
    }
}
#[doc = "Field `ERROR` reader - Write '1' to enable interrupt for ERROR event"]
pub type ERROR_R = crate::BitReader<ERROR_A>;
#[doc = "Write '1' to enable interrupt for ERROR event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl ERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::DISABLED,
            true => ERROR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERROR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERROR_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for ERROR event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<ERROR_AW> for bool {
    #[inline(always)]
    fn from(variant: ERROR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` writer - Write '1' to enable interrupt for ERROR event"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, ERROR_AW, O>;
impl<'a, const O: u8> ERROR_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ERROR_AW::SET)
    }
}
#[doc = "Field `BB` reader - Write '1' to enable interrupt for BB event"]
pub type BB_R = crate::BitReader<BB_A>;
#[doc = "Write '1' to enable interrupt for BB event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<BB_A> for bool {
    #[inline(always)]
    fn from(variant: BB_A) -> Self {
        variant as u8 != 0
    }
}
impl BB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_A {
        match self.bits {
            false => BB_A::DISABLED,
            true => BB_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BB_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for BB event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<BB_AW> for bool {
    #[inline(always)]
    fn from(variant: BB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BB` writer - Write '1' to enable interrupt for BB event"]
pub type BB_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, BB_AW, O>;
impl<'a, const O: u8> BB_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BB_AW::SET)
    }
}
#[doc = "Field `SUSPENDED` reader - Write '1' to enable interrupt for SUSPENDED event"]
pub type SUSPENDED_R = crate::BitReader<SUSPENDED_A>;
#[doc = "Write '1' to enable interrupt for SUSPENDED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPENDED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<SUSPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPENDED_A) -> Self {
        variant as u8 != 0
    }
}
impl SUSPENDED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSPENDED_A {
        match self.bits {
            false => SUSPENDED_A::DISABLED,
            true => SUSPENDED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SUSPENDED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SUSPENDED_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for SUSPENDED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPENDED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<SUSPENDED_AW> for bool {
    #[inline(always)]
    fn from(variant: SUSPENDED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPENDED` writer - Write '1' to enable interrupt for SUSPENDED event"]
pub type SUSPENDED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, SUSPENDED_AW, O>;
impl<'a, const O: u8> SUSPENDED_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(SUSPENDED_AW::SET)
    }
}
impl R {
    #[doc = "Bit 1 - Write '1' to enable interrupt for STOPPED event"]
    #[inline(always)]
    pub fn stopped(&self) -> STOPPED_R {
        STOPPED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for RXDREADY event"]
    #[inline(always)]
    pub fn rxdready(&self) -> RXDREADY_R {
        RXDREADY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for TXDSENT event"]
    #[inline(always)]
    pub fn txdsent(&self) -> TXDSENT_R {
        TXDSENT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for ERROR event"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Write '1' to enable interrupt for BB event"]
    #[inline(always)]
    pub fn bb(&self) -> BB_R {
        BB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Write '1' to enable interrupt for SUSPENDED event"]
    #[inline(always)]
    pub fn suspended(&self) -> SUSPENDED_R {
        SUSPENDED_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Write '1' to enable interrupt for STOPPED event"]
    #[inline(always)]
    pub fn stopped(&mut self) -> STOPPED_W<1> {
        STOPPED_W::new(self)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for RXDREADY event"]
    #[inline(always)]
    pub fn rxdready(&mut self) -> RXDREADY_W<2> {
        RXDREADY_W::new(self)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for TXDSENT event"]
    #[inline(always)]
    pub fn txdsent(&mut self) -> TXDSENT_W<7> {
        TXDSENT_W::new(self)
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for ERROR event"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W<9> {
        ERROR_W::new(self)
    }
    #[doc = "Bit 14 - Write '1' to enable interrupt for BB event"]
    #[inline(always)]
    pub fn bb(&mut self) -> BB_W<14> {
        BB_W::new(self)
    }
    #[doc = "Bit 18 - Write '1' to enable interrupt for SUSPENDED event"]
    #[inline(always)]
    pub fn suspended(&mut self) -> SUSPENDED_W<18> {
        SUSPENDED_W::new(self)
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
