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
#[doc = "Field `STOPPED` reader - Enable or disable interrupt for STOPPED event"]
pub type STOPPED_R = crate::BitReader<STOPPED_A>;
#[doc = "Enable or disable interrupt for STOPPED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPPED_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
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
#[doc = "Field `STOPPED` writer - Enable or disable interrupt for STOPPED event"]
pub type STOPPED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, STOPPED_A, O>;
impl<'a, const O: u8> STOPPED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STOPPED_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(STOPPED_A::ENABLED)
    }
}
#[doc = "Field `ERROR` reader - Enable or disable interrupt for ERROR event"]
pub type ERROR_R = crate::BitReader<ERROR_A>;
#[doc = "Enable or disable interrupt for ERROR event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
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
#[doc = "Field `ERROR` writer - Enable or disable interrupt for ERROR event"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, ERROR_A, O>;
impl<'a, const O: u8> ERROR_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERROR_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERROR_A::ENABLED)
    }
}
#[doc = "Field `RXSTARTED` reader - Enable or disable interrupt for RXSTARTED event"]
pub type RXSTARTED_R = crate::BitReader<RXSTARTED_A>;
#[doc = "Enable or disable interrupt for RXSTARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSTARTED_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RXSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: RXSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
impl RXSTARTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXSTARTED_A {
        match self.bits {
            false => RXSTARTED_A::DISABLED,
            true => RXSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXSTARTED_A::ENABLED
    }
}
#[doc = "Field `RXSTARTED` writer - Enable or disable interrupt for RXSTARTED event"]
pub type RXSTARTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RXSTARTED_A, O>;
impl<'a, const O: u8> RXSTARTED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXSTARTED_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXSTARTED_A::ENABLED)
    }
}
#[doc = "Field `TXSTARTED` reader - Enable or disable interrupt for TXSTARTED event"]
pub type TXSTARTED_R = crate::BitReader<TXSTARTED_A>;
#[doc = "Enable or disable interrupt for TXSTARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSTARTED_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TXSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: TXSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
impl TXSTARTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXSTARTED_A {
        match self.bits {
            false => TXSTARTED_A::DISABLED,
            true => TXSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXSTARTED_A::ENABLED
    }
}
#[doc = "Field `TXSTARTED` writer - Enable or disable interrupt for TXSTARTED event"]
pub type TXSTARTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TXSTARTED_A, O>;
impl<'a, const O: u8> TXSTARTED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXSTARTED_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXSTARTED_A::ENABLED)
    }
}
#[doc = "Field `WRITE` reader - Enable or disable interrupt for WRITE event"]
pub type WRITE_R = crate::BitReader<WRITE_A>;
#[doc = "Enable or disable interrupt for WRITE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<WRITE_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_A) -> Self {
        variant as u8 != 0
    }
}
impl WRITE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_A {
        match self.bits {
            false => WRITE_A::DISABLED,
            true => WRITE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WRITE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WRITE_A::ENABLED
    }
}
#[doc = "Field `WRITE` writer - Enable or disable interrupt for WRITE event"]
pub type WRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, WRITE_A, O>;
impl<'a, const O: u8> WRITE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WRITE_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WRITE_A::ENABLED)
    }
}
#[doc = "Field `READ` reader - Enable or disable interrupt for READ event"]
pub type READ_R = crate::BitReader<READ_A>;
#[doc = "Enable or disable interrupt for READ event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<READ_A> for bool {
    #[inline(always)]
    fn from(variant: READ_A) -> Self {
        variant as u8 != 0
    }
}
impl READ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_A {
        match self.bits {
            false => READ_A::DISABLED,
            true => READ_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READ_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READ_A::ENABLED
    }
}
#[doc = "Field `READ` writer - Enable or disable interrupt for READ event"]
pub type READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, READ_A, O>;
impl<'a, const O: u8> READ_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READ_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READ_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 1 - Enable or disable interrupt for STOPPED event"]
    #[inline(always)]
    pub fn stopped(&self) -> STOPPED_R {
        STOPPED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for ERROR event"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable or disable interrupt for RXSTARTED event"]
    #[inline(always)]
    pub fn rxstarted(&self) -> RXSTARTED_R {
        RXSTARTED_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable or disable interrupt for TXSTARTED event"]
    #[inline(always)]
    pub fn txstarted(&self) -> TXSTARTED_R {
        TXSTARTED_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable or disable interrupt for WRITE event"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable or disable interrupt for READ event"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable or disable interrupt for STOPPED event"]
    #[inline(always)]
    pub fn stopped(&mut self) -> STOPPED_W<1> {
        STOPPED_W::new(self)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for ERROR event"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W<9> {
        ERROR_W::new(self)
    }
    #[doc = "Bit 19 - Enable or disable interrupt for RXSTARTED event"]
    #[inline(always)]
    pub fn rxstarted(&mut self) -> RXSTARTED_W<19> {
        RXSTARTED_W::new(self)
    }
    #[doc = "Bit 20 - Enable or disable interrupt for TXSTARTED event"]
    #[inline(always)]
    pub fn txstarted(&mut self) -> TXSTARTED_W<20> {
        TXSTARTED_W::new(self)
    }
    #[doc = "Bit 25 - Enable or disable interrupt for WRITE event"]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W<25> {
        WRITE_W::new(self)
    }
    #[doc = "Bit 26 - Enable or disable interrupt for READ event"]
    #[inline(always)]
    pub fn read(&mut self) -> READ_W<26> {
        READ_W::new(self)
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
