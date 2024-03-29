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
#[doc = "Field `STARTED` reader - Enable or disable interrupt for STARTED event"]
pub type STARTED_R = crate::BitReader<STARTED_A>;
#[doc = "Enable or disable interrupt for STARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTED_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<STARTED_A> for bool {
    #[inline(always)]
    fn from(variant: STARTED_A) -> Self {
        variant as u8 != 0
    }
}
impl STARTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTED_A {
        match self.bits {
            false => STARTED_A::DISABLED,
            true => STARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STARTED_A::ENABLED
    }
}
#[doc = "Field `STARTED` writer - Enable or disable interrupt for STARTED event"]
pub type STARTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, STARTED_A, O>;
impl<'a, const O: u8> STARTED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STARTED_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(STARTED_A::ENABLED)
    }
}
#[doc = "Field `END` reader - Enable or disable interrupt for END event"]
pub type END_R = crate::BitReader<END_A>;
#[doc = "Enable or disable interrupt for END event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<END_A> for bool {
    #[inline(always)]
    fn from(variant: END_A) -> Self {
        variant as u8 != 0
    }
}
impl END_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> END_A {
        match self.bits {
            false => END_A::DISABLED,
            true => END_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == END_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == END_A::ENABLED
    }
}
#[doc = "Field `END` writer - Enable or disable interrupt for END event"]
pub type END_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, END_A, O>;
impl<'a, const O: u8> END_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(END_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(END_A::ENABLED)
    }
}
#[doc = "Field `DONE` reader - Enable or disable interrupt for DONE event"]
pub type DONE_R = crate::BitReader<DONE_A>;
#[doc = "Enable or disable interrupt for DONE event\n\nValue on reset: 0"]
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
#[doc = "Field `DONE` writer - Enable or disable interrupt for DONE event"]
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
#[doc = "Field `RESULTDONE` reader - Enable or disable interrupt for RESULTDONE event"]
pub type RESULTDONE_R = crate::BitReader<RESULTDONE_A>;
#[doc = "Enable or disable interrupt for RESULTDONE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESULTDONE_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RESULTDONE_A> for bool {
    #[inline(always)]
    fn from(variant: RESULTDONE_A) -> Self {
        variant as u8 != 0
    }
}
impl RESULTDONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESULTDONE_A {
        match self.bits {
            false => RESULTDONE_A::DISABLED,
            true => RESULTDONE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RESULTDONE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RESULTDONE_A::ENABLED
    }
}
#[doc = "Field `RESULTDONE` writer - Enable or disable interrupt for RESULTDONE event"]
pub type RESULTDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RESULTDONE_A, O>;
impl<'a, const O: u8> RESULTDONE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RESULTDONE_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RESULTDONE_A::ENABLED)
    }
}
#[doc = "Field `CALIBRATEDONE` reader - Enable or disable interrupt for CALIBRATEDONE event"]
pub type CALIBRATEDONE_R = crate::BitReader<CALIBRATEDONE_A>;
#[doc = "Enable or disable interrupt for CALIBRATEDONE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALIBRATEDONE_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CALIBRATEDONE_A> for bool {
    #[inline(always)]
    fn from(variant: CALIBRATEDONE_A) -> Self {
        variant as u8 != 0
    }
}
impl CALIBRATEDONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALIBRATEDONE_A {
        match self.bits {
            false => CALIBRATEDONE_A::DISABLED,
            true => CALIBRATEDONE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CALIBRATEDONE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CALIBRATEDONE_A::ENABLED
    }
}
#[doc = "Field `CALIBRATEDONE` writer - Enable or disable interrupt for CALIBRATEDONE event"]
pub type CALIBRATEDONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTEN_SPEC, CALIBRATEDONE_A, O>;
impl<'a, const O: u8> CALIBRATEDONE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CALIBRATEDONE_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CALIBRATEDONE_A::ENABLED)
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
#[doc = "Field `CH0LIMITH` reader - Enable or disable interrupt for CH\\[0\\].LIMITH event"]
pub type CH0LIMITH_R = crate::BitReader<CH0LIMITH_A>;
#[doc = "Enable or disable interrupt for CH\\[0\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0LIMITH_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CH0LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH0LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0LIMITH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0LIMITH_A {
        match self.bits {
            false => CH0LIMITH_A::DISABLED,
            true => CH0LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH0LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH0LIMITH_A::ENABLED
    }
}
#[doc = "Field `CH0LIMITH` writer - Enable or disable interrupt for CH\\[0\\].LIMITH event"]
pub type CH0LIMITH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, CH0LIMITH_A, O>;
impl<'a, const O: u8> CH0LIMITH_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH0LIMITH_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH0LIMITH_A::ENABLED)
    }
}
#[doc = "Field `CH0LIMITL` reader - Enable or disable interrupt for CH\\[0\\].LIMITL event"]
pub type CH0LIMITL_R = crate::BitReader<CH0LIMITL_A>;
#[doc = "Enable or disable interrupt for CH\\[0\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0LIMITL_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CH0LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH0LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0LIMITL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0LIMITL_A {
        match self.bits {
            false => CH0LIMITL_A::DISABLED,
            true => CH0LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH0LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH0LIMITL_A::ENABLED
    }
}
#[doc = "Field `CH0LIMITL` writer - Enable or disable interrupt for CH\\[0\\].LIMITL event"]
pub type CH0LIMITL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, CH0LIMITL_A, O>;
impl<'a, const O: u8> CH0LIMITL_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH0LIMITL_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH0LIMITL_A::ENABLED)
    }
}
#[doc = "Field `CH1LIMITH` reader - Enable or disable interrupt for CH\\[1\\].LIMITH event"]
pub type CH1LIMITH_R = crate::BitReader<CH1LIMITH_A>;
#[doc = "Enable or disable interrupt for CH\\[1\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1LIMITH_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CH1LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH1LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1LIMITH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1LIMITH_A {
        match self.bits {
            false => CH1LIMITH_A::DISABLED,
            true => CH1LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH1LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH1LIMITH_A::ENABLED
    }
}
#[doc = "Field `CH1LIMITH` writer - Enable or disable interrupt for CH\\[1\\].LIMITH event"]
pub type CH1LIMITH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, CH1LIMITH_A, O>;
impl<'a, const O: u8> CH1LIMITH_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH1LIMITH_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH1LIMITH_A::ENABLED)
    }
}
#[doc = "Field `CH1LIMITL` reader - Enable or disable interrupt for CH\\[1\\].LIMITL event"]
pub type CH1LIMITL_R = crate::BitReader<CH1LIMITL_A>;
#[doc = "Enable or disable interrupt for CH\\[1\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1LIMITL_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CH1LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH1LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1LIMITL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1LIMITL_A {
        match self.bits {
            false => CH1LIMITL_A::DISABLED,
            true => CH1LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH1LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH1LIMITL_A::ENABLED
    }
}
#[doc = "Field `CH1LIMITL` writer - Enable or disable interrupt for CH\\[1\\].LIMITL event"]
pub type CH1LIMITL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, CH1LIMITL_A, O>;
impl<'a, const O: u8> CH1LIMITL_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH1LIMITL_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH1LIMITL_A::ENABLED)
    }
}
#[doc = "Field `CH2LIMITH` reader - Enable or disable interrupt for CH\\[2\\].LIMITH event"]
pub type CH2LIMITH_R = crate::BitReader<CH2LIMITH_A>;
#[doc = "Enable or disable interrupt for CH\\[2\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2LIMITH_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CH2LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH2LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2LIMITH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2LIMITH_A {
        match self.bits {
            false => CH2LIMITH_A::DISABLED,
            true => CH2LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH2LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH2LIMITH_A::ENABLED
    }
}
#[doc = "Field `CH2LIMITH` writer - Enable or disable interrupt for CH\\[2\\].LIMITH event"]
pub type CH2LIMITH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, CH2LIMITH_A, O>;
impl<'a, const O: u8> CH2LIMITH_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH2LIMITH_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH2LIMITH_A::ENABLED)
    }
}
#[doc = "Field `CH2LIMITL` reader - Enable or disable interrupt for CH\\[2\\].LIMITL event"]
pub type CH2LIMITL_R = crate::BitReader<CH2LIMITL_A>;
#[doc = "Enable or disable interrupt for CH\\[2\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2LIMITL_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CH2LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH2LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2LIMITL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2LIMITL_A {
        match self.bits {
            false => CH2LIMITL_A::DISABLED,
            true => CH2LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH2LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH2LIMITL_A::ENABLED
    }
}
#[doc = "Field `CH2LIMITL` writer - Enable or disable interrupt for CH\\[2\\].LIMITL event"]
pub type CH2LIMITL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, CH2LIMITL_A, O>;
impl<'a, const O: u8> CH2LIMITL_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH2LIMITL_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH2LIMITL_A::ENABLED)
    }
}
#[doc = "Field `CH3LIMITH` reader - Enable or disable interrupt for CH\\[3\\].LIMITH event"]
pub type CH3LIMITH_R = crate::BitReader<CH3LIMITH_A>;
#[doc = "Enable or disable interrupt for CH\\[3\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3LIMITH_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CH3LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH3LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
impl CH3LIMITH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3LIMITH_A {
        match self.bits {
            false => CH3LIMITH_A::DISABLED,
            true => CH3LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH3LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH3LIMITH_A::ENABLED
    }
}
#[doc = "Field `CH3LIMITH` writer - Enable or disable interrupt for CH\\[3\\].LIMITH event"]
pub type CH3LIMITH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, CH3LIMITH_A, O>;
impl<'a, const O: u8> CH3LIMITH_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH3LIMITH_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH3LIMITH_A::ENABLED)
    }
}
#[doc = "Field `CH3LIMITL` reader - Enable or disable interrupt for CH\\[3\\].LIMITL event"]
pub type CH3LIMITL_R = crate::BitReader<CH3LIMITL_A>;
#[doc = "Enable or disable interrupt for CH\\[3\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3LIMITL_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CH3LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH3LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH3LIMITL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3LIMITL_A {
        match self.bits {
            false => CH3LIMITL_A::DISABLED,
            true => CH3LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH3LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH3LIMITL_A::ENABLED
    }
}
#[doc = "Field `CH3LIMITL` writer - Enable or disable interrupt for CH\\[3\\].LIMITL event"]
pub type CH3LIMITL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, CH3LIMITL_A, O>;
impl<'a, const O: u8> CH3LIMITL_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH3LIMITL_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH3LIMITL_A::ENABLED)
    }
}
#[doc = "Field `CH4LIMITH` reader - Enable or disable interrupt for CH\\[4\\].LIMITH event"]
pub type CH4LIMITH_R = crate::BitReader<CH4LIMITH_A>;
#[doc = "Enable or disable interrupt for CH\\[4\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4LIMITH_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CH4LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH4LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
impl CH4LIMITH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4LIMITH_A {
        match self.bits {
            false => CH4LIMITH_A::DISABLED,
            true => CH4LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH4LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH4LIMITH_A::ENABLED
    }
}
#[doc = "Field `CH4LIMITH` writer - Enable or disable interrupt for CH\\[4\\].LIMITH event"]
pub type CH4LIMITH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, CH4LIMITH_A, O>;
impl<'a, const O: u8> CH4LIMITH_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH4LIMITH_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH4LIMITH_A::ENABLED)
    }
}
#[doc = "Field `CH4LIMITL` reader - Enable or disable interrupt for CH\\[4\\].LIMITL event"]
pub type CH4LIMITL_R = crate::BitReader<CH4LIMITL_A>;
#[doc = "Enable or disable interrupt for CH\\[4\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4LIMITL_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CH4LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH4LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH4LIMITL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4LIMITL_A {
        match self.bits {
            false => CH4LIMITL_A::DISABLED,
            true => CH4LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH4LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH4LIMITL_A::ENABLED
    }
}
#[doc = "Field `CH4LIMITL` writer - Enable or disable interrupt for CH\\[4\\].LIMITL event"]
pub type CH4LIMITL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, CH4LIMITL_A, O>;
impl<'a, const O: u8> CH4LIMITL_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH4LIMITL_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH4LIMITL_A::ENABLED)
    }
}
#[doc = "Field `CH5LIMITH` reader - Enable or disable interrupt for CH\\[5\\].LIMITH event"]
pub type CH5LIMITH_R = crate::BitReader<CH5LIMITH_A>;
#[doc = "Enable or disable interrupt for CH\\[5\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5LIMITH_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CH5LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH5LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
impl CH5LIMITH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5LIMITH_A {
        match self.bits {
            false => CH5LIMITH_A::DISABLED,
            true => CH5LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH5LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH5LIMITH_A::ENABLED
    }
}
#[doc = "Field `CH5LIMITH` writer - Enable or disable interrupt for CH\\[5\\].LIMITH event"]
pub type CH5LIMITH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, CH5LIMITH_A, O>;
impl<'a, const O: u8> CH5LIMITH_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH5LIMITH_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH5LIMITH_A::ENABLED)
    }
}
#[doc = "Field `CH5LIMITL` reader - Enable or disable interrupt for CH\\[5\\].LIMITL event"]
pub type CH5LIMITL_R = crate::BitReader<CH5LIMITL_A>;
#[doc = "Enable or disable interrupt for CH\\[5\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5LIMITL_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CH5LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH5LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH5LIMITL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5LIMITL_A {
        match self.bits {
            false => CH5LIMITL_A::DISABLED,
            true => CH5LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH5LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH5LIMITL_A::ENABLED
    }
}
#[doc = "Field `CH5LIMITL` writer - Enable or disable interrupt for CH\\[5\\].LIMITL event"]
pub type CH5LIMITL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, CH5LIMITL_A, O>;
impl<'a, const O: u8> CH5LIMITL_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH5LIMITL_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH5LIMITL_A::ENABLED)
    }
}
#[doc = "Field `CH6LIMITH` reader - Enable or disable interrupt for CH\\[6\\].LIMITH event"]
pub type CH6LIMITH_R = crate::BitReader<CH6LIMITH_A>;
#[doc = "Enable or disable interrupt for CH\\[6\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6LIMITH_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CH6LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH6LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
impl CH6LIMITH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6LIMITH_A {
        match self.bits {
            false => CH6LIMITH_A::DISABLED,
            true => CH6LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH6LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH6LIMITH_A::ENABLED
    }
}
#[doc = "Field `CH6LIMITH` writer - Enable or disable interrupt for CH\\[6\\].LIMITH event"]
pub type CH6LIMITH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, CH6LIMITH_A, O>;
impl<'a, const O: u8> CH6LIMITH_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH6LIMITH_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH6LIMITH_A::ENABLED)
    }
}
#[doc = "Field `CH6LIMITL` reader - Enable or disable interrupt for CH\\[6\\].LIMITL event"]
pub type CH6LIMITL_R = crate::BitReader<CH6LIMITL_A>;
#[doc = "Enable or disable interrupt for CH\\[6\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6LIMITL_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CH6LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH6LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH6LIMITL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6LIMITL_A {
        match self.bits {
            false => CH6LIMITL_A::DISABLED,
            true => CH6LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH6LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH6LIMITL_A::ENABLED
    }
}
#[doc = "Field `CH6LIMITL` writer - Enable or disable interrupt for CH\\[6\\].LIMITL event"]
pub type CH6LIMITL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, CH6LIMITL_A, O>;
impl<'a, const O: u8> CH6LIMITL_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH6LIMITL_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH6LIMITL_A::ENABLED)
    }
}
#[doc = "Field `CH7LIMITH` reader - Enable or disable interrupt for CH\\[7\\].LIMITH event"]
pub type CH7LIMITH_R = crate::BitReader<CH7LIMITH_A>;
#[doc = "Enable or disable interrupt for CH\\[7\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7LIMITH_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CH7LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH7LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
impl CH7LIMITH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7LIMITH_A {
        match self.bits {
            false => CH7LIMITH_A::DISABLED,
            true => CH7LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH7LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH7LIMITH_A::ENABLED
    }
}
#[doc = "Field `CH7LIMITH` writer - Enable or disable interrupt for CH\\[7\\].LIMITH event"]
pub type CH7LIMITH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, CH7LIMITH_A, O>;
impl<'a, const O: u8> CH7LIMITH_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH7LIMITH_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH7LIMITH_A::ENABLED)
    }
}
#[doc = "Field `CH7LIMITL` reader - Enable or disable interrupt for CH\\[7\\].LIMITL event"]
pub type CH7LIMITL_R = crate::BitReader<CH7LIMITL_A>;
#[doc = "Enable or disable interrupt for CH\\[7\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7LIMITL_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CH7LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH7LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH7LIMITL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7LIMITL_A {
        match self.bits {
            false => CH7LIMITL_A::DISABLED,
            true => CH7LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH7LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH7LIMITL_A::ENABLED
    }
}
#[doc = "Field `CH7LIMITL` writer - Enable or disable interrupt for CH\\[7\\].LIMITL event"]
pub type CH7LIMITL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, CH7LIMITL_A, O>;
impl<'a, const O: u8> CH7LIMITL_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CH7LIMITL_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CH7LIMITL_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for STARTED event"]
    #[inline(always)]
    pub fn started(&self) -> STARTED_R {
        STARTED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for END event"]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for DONE event"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for RESULTDONE event"]
    #[inline(always)]
    pub fn resultdone(&self) -> RESULTDONE_R {
        RESULTDONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for CALIBRATEDONE event"]
    #[inline(always)]
    pub fn calibratedone(&self) -> CALIBRATEDONE_R {
        CALIBRATEDONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for STOPPED event"]
    #[inline(always)]
    pub fn stopped(&self) -> STOPPED_R {
        STOPPED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for CH\\[0\\].LIMITH event"]
    #[inline(always)]
    pub fn ch0limith(&self) -> CH0LIMITH_R {
        CH0LIMITH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for CH\\[0\\].LIMITL event"]
    #[inline(always)]
    pub fn ch0limitl(&self) -> CH0LIMITL_R {
        CH0LIMITL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for CH\\[1\\].LIMITH event"]
    #[inline(always)]
    pub fn ch1limith(&self) -> CH1LIMITH_R {
        CH1LIMITH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for CH\\[1\\].LIMITL event"]
    #[inline(always)]
    pub fn ch1limitl(&self) -> CH1LIMITL_R {
        CH1LIMITL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for CH\\[2\\].LIMITH event"]
    #[inline(always)]
    pub fn ch2limith(&self) -> CH2LIMITH_R {
        CH2LIMITH_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable or disable interrupt for CH\\[2\\].LIMITL event"]
    #[inline(always)]
    pub fn ch2limitl(&self) -> CH2LIMITL_R {
        CH2LIMITL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable or disable interrupt for CH\\[3\\].LIMITH event"]
    #[inline(always)]
    pub fn ch3limith(&self) -> CH3LIMITH_R {
        CH3LIMITH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for CH\\[3\\].LIMITL event"]
    #[inline(always)]
    pub fn ch3limitl(&self) -> CH3LIMITL_R {
        CH3LIMITL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for CH\\[4\\].LIMITH event"]
    #[inline(always)]
    pub fn ch4limith(&self) -> CH4LIMITH_R {
        CH4LIMITH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable or disable interrupt for CH\\[4\\].LIMITL event"]
    #[inline(always)]
    pub fn ch4limitl(&self) -> CH4LIMITL_R {
        CH4LIMITL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable or disable interrupt for CH\\[5\\].LIMITH event"]
    #[inline(always)]
    pub fn ch5limith(&self) -> CH5LIMITH_R {
        CH5LIMITH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable or disable interrupt for CH\\[5\\].LIMITL event"]
    #[inline(always)]
    pub fn ch5limitl(&self) -> CH5LIMITL_R {
        CH5LIMITL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable or disable interrupt for CH\\[6\\].LIMITH event"]
    #[inline(always)]
    pub fn ch6limith(&self) -> CH6LIMITH_R {
        CH6LIMITH_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable or disable interrupt for CH\\[6\\].LIMITL event"]
    #[inline(always)]
    pub fn ch6limitl(&self) -> CH6LIMITL_R {
        CH6LIMITL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable or disable interrupt for CH\\[7\\].LIMITH event"]
    #[inline(always)]
    pub fn ch7limith(&self) -> CH7LIMITH_R {
        CH7LIMITH_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable or disable interrupt for CH\\[7\\].LIMITL event"]
    #[inline(always)]
    pub fn ch7limitl(&self) -> CH7LIMITL_R {
        CH7LIMITL_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for STARTED event"]
    #[inline(always)]
    pub fn started(&mut self) -> STARTED_W<0> {
        STARTED_W::new(self)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for END event"]
    #[inline(always)]
    pub fn end(&mut self) -> END_W<1> {
        END_W::new(self)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for DONE event"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W<2> {
        DONE_W::new(self)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for RESULTDONE event"]
    #[inline(always)]
    pub fn resultdone(&mut self) -> RESULTDONE_W<3> {
        RESULTDONE_W::new(self)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for CALIBRATEDONE event"]
    #[inline(always)]
    pub fn calibratedone(&mut self) -> CALIBRATEDONE_W<4> {
        CALIBRATEDONE_W::new(self)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for STOPPED event"]
    #[inline(always)]
    pub fn stopped(&mut self) -> STOPPED_W<5> {
        STOPPED_W::new(self)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for CH\\[0\\].LIMITH event"]
    #[inline(always)]
    pub fn ch0limith(&mut self) -> CH0LIMITH_W<6> {
        CH0LIMITH_W::new(self)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for CH\\[0\\].LIMITL event"]
    #[inline(always)]
    pub fn ch0limitl(&mut self) -> CH0LIMITL_W<7> {
        CH0LIMITL_W::new(self)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for CH\\[1\\].LIMITH event"]
    #[inline(always)]
    pub fn ch1limith(&mut self) -> CH1LIMITH_W<8> {
        CH1LIMITH_W::new(self)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for CH\\[1\\].LIMITL event"]
    #[inline(always)]
    pub fn ch1limitl(&mut self) -> CH1LIMITL_W<9> {
        CH1LIMITL_W::new(self)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for CH\\[2\\].LIMITH event"]
    #[inline(always)]
    pub fn ch2limith(&mut self) -> CH2LIMITH_W<10> {
        CH2LIMITH_W::new(self)
    }
    #[doc = "Bit 11 - Enable or disable interrupt for CH\\[2\\].LIMITL event"]
    #[inline(always)]
    pub fn ch2limitl(&mut self) -> CH2LIMITL_W<11> {
        CH2LIMITL_W::new(self)
    }
    #[doc = "Bit 12 - Enable or disable interrupt for CH\\[3\\].LIMITH event"]
    #[inline(always)]
    pub fn ch3limith(&mut self) -> CH3LIMITH_W<12> {
        CH3LIMITH_W::new(self)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for CH\\[3\\].LIMITL event"]
    #[inline(always)]
    pub fn ch3limitl(&mut self) -> CH3LIMITL_W<13> {
        CH3LIMITL_W::new(self)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for CH\\[4\\].LIMITH event"]
    #[inline(always)]
    pub fn ch4limith(&mut self) -> CH4LIMITH_W<14> {
        CH4LIMITH_W::new(self)
    }
    #[doc = "Bit 15 - Enable or disable interrupt for CH\\[4\\].LIMITL event"]
    #[inline(always)]
    pub fn ch4limitl(&mut self) -> CH4LIMITL_W<15> {
        CH4LIMITL_W::new(self)
    }
    #[doc = "Bit 16 - Enable or disable interrupt for CH\\[5\\].LIMITH event"]
    #[inline(always)]
    pub fn ch5limith(&mut self) -> CH5LIMITH_W<16> {
        CH5LIMITH_W::new(self)
    }
    #[doc = "Bit 17 - Enable or disable interrupt for CH\\[5\\].LIMITL event"]
    #[inline(always)]
    pub fn ch5limitl(&mut self) -> CH5LIMITL_W<17> {
        CH5LIMITL_W::new(self)
    }
    #[doc = "Bit 18 - Enable or disable interrupt for CH\\[6\\].LIMITH event"]
    #[inline(always)]
    pub fn ch6limith(&mut self) -> CH6LIMITH_W<18> {
        CH6LIMITH_W::new(self)
    }
    #[doc = "Bit 19 - Enable or disable interrupt for CH\\[6\\].LIMITL event"]
    #[inline(always)]
    pub fn ch6limitl(&mut self) -> CH6LIMITL_W<19> {
        CH6LIMITL_W::new(self)
    }
    #[doc = "Bit 20 - Enable or disable interrupt for CH\\[7\\].LIMITH event"]
    #[inline(always)]
    pub fn ch7limith(&mut self) -> CH7LIMITH_W<20> {
        CH7LIMITH_W::new(self)
    }
    #[doc = "Bit 21 - Enable or disable interrupt for CH\\[7\\].LIMITL event"]
    #[inline(always)]
    pub fn ch7limitl(&mut self) -> CH7LIMITL_W<21> {
        CH7LIMITL_W::new(self)
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
