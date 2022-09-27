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
#[doc = "Field `RECEIVE0` reader - Enable or disable interrupt for event RECEIVE\\[0\\]"]
pub type RECEIVE0_R = crate::BitReader<RECEIVE0_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE0_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE0_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE0_A {
        match self.bits {
            false => RECEIVE0_A::DISABLED,
            true => RECEIVE0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE0_A::ENABLED
    }
}
#[doc = "Field `RECEIVE0` writer - Enable or disable interrupt for event RECEIVE\\[0\\]"]
pub type RECEIVE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE0_A, O>;
impl<'a, const O: u8> RECEIVE0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE0_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE1` reader - Enable or disable interrupt for event RECEIVE\\[1\\]"]
pub type RECEIVE1_R = crate::BitReader<RECEIVE1_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE1_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE1_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE1_A {
        match self.bits {
            false => RECEIVE1_A::DISABLED,
            true => RECEIVE1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE1_A::ENABLED
    }
}
#[doc = "Field `RECEIVE1` writer - Enable or disable interrupt for event RECEIVE\\[1\\]"]
pub type RECEIVE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE1_A, O>;
impl<'a, const O: u8> RECEIVE1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE1_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE2` reader - Enable or disable interrupt for event RECEIVE\\[2\\]"]
pub type RECEIVE2_R = crate::BitReader<RECEIVE2_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE2_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE2_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE2_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE2_A {
        match self.bits {
            false => RECEIVE2_A::DISABLED,
            true => RECEIVE2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE2_A::ENABLED
    }
}
#[doc = "Field `RECEIVE2` writer - Enable or disable interrupt for event RECEIVE\\[2\\]"]
pub type RECEIVE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE2_A, O>;
impl<'a, const O: u8> RECEIVE2_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE2_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE2_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE3` reader - Enable or disable interrupt for event RECEIVE\\[3\\]"]
pub type RECEIVE3_R = crate::BitReader<RECEIVE3_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE3_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE3_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE3_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE3_A {
        match self.bits {
            false => RECEIVE3_A::DISABLED,
            true => RECEIVE3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE3_A::ENABLED
    }
}
#[doc = "Field `RECEIVE3` writer - Enable or disable interrupt for event RECEIVE\\[3\\]"]
pub type RECEIVE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE3_A, O>;
impl<'a, const O: u8> RECEIVE3_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE3_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE3_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE4` reader - Enable or disable interrupt for event RECEIVE\\[4\\]"]
pub type RECEIVE4_R = crate::BitReader<RECEIVE4_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE4_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE4_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE4_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE4_A {
        match self.bits {
            false => RECEIVE4_A::DISABLED,
            true => RECEIVE4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE4_A::ENABLED
    }
}
#[doc = "Field `RECEIVE4` writer - Enable or disable interrupt for event RECEIVE\\[4\\]"]
pub type RECEIVE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE4_A, O>;
impl<'a, const O: u8> RECEIVE4_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE4_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE4_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE5` reader - Enable or disable interrupt for event RECEIVE\\[5\\]"]
pub type RECEIVE5_R = crate::BitReader<RECEIVE5_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE5_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE5_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE5_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE5_A {
        match self.bits {
            false => RECEIVE5_A::DISABLED,
            true => RECEIVE5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE5_A::ENABLED
    }
}
#[doc = "Field `RECEIVE5` writer - Enable or disable interrupt for event RECEIVE\\[5\\]"]
pub type RECEIVE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE5_A, O>;
impl<'a, const O: u8> RECEIVE5_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE5_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE5_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE6` reader - Enable or disable interrupt for event RECEIVE\\[6\\]"]
pub type RECEIVE6_R = crate::BitReader<RECEIVE6_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE6_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE6_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE6_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE6_A {
        match self.bits {
            false => RECEIVE6_A::DISABLED,
            true => RECEIVE6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE6_A::ENABLED
    }
}
#[doc = "Field `RECEIVE6` writer - Enable or disable interrupt for event RECEIVE\\[6\\]"]
pub type RECEIVE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE6_A, O>;
impl<'a, const O: u8> RECEIVE6_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE6_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE6_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE7` reader - Enable or disable interrupt for event RECEIVE\\[7\\]"]
pub type RECEIVE7_R = crate::BitReader<RECEIVE7_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE7_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE7_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE7_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE7_A {
        match self.bits {
            false => RECEIVE7_A::DISABLED,
            true => RECEIVE7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE7_A::ENABLED
    }
}
#[doc = "Field `RECEIVE7` writer - Enable or disable interrupt for event RECEIVE\\[7\\]"]
pub type RECEIVE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE7_A, O>;
impl<'a, const O: u8> RECEIVE7_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE7_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE7_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE8` reader - Enable or disable interrupt for event RECEIVE\\[8\\]"]
pub type RECEIVE8_R = crate::BitReader<RECEIVE8_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[8\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE8_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE8_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE8_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE8_A {
        match self.bits {
            false => RECEIVE8_A::DISABLED,
            true => RECEIVE8_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE8_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE8_A::ENABLED
    }
}
#[doc = "Field `RECEIVE8` writer - Enable or disable interrupt for event RECEIVE\\[8\\]"]
pub type RECEIVE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE8_A, O>;
impl<'a, const O: u8> RECEIVE8_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE8_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE8_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE9` reader - Enable or disable interrupt for event RECEIVE\\[9\\]"]
pub type RECEIVE9_R = crate::BitReader<RECEIVE9_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[9\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE9_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE9_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE9_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE9_A {
        match self.bits {
            false => RECEIVE9_A::DISABLED,
            true => RECEIVE9_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE9_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE9_A::ENABLED
    }
}
#[doc = "Field `RECEIVE9` writer - Enable or disable interrupt for event RECEIVE\\[9\\]"]
pub type RECEIVE9_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE9_A, O>;
impl<'a, const O: u8> RECEIVE9_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE9_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE9_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE10` reader - Enable or disable interrupt for event RECEIVE\\[10\\]"]
pub type RECEIVE10_R = crate::BitReader<RECEIVE10_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[10\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE10_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE10_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE10_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE10_A {
        match self.bits {
            false => RECEIVE10_A::DISABLED,
            true => RECEIVE10_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE10_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE10_A::ENABLED
    }
}
#[doc = "Field `RECEIVE10` writer - Enable or disable interrupt for event RECEIVE\\[10\\]"]
pub type RECEIVE10_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE10_A, O>;
impl<'a, const O: u8> RECEIVE10_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE10_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE10_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE11` reader - Enable or disable interrupt for event RECEIVE\\[11\\]"]
pub type RECEIVE11_R = crate::BitReader<RECEIVE11_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[11\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE11_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE11_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE11_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE11_A {
        match self.bits {
            false => RECEIVE11_A::DISABLED,
            true => RECEIVE11_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE11_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE11_A::ENABLED
    }
}
#[doc = "Field `RECEIVE11` writer - Enable or disable interrupt for event RECEIVE\\[11\\]"]
pub type RECEIVE11_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE11_A, O>;
impl<'a, const O: u8> RECEIVE11_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE11_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE11_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE12` reader - Enable or disable interrupt for event RECEIVE\\[12\\]"]
pub type RECEIVE12_R = crate::BitReader<RECEIVE12_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[12\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE12_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE12_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE12_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE12_A {
        match self.bits {
            false => RECEIVE12_A::DISABLED,
            true => RECEIVE12_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE12_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE12_A::ENABLED
    }
}
#[doc = "Field `RECEIVE12` writer - Enable or disable interrupt for event RECEIVE\\[12\\]"]
pub type RECEIVE12_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE12_A, O>;
impl<'a, const O: u8> RECEIVE12_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE12_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE12_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE13` reader - Enable or disable interrupt for event RECEIVE\\[13\\]"]
pub type RECEIVE13_R = crate::BitReader<RECEIVE13_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[13\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE13_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE13_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE13_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE13_A {
        match self.bits {
            false => RECEIVE13_A::DISABLED,
            true => RECEIVE13_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE13_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE13_A::ENABLED
    }
}
#[doc = "Field `RECEIVE13` writer - Enable or disable interrupt for event RECEIVE\\[13\\]"]
pub type RECEIVE13_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE13_A, O>;
impl<'a, const O: u8> RECEIVE13_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE13_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE13_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE14` reader - Enable or disable interrupt for event RECEIVE\\[14\\]"]
pub type RECEIVE14_R = crate::BitReader<RECEIVE14_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[14\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE14_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE14_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE14_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE14_A {
        match self.bits {
            false => RECEIVE14_A::DISABLED,
            true => RECEIVE14_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE14_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE14_A::ENABLED
    }
}
#[doc = "Field `RECEIVE14` writer - Enable or disable interrupt for event RECEIVE\\[14\\]"]
pub type RECEIVE14_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE14_A, O>;
impl<'a, const O: u8> RECEIVE14_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE14_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE14_A::ENABLED)
    }
}
#[doc = "Field `RECEIVE15` reader - Enable or disable interrupt for event RECEIVE\\[15\\]"]
pub type RECEIVE15_R = crate::BitReader<RECEIVE15_A>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[15\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE15_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RECEIVE15_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE15_A) -> Self {
        variant as u8 != 0
    }
}
impl RECEIVE15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE15_A {
        match self.bits {
            false => RECEIVE15_A::DISABLED,
            true => RECEIVE15_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE15_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE15_A::ENABLED
    }
}
#[doc = "Field `RECEIVE15` writer - Enable or disable interrupt for event RECEIVE\\[15\\]"]
pub type RECEIVE15_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, RECEIVE15_A, O>;
impl<'a, const O: u8> RECEIVE15_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RECEIVE15_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RECEIVE15_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event RECEIVE\\[0\\]"]
    #[inline(always)]
    pub fn receive0(&self) -> RECEIVE0_R {
        RECEIVE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event RECEIVE\\[1\\]"]
    #[inline(always)]
    pub fn receive1(&self) -> RECEIVE1_R {
        RECEIVE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event RECEIVE\\[2\\]"]
    #[inline(always)]
    pub fn receive2(&self) -> RECEIVE2_R {
        RECEIVE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event RECEIVE\\[3\\]"]
    #[inline(always)]
    pub fn receive3(&self) -> RECEIVE3_R {
        RECEIVE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event RECEIVE\\[4\\]"]
    #[inline(always)]
    pub fn receive4(&self) -> RECEIVE4_R {
        RECEIVE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event RECEIVE\\[5\\]"]
    #[inline(always)]
    pub fn receive5(&self) -> RECEIVE5_R {
        RECEIVE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event RECEIVE\\[6\\]"]
    #[inline(always)]
    pub fn receive6(&self) -> RECEIVE6_R {
        RECEIVE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event RECEIVE\\[7\\]"]
    #[inline(always)]
    pub fn receive7(&self) -> RECEIVE7_R {
        RECEIVE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for event RECEIVE\\[8\\]"]
    #[inline(always)]
    pub fn receive8(&self) -> RECEIVE8_R {
        RECEIVE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event RECEIVE\\[9\\]"]
    #[inline(always)]
    pub fn receive9(&self) -> RECEIVE9_R {
        RECEIVE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for event RECEIVE\\[10\\]"]
    #[inline(always)]
    pub fn receive10(&self) -> RECEIVE10_R {
        RECEIVE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable or disable interrupt for event RECEIVE\\[11\\]"]
    #[inline(always)]
    pub fn receive11(&self) -> RECEIVE11_R {
        RECEIVE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable or disable interrupt for event RECEIVE\\[12\\]"]
    #[inline(always)]
    pub fn receive12(&self) -> RECEIVE12_R {
        RECEIVE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for event RECEIVE\\[13\\]"]
    #[inline(always)]
    pub fn receive13(&self) -> RECEIVE13_R {
        RECEIVE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for event RECEIVE\\[14\\]"]
    #[inline(always)]
    pub fn receive14(&self) -> RECEIVE14_R {
        RECEIVE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable or disable interrupt for event RECEIVE\\[15\\]"]
    #[inline(always)]
    pub fn receive15(&self) -> RECEIVE15_R {
        RECEIVE15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event RECEIVE\\[0\\]"]
    #[inline(always)]
    pub fn receive0(&mut self) -> RECEIVE0_W<0> {
        RECEIVE0_W::new(self)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event RECEIVE\\[1\\]"]
    #[inline(always)]
    pub fn receive1(&mut self) -> RECEIVE1_W<1> {
        RECEIVE1_W::new(self)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event RECEIVE\\[2\\]"]
    #[inline(always)]
    pub fn receive2(&mut self) -> RECEIVE2_W<2> {
        RECEIVE2_W::new(self)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event RECEIVE\\[3\\]"]
    #[inline(always)]
    pub fn receive3(&mut self) -> RECEIVE3_W<3> {
        RECEIVE3_W::new(self)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event RECEIVE\\[4\\]"]
    #[inline(always)]
    pub fn receive4(&mut self) -> RECEIVE4_W<4> {
        RECEIVE4_W::new(self)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event RECEIVE\\[5\\]"]
    #[inline(always)]
    pub fn receive5(&mut self) -> RECEIVE5_W<5> {
        RECEIVE5_W::new(self)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event RECEIVE\\[6\\]"]
    #[inline(always)]
    pub fn receive6(&mut self) -> RECEIVE6_W<6> {
        RECEIVE6_W::new(self)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event RECEIVE\\[7\\]"]
    #[inline(always)]
    pub fn receive7(&mut self) -> RECEIVE7_W<7> {
        RECEIVE7_W::new(self)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for event RECEIVE\\[8\\]"]
    #[inline(always)]
    pub fn receive8(&mut self) -> RECEIVE8_W<8> {
        RECEIVE8_W::new(self)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event RECEIVE\\[9\\]"]
    #[inline(always)]
    pub fn receive9(&mut self) -> RECEIVE9_W<9> {
        RECEIVE9_W::new(self)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for event RECEIVE\\[10\\]"]
    #[inline(always)]
    pub fn receive10(&mut self) -> RECEIVE10_W<10> {
        RECEIVE10_W::new(self)
    }
    #[doc = "Bit 11 - Enable or disable interrupt for event RECEIVE\\[11\\]"]
    #[inline(always)]
    pub fn receive11(&mut self) -> RECEIVE11_W<11> {
        RECEIVE11_W::new(self)
    }
    #[doc = "Bit 12 - Enable or disable interrupt for event RECEIVE\\[12\\]"]
    #[inline(always)]
    pub fn receive12(&mut self) -> RECEIVE12_W<12> {
        RECEIVE12_W::new(self)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for event RECEIVE\\[13\\]"]
    #[inline(always)]
    pub fn receive13(&mut self) -> RECEIVE13_W<13> {
        RECEIVE13_W::new(self)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for event RECEIVE\\[14\\]"]
    #[inline(always)]
    pub fn receive14(&mut self) -> RECEIVE14_W<14> {
        RECEIVE14_W::new(self)
    }
    #[doc = "Bit 15 - Enable or disable interrupt for event RECEIVE\\[15\\]"]
    #[inline(always)]
    pub fn receive15(&mut self) -> RECEIVE15_W<15> {
        RECEIVE15_W::new(self)
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
