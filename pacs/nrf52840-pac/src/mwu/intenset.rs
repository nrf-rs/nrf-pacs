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
#[doc = "Field `REGION0WA` reader - Write '1' to enable interrupt for REGION\\[0\\].WA event"]
pub type REGION0WA_R = crate::BitReader<REGION0WA_A>;
#[doc = "Write '1' to enable interrupt for REGION\\[0\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION0WA_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<REGION0WA_A> for bool {
    #[inline(always)]
    fn from(variant: REGION0WA_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION0WA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION0WA_A {
        match self.bits {
            false => REGION0WA_A::DISABLED,
            true => REGION0WA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION0WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION0WA_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for REGION\\[0\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION0WA_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<REGION0WA_AW> for bool {
    #[inline(always)]
    fn from(variant: REGION0WA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION0WA` writer - Write '1' to enable interrupt for REGION\\[0\\].WA event"]
pub type REGION0WA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, REGION0WA_AW, O>;
impl<'a, const O: u8> REGION0WA_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(REGION0WA_AW::SET)
    }
}
#[doc = "Field `REGION0RA` reader - Write '1' to enable interrupt for REGION\\[0\\].RA event"]
pub type REGION0RA_R = crate::BitReader<REGION0RA_A>;
#[doc = "Write '1' to enable interrupt for REGION\\[0\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION0RA_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<REGION0RA_A> for bool {
    #[inline(always)]
    fn from(variant: REGION0RA_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION0RA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION0RA_A {
        match self.bits {
            false => REGION0RA_A::DISABLED,
            true => REGION0RA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION0RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION0RA_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for REGION\\[0\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION0RA_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<REGION0RA_AW> for bool {
    #[inline(always)]
    fn from(variant: REGION0RA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION0RA` writer - Write '1' to enable interrupt for REGION\\[0\\].RA event"]
pub type REGION0RA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, REGION0RA_AW, O>;
impl<'a, const O: u8> REGION0RA_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(REGION0RA_AW::SET)
    }
}
#[doc = "Field `REGION1WA` reader - Write '1' to enable interrupt for REGION\\[1\\].WA event"]
pub type REGION1WA_R = crate::BitReader<REGION1WA_A>;
#[doc = "Write '1' to enable interrupt for REGION\\[1\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION1WA_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<REGION1WA_A> for bool {
    #[inline(always)]
    fn from(variant: REGION1WA_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION1WA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION1WA_A {
        match self.bits {
            false => REGION1WA_A::DISABLED,
            true => REGION1WA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION1WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION1WA_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for REGION\\[1\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION1WA_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<REGION1WA_AW> for bool {
    #[inline(always)]
    fn from(variant: REGION1WA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION1WA` writer - Write '1' to enable interrupt for REGION\\[1\\].WA event"]
pub type REGION1WA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, REGION1WA_AW, O>;
impl<'a, const O: u8> REGION1WA_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(REGION1WA_AW::SET)
    }
}
#[doc = "Field `REGION1RA` reader - Write '1' to enable interrupt for REGION\\[1\\].RA event"]
pub type REGION1RA_R = crate::BitReader<REGION1RA_A>;
#[doc = "Write '1' to enable interrupt for REGION\\[1\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION1RA_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<REGION1RA_A> for bool {
    #[inline(always)]
    fn from(variant: REGION1RA_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION1RA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION1RA_A {
        match self.bits {
            false => REGION1RA_A::DISABLED,
            true => REGION1RA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION1RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION1RA_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for REGION\\[1\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION1RA_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<REGION1RA_AW> for bool {
    #[inline(always)]
    fn from(variant: REGION1RA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION1RA` writer - Write '1' to enable interrupt for REGION\\[1\\].RA event"]
pub type REGION1RA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, REGION1RA_AW, O>;
impl<'a, const O: u8> REGION1RA_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(REGION1RA_AW::SET)
    }
}
#[doc = "Field `REGION2WA` reader - Write '1' to enable interrupt for REGION\\[2\\].WA event"]
pub type REGION2WA_R = crate::BitReader<REGION2WA_A>;
#[doc = "Write '1' to enable interrupt for REGION\\[2\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION2WA_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<REGION2WA_A> for bool {
    #[inline(always)]
    fn from(variant: REGION2WA_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION2WA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION2WA_A {
        match self.bits {
            false => REGION2WA_A::DISABLED,
            true => REGION2WA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION2WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION2WA_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for REGION\\[2\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION2WA_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<REGION2WA_AW> for bool {
    #[inline(always)]
    fn from(variant: REGION2WA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION2WA` writer - Write '1' to enable interrupt for REGION\\[2\\].WA event"]
pub type REGION2WA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, REGION2WA_AW, O>;
impl<'a, const O: u8> REGION2WA_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(REGION2WA_AW::SET)
    }
}
#[doc = "Field `REGION2RA` reader - Write '1' to enable interrupt for REGION\\[2\\].RA event"]
pub type REGION2RA_R = crate::BitReader<REGION2RA_A>;
#[doc = "Write '1' to enable interrupt for REGION\\[2\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION2RA_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<REGION2RA_A> for bool {
    #[inline(always)]
    fn from(variant: REGION2RA_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION2RA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION2RA_A {
        match self.bits {
            false => REGION2RA_A::DISABLED,
            true => REGION2RA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION2RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION2RA_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for REGION\\[2\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION2RA_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<REGION2RA_AW> for bool {
    #[inline(always)]
    fn from(variant: REGION2RA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION2RA` writer - Write '1' to enable interrupt for REGION\\[2\\].RA event"]
pub type REGION2RA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, REGION2RA_AW, O>;
impl<'a, const O: u8> REGION2RA_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(REGION2RA_AW::SET)
    }
}
#[doc = "Field `REGION3WA` reader - Write '1' to enable interrupt for REGION\\[3\\].WA event"]
pub type REGION3WA_R = crate::BitReader<REGION3WA_A>;
#[doc = "Write '1' to enable interrupt for REGION\\[3\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION3WA_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<REGION3WA_A> for bool {
    #[inline(always)]
    fn from(variant: REGION3WA_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION3WA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION3WA_A {
        match self.bits {
            false => REGION3WA_A::DISABLED,
            true => REGION3WA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION3WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION3WA_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for REGION\\[3\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION3WA_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<REGION3WA_AW> for bool {
    #[inline(always)]
    fn from(variant: REGION3WA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION3WA` writer - Write '1' to enable interrupt for REGION\\[3\\].WA event"]
pub type REGION3WA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, REGION3WA_AW, O>;
impl<'a, const O: u8> REGION3WA_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(REGION3WA_AW::SET)
    }
}
#[doc = "Field `REGION3RA` reader - Write '1' to enable interrupt for REGION\\[3\\].RA event"]
pub type REGION3RA_R = crate::BitReader<REGION3RA_A>;
#[doc = "Write '1' to enable interrupt for REGION\\[3\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION3RA_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<REGION3RA_A> for bool {
    #[inline(always)]
    fn from(variant: REGION3RA_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION3RA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION3RA_A {
        match self.bits {
            false => REGION3RA_A::DISABLED,
            true => REGION3RA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION3RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION3RA_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for REGION\\[3\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION3RA_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<REGION3RA_AW> for bool {
    #[inline(always)]
    fn from(variant: REGION3RA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION3RA` writer - Write '1' to enable interrupt for REGION\\[3\\].RA event"]
pub type REGION3RA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, REGION3RA_AW, O>;
impl<'a, const O: u8> REGION3RA_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(REGION3RA_AW::SET)
    }
}
#[doc = "Field `PREGION0WA` reader - Write '1' to enable interrupt for PREGION\\[0\\].WA event"]
pub type PREGION0WA_R = crate::BitReader<PREGION0WA_A>;
#[doc = "Write '1' to enable interrupt for PREGION\\[0\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREGION0WA_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<PREGION0WA_A> for bool {
    #[inline(always)]
    fn from(variant: PREGION0WA_A) -> Self {
        variant as u8 != 0
    }
}
impl PREGION0WA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREGION0WA_A {
        match self.bits {
            false => PREGION0WA_A::DISABLED,
            true => PREGION0WA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PREGION0WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PREGION0WA_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for PREGION\\[0\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREGION0WA_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<PREGION0WA_AW> for bool {
    #[inline(always)]
    fn from(variant: PREGION0WA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION0WA` writer - Write '1' to enable interrupt for PREGION\\[0\\].WA event"]
pub type PREGION0WA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, PREGION0WA_AW, O>;
impl<'a, const O: u8> PREGION0WA_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(PREGION0WA_AW::SET)
    }
}
#[doc = "Field `PREGION0RA` reader - Write '1' to enable interrupt for PREGION\\[0\\].RA event"]
pub type PREGION0RA_R = crate::BitReader<PREGION0RA_A>;
#[doc = "Write '1' to enable interrupt for PREGION\\[0\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREGION0RA_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<PREGION0RA_A> for bool {
    #[inline(always)]
    fn from(variant: PREGION0RA_A) -> Self {
        variant as u8 != 0
    }
}
impl PREGION0RA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREGION0RA_A {
        match self.bits {
            false => PREGION0RA_A::DISABLED,
            true => PREGION0RA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PREGION0RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PREGION0RA_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for PREGION\\[0\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREGION0RA_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<PREGION0RA_AW> for bool {
    #[inline(always)]
    fn from(variant: PREGION0RA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION0RA` writer - Write '1' to enable interrupt for PREGION\\[0\\].RA event"]
pub type PREGION0RA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, PREGION0RA_AW, O>;
impl<'a, const O: u8> PREGION0RA_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(PREGION0RA_AW::SET)
    }
}
#[doc = "Field `PREGION1WA` reader - Write '1' to enable interrupt for PREGION\\[1\\].WA event"]
pub type PREGION1WA_R = crate::BitReader<PREGION1WA_A>;
#[doc = "Write '1' to enable interrupt for PREGION\\[1\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREGION1WA_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<PREGION1WA_A> for bool {
    #[inline(always)]
    fn from(variant: PREGION1WA_A) -> Self {
        variant as u8 != 0
    }
}
impl PREGION1WA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREGION1WA_A {
        match self.bits {
            false => PREGION1WA_A::DISABLED,
            true => PREGION1WA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PREGION1WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PREGION1WA_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for PREGION\\[1\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREGION1WA_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<PREGION1WA_AW> for bool {
    #[inline(always)]
    fn from(variant: PREGION1WA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION1WA` writer - Write '1' to enable interrupt for PREGION\\[1\\].WA event"]
pub type PREGION1WA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, PREGION1WA_AW, O>;
impl<'a, const O: u8> PREGION1WA_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(PREGION1WA_AW::SET)
    }
}
#[doc = "Field `PREGION1RA` reader - Write '1' to enable interrupt for PREGION\\[1\\].RA event"]
pub type PREGION1RA_R = crate::BitReader<PREGION1RA_A>;
#[doc = "Write '1' to enable interrupt for PREGION\\[1\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREGION1RA_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<PREGION1RA_A> for bool {
    #[inline(always)]
    fn from(variant: PREGION1RA_A) -> Self {
        variant as u8 != 0
    }
}
impl PREGION1RA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREGION1RA_A {
        match self.bits {
            false => PREGION1RA_A::DISABLED,
            true => PREGION1RA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PREGION1RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PREGION1RA_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for PREGION\\[1\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREGION1RA_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<PREGION1RA_AW> for bool {
    #[inline(always)]
    fn from(variant: PREGION1RA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION1RA` writer - Write '1' to enable interrupt for PREGION\\[1\\].RA event"]
pub type PREGION1RA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, PREGION1RA_AW, O>;
impl<'a, const O: u8> PREGION1RA_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(PREGION1RA_AW::SET)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for REGION\\[0\\].WA event"]
    #[inline(always)]
    pub fn region0wa(&self) -> REGION0WA_R {
        REGION0WA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for REGION\\[0\\].RA event"]
    #[inline(always)]
    pub fn region0ra(&self) -> REGION0RA_R {
        REGION0RA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for REGION\\[1\\].WA event"]
    #[inline(always)]
    pub fn region1wa(&self) -> REGION1WA_R {
        REGION1WA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for REGION\\[1\\].RA event"]
    #[inline(always)]
    pub fn region1ra(&self) -> REGION1RA_R {
        REGION1RA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for REGION\\[2\\].WA event"]
    #[inline(always)]
    pub fn region2wa(&self) -> REGION2WA_R {
        REGION2WA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for REGION\\[2\\].RA event"]
    #[inline(always)]
    pub fn region2ra(&self) -> REGION2RA_R {
        REGION2RA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for REGION\\[3\\].WA event"]
    #[inline(always)]
    pub fn region3wa(&self) -> REGION3WA_R {
        REGION3WA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for REGION\\[3\\].RA event"]
    #[inline(always)]
    pub fn region3ra(&self) -> REGION3RA_R {
        REGION3RA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 24 - Write '1' to enable interrupt for PREGION\\[0\\].WA event"]
    #[inline(always)]
    pub fn pregion0wa(&self) -> PREGION0WA_R {
        PREGION0WA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write '1' to enable interrupt for PREGION\\[0\\].RA event"]
    #[inline(always)]
    pub fn pregion0ra(&self) -> PREGION0RA_R {
        PREGION0RA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Write '1' to enable interrupt for PREGION\\[1\\].WA event"]
    #[inline(always)]
    pub fn pregion1wa(&self) -> PREGION1WA_R {
        PREGION1WA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Write '1' to enable interrupt for PREGION\\[1\\].RA event"]
    #[inline(always)]
    pub fn pregion1ra(&self) -> PREGION1RA_R {
        PREGION1RA_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for REGION\\[0\\].WA event"]
    #[inline(always)]
    pub fn region0wa(&mut self) -> REGION0WA_W<0> {
        REGION0WA_W::new(self)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for REGION\\[0\\].RA event"]
    #[inline(always)]
    pub fn region0ra(&mut self) -> REGION0RA_W<1> {
        REGION0RA_W::new(self)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for REGION\\[1\\].WA event"]
    #[inline(always)]
    pub fn region1wa(&mut self) -> REGION1WA_W<2> {
        REGION1WA_W::new(self)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for REGION\\[1\\].RA event"]
    #[inline(always)]
    pub fn region1ra(&mut self) -> REGION1RA_W<3> {
        REGION1RA_W::new(self)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for REGION\\[2\\].WA event"]
    #[inline(always)]
    pub fn region2wa(&mut self) -> REGION2WA_W<4> {
        REGION2WA_W::new(self)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for REGION\\[2\\].RA event"]
    #[inline(always)]
    pub fn region2ra(&mut self) -> REGION2RA_W<5> {
        REGION2RA_W::new(self)
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for REGION\\[3\\].WA event"]
    #[inline(always)]
    pub fn region3wa(&mut self) -> REGION3WA_W<6> {
        REGION3WA_W::new(self)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for REGION\\[3\\].RA event"]
    #[inline(always)]
    pub fn region3ra(&mut self) -> REGION3RA_W<7> {
        REGION3RA_W::new(self)
    }
    #[doc = "Bit 24 - Write '1' to enable interrupt for PREGION\\[0\\].WA event"]
    #[inline(always)]
    pub fn pregion0wa(&mut self) -> PREGION0WA_W<24> {
        PREGION0WA_W::new(self)
    }
    #[doc = "Bit 25 - Write '1' to enable interrupt for PREGION\\[0\\].RA event"]
    #[inline(always)]
    pub fn pregion0ra(&mut self) -> PREGION0RA_W<25> {
        PREGION0RA_W::new(self)
    }
    #[doc = "Bit 26 - Write '1' to enable interrupt for PREGION\\[1\\].WA event"]
    #[inline(always)]
    pub fn pregion1wa(&mut self) -> PREGION1WA_W<26> {
        PREGION1WA_W::new(self)
    }
    #[doc = "Bit 27 - Write '1' to enable interrupt for PREGION\\[1\\].RA event"]
    #[inline(always)]
    pub fn pregion1ra(&mut self) -> PREGION1RA_W<27> {
        PREGION1RA_W::new(self)
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
