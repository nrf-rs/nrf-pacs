#[doc = "Register `CHENSET` reader"]
pub struct R(crate::R<CHENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHENSET` writer"]
pub struct W(crate::W<CHENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHENSET_SPEC>;
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
impl From<crate::W<CHENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` reader - Channel 0 enable set register. Writing '0' has no effect"]
pub type CH0_R = crate::BitReader<CH0_A>;
#[doc = "Channel 0 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH0_A> for bool {
    #[inline(always)]
    fn from(variant: CH0_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0_A {
        match self.bits {
            false => CH0_A::DISABLED,
            true => CH0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH0_A::ENABLED
    }
}
#[doc = "Channel 0 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0_AW {
    #[doc = "1: Write: Enable channel"]
    SET = 1,
}
impl From<CH0_AW> for bool {
    #[inline(always)]
    fn from(variant: CH0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` writer - Channel 0 enable set register. Writing '0' has no effect"]
pub type CH0_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CHENSET_SPEC, CH0_AW, O>;
impl<'a, const O: u8> CH0_W<'a, O> {
    #[doc = "Write: Enable channel"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH0_AW::SET)
    }
}
#[doc = "Field `CH1` reader - Channel 1 enable set register. Writing '0' has no effect"]
pub type CH1_R = crate::BitReader<CH1_A>;
#[doc = "Channel 1 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH1_A> for bool {
    #[inline(always)]
    fn from(variant: CH1_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1_A {
        match self.bits {
            false => CH1_A::DISABLED,
            true => CH1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH1_A::ENABLED
    }
}
#[doc = "Channel 1 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1_AW {
    #[doc = "1: Write: Enable channel"]
    SET = 1,
}
impl From<CH1_AW> for bool {
    #[inline(always)]
    fn from(variant: CH1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1` writer - Channel 1 enable set register. Writing '0' has no effect"]
pub type CH1_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CHENSET_SPEC, CH1_AW, O>;
impl<'a, const O: u8> CH1_W<'a, O> {
    #[doc = "Write: Enable channel"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH1_AW::SET)
    }
}
#[doc = "Field `CH2` reader - Channel 2 enable set register. Writing '0' has no effect"]
pub type CH2_R = crate::BitReader<CH2_A>;
#[doc = "Channel 2 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH2_A> for bool {
    #[inline(always)]
    fn from(variant: CH2_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2_A {
        match self.bits {
            false => CH2_A::DISABLED,
            true => CH2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH2_A::ENABLED
    }
}
#[doc = "Channel 2 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2_AW {
    #[doc = "1: Write: Enable channel"]
    SET = 1,
}
impl From<CH2_AW> for bool {
    #[inline(always)]
    fn from(variant: CH2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2` writer - Channel 2 enable set register. Writing '0' has no effect"]
pub type CH2_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CHENSET_SPEC, CH2_AW, O>;
impl<'a, const O: u8> CH2_W<'a, O> {
    #[doc = "Write: Enable channel"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH2_AW::SET)
    }
}
#[doc = "Field `CH3` reader - Channel 3 enable set register. Writing '0' has no effect"]
pub type CH3_R = crate::BitReader<CH3_A>;
#[doc = "Channel 3 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH3_A> for bool {
    #[inline(always)]
    fn from(variant: CH3_A) -> Self {
        variant as u8 != 0
    }
}
impl CH3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3_A {
        match self.bits {
            false => CH3_A::DISABLED,
            true => CH3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH3_A::ENABLED
    }
}
#[doc = "Channel 3 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3_AW {
    #[doc = "1: Write: Enable channel"]
    SET = 1,
}
impl From<CH3_AW> for bool {
    #[inline(always)]
    fn from(variant: CH3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3` writer - Channel 3 enable set register. Writing '0' has no effect"]
pub type CH3_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CHENSET_SPEC, CH3_AW, O>;
impl<'a, const O: u8> CH3_W<'a, O> {
    #[doc = "Write: Enable channel"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH3_AW::SET)
    }
}
#[doc = "Field `CH4` reader - Channel 4 enable set register. Writing '0' has no effect"]
pub type CH4_R = crate::BitReader<CH4_A>;
#[doc = "Channel 4 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH4_A> for bool {
    #[inline(always)]
    fn from(variant: CH4_A) -> Self {
        variant as u8 != 0
    }
}
impl CH4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4_A {
        match self.bits {
            false => CH4_A::DISABLED,
            true => CH4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH4_A::ENABLED
    }
}
#[doc = "Channel 4 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4_AW {
    #[doc = "1: Write: Enable channel"]
    SET = 1,
}
impl From<CH4_AW> for bool {
    #[inline(always)]
    fn from(variant: CH4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4` writer - Channel 4 enable set register. Writing '0' has no effect"]
pub type CH4_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CHENSET_SPEC, CH4_AW, O>;
impl<'a, const O: u8> CH4_W<'a, O> {
    #[doc = "Write: Enable channel"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH4_AW::SET)
    }
}
#[doc = "Field `CH5` reader - Channel 5 enable set register. Writing '0' has no effect"]
pub type CH5_R = crate::BitReader<CH5_A>;
#[doc = "Channel 5 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH5_A> for bool {
    #[inline(always)]
    fn from(variant: CH5_A) -> Self {
        variant as u8 != 0
    }
}
impl CH5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5_A {
        match self.bits {
            false => CH5_A::DISABLED,
            true => CH5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH5_A::ENABLED
    }
}
#[doc = "Channel 5 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5_AW {
    #[doc = "1: Write: Enable channel"]
    SET = 1,
}
impl From<CH5_AW> for bool {
    #[inline(always)]
    fn from(variant: CH5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5` writer - Channel 5 enable set register. Writing '0' has no effect"]
pub type CH5_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CHENSET_SPEC, CH5_AW, O>;
impl<'a, const O: u8> CH5_W<'a, O> {
    #[doc = "Write: Enable channel"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH5_AW::SET)
    }
}
#[doc = "Field `CH6` reader - Channel 6 enable set register. Writing '0' has no effect"]
pub type CH6_R = crate::BitReader<CH6_A>;
#[doc = "Channel 6 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH6_A> for bool {
    #[inline(always)]
    fn from(variant: CH6_A) -> Self {
        variant as u8 != 0
    }
}
impl CH6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6_A {
        match self.bits {
            false => CH6_A::DISABLED,
            true => CH6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH6_A::ENABLED
    }
}
#[doc = "Channel 6 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6_AW {
    #[doc = "1: Write: Enable channel"]
    SET = 1,
}
impl From<CH6_AW> for bool {
    #[inline(always)]
    fn from(variant: CH6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6` writer - Channel 6 enable set register. Writing '0' has no effect"]
pub type CH6_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CHENSET_SPEC, CH6_AW, O>;
impl<'a, const O: u8> CH6_W<'a, O> {
    #[doc = "Write: Enable channel"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH6_AW::SET)
    }
}
#[doc = "Field `CH7` reader - Channel 7 enable set register. Writing '0' has no effect"]
pub type CH7_R = crate::BitReader<CH7_A>;
#[doc = "Channel 7 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH7_A> for bool {
    #[inline(always)]
    fn from(variant: CH7_A) -> Self {
        variant as u8 != 0
    }
}
impl CH7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7_A {
        match self.bits {
            false => CH7_A::DISABLED,
            true => CH7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH7_A::ENABLED
    }
}
#[doc = "Channel 7 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7_AW {
    #[doc = "1: Write: Enable channel"]
    SET = 1,
}
impl From<CH7_AW> for bool {
    #[inline(always)]
    fn from(variant: CH7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7` writer - Channel 7 enable set register. Writing '0' has no effect"]
pub type CH7_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CHENSET_SPEC, CH7_AW, O>;
impl<'a, const O: u8> CH7_W<'a, O> {
    #[doc = "Write: Enable channel"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH7_AW::SET)
    }
}
#[doc = "Field `CH8` reader - Channel 8 enable set register. Writing '0' has no effect"]
pub type CH8_R = crate::BitReader<CH8_A>;
#[doc = "Channel 8 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH8_A> for bool {
    #[inline(always)]
    fn from(variant: CH8_A) -> Self {
        variant as u8 != 0
    }
}
impl CH8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH8_A {
        match self.bits {
            false => CH8_A::DISABLED,
            true => CH8_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH8_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH8_A::ENABLED
    }
}
#[doc = "Channel 8 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8_AW {
    #[doc = "1: Write: Enable channel"]
    SET = 1,
}
impl From<CH8_AW> for bool {
    #[inline(always)]
    fn from(variant: CH8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH8` writer - Channel 8 enable set register. Writing '0' has no effect"]
pub type CH8_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CHENSET_SPEC, CH8_AW, O>;
impl<'a, const O: u8> CH8_W<'a, O> {
    #[doc = "Write: Enable channel"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH8_AW::SET)
    }
}
#[doc = "Field `CH9` reader - Channel 9 enable set register. Writing '0' has no effect"]
pub type CH9_R = crate::BitReader<CH9_A>;
#[doc = "Channel 9 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH9_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH9_A> for bool {
    #[inline(always)]
    fn from(variant: CH9_A) -> Self {
        variant as u8 != 0
    }
}
impl CH9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH9_A {
        match self.bits {
            false => CH9_A::DISABLED,
            true => CH9_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH9_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH9_A::ENABLED
    }
}
#[doc = "Channel 9 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH9_AW {
    #[doc = "1: Write: Enable channel"]
    SET = 1,
}
impl From<CH9_AW> for bool {
    #[inline(always)]
    fn from(variant: CH9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH9` writer - Channel 9 enable set register. Writing '0' has no effect"]
pub type CH9_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CHENSET_SPEC, CH9_AW, O>;
impl<'a, const O: u8> CH9_W<'a, O> {
    #[doc = "Write: Enable channel"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH9_AW::SET)
    }
}
#[doc = "Field `CH10` reader - Channel 10 enable set register. Writing '0' has no effect"]
pub type CH10_R = crate::BitReader<CH10_A>;
#[doc = "Channel 10 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH10_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH10_A> for bool {
    #[inline(always)]
    fn from(variant: CH10_A) -> Self {
        variant as u8 != 0
    }
}
impl CH10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH10_A {
        match self.bits {
            false => CH10_A::DISABLED,
            true => CH10_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH10_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH10_A::ENABLED
    }
}
#[doc = "Channel 10 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH10_AW {
    #[doc = "1: Write: Enable channel"]
    SET = 1,
}
impl From<CH10_AW> for bool {
    #[inline(always)]
    fn from(variant: CH10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH10` writer - Channel 10 enable set register. Writing '0' has no effect"]
pub type CH10_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CHENSET_SPEC, CH10_AW, O>;
impl<'a, const O: u8> CH10_W<'a, O> {
    #[doc = "Write: Enable channel"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH10_AW::SET)
    }
}
#[doc = "Field `CH11` reader - Channel 11 enable set register. Writing '0' has no effect"]
pub type CH11_R = crate::BitReader<CH11_A>;
#[doc = "Channel 11 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH11_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH11_A> for bool {
    #[inline(always)]
    fn from(variant: CH11_A) -> Self {
        variant as u8 != 0
    }
}
impl CH11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH11_A {
        match self.bits {
            false => CH11_A::DISABLED,
            true => CH11_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH11_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH11_A::ENABLED
    }
}
#[doc = "Channel 11 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH11_AW {
    #[doc = "1: Write: Enable channel"]
    SET = 1,
}
impl From<CH11_AW> for bool {
    #[inline(always)]
    fn from(variant: CH11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH11` writer - Channel 11 enable set register. Writing '0' has no effect"]
pub type CH11_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CHENSET_SPEC, CH11_AW, O>;
impl<'a, const O: u8> CH11_W<'a, O> {
    #[doc = "Write: Enable channel"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH11_AW::SET)
    }
}
#[doc = "Field `CH12` reader - Channel 12 enable set register. Writing '0' has no effect"]
pub type CH12_R = crate::BitReader<CH12_A>;
#[doc = "Channel 12 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH12_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH12_A> for bool {
    #[inline(always)]
    fn from(variant: CH12_A) -> Self {
        variant as u8 != 0
    }
}
impl CH12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH12_A {
        match self.bits {
            false => CH12_A::DISABLED,
            true => CH12_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH12_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH12_A::ENABLED
    }
}
#[doc = "Channel 12 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH12_AW {
    #[doc = "1: Write: Enable channel"]
    SET = 1,
}
impl From<CH12_AW> for bool {
    #[inline(always)]
    fn from(variant: CH12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH12` writer - Channel 12 enable set register. Writing '0' has no effect"]
pub type CH12_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CHENSET_SPEC, CH12_AW, O>;
impl<'a, const O: u8> CH12_W<'a, O> {
    #[doc = "Write: Enable channel"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH12_AW::SET)
    }
}
#[doc = "Field `CH13` reader - Channel 13 enable set register. Writing '0' has no effect"]
pub type CH13_R = crate::BitReader<CH13_A>;
#[doc = "Channel 13 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH13_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH13_A> for bool {
    #[inline(always)]
    fn from(variant: CH13_A) -> Self {
        variant as u8 != 0
    }
}
impl CH13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH13_A {
        match self.bits {
            false => CH13_A::DISABLED,
            true => CH13_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH13_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH13_A::ENABLED
    }
}
#[doc = "Channel 13 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH13_AW {
    #[doc = "1: Write: Enable channel"]
    SET = 1,
}
impl From<CH13_AW> for bool {
    #[inline(always)]
    fn from(variant: CH13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH13` writer - Channel 13 enable set register. Writing '0' has no effect"]
pub type CH13_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CHENSET_SPEC, CH13_AW, O>;
impl<'a, const O: u8> CH13_W<'a, O> {
    #[doc = "Write: Enable channel"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH13_AW::SET)
    }
}
#[doc = "Field `CH14` reader - Channel 14 enable set register. Writing '0' has no effect"]
pub type CH14_R = crate::BitReader<CH14_A>;
#[doc = "Channel 14 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH14_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH14_A> for bool {
    #[inline(always)]
    fn from(variant: CH14_A) -> Self {
        variant as u8 != 0
    }
}
impl CH14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH14_A {
        match self.bits {
            false => CH14_A::DISABLED,
            true => CH14_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH14_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH14_A::ENABLED
    }
}
#[doc = "Channel 14 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH14_AW {
    #[doc = "1: Write: Enable channel"]
    SET = 1,
}
impl From<CH14_AW> for bool {
    #[inline(always)]
    fn from(variant: CH14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH14` writer - Channel 14 enable set register. Writing '0' has no effect"]
pub type CH14_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CHENSET_SPEC, CH14_AW, O>;
impl<'a, const O: u8> CH14_W<'a, O> {
    #[doc = "Write: Enable channel"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH14_AW::SET)
    }
}
#[doc = "Field `CH15` reader - Channel 15 enable set register. Writing '0' has no effect"]
pub type CH15_R = crate::BitReader<CH15_A>;
#[doc = "Channel 15 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH15_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH15_A> for bool {
    #[inline(always)]
    fn from(variant: CH15_A) -> Self {
        variant as u8 != 0
    }
}
impl CH15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH15_A {
        match self.bits {
            false => CH15_A::DISABLED,
            true => CH15_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH15_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH15_A::ENABLED
    }
}
#[doc = "Channel 15 enable set register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH15_AW {
    #[doc = "1: Write: Enable channel"]
    SET = 1,
}
impl From<CH15_AW> for bool {
    #[inline(always)]
    fn from(variant: CH15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH15` writer - Channel 15 enable set register. Writing '0' has no effect"]
pub type CH15_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, CHENSET_SPEC, CH15_AW, O>;
impl<'a, const O: u8> CH15_W<'a, O> {
    #[doc = "Write: Enable channel"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH15_AW::SET)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 12 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch12(&self) -> CH12_R {
        CH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 13 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch13(&self) -> CH13_R {
        CH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 14 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch14(&self) -> CH14_R {
        CH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 15 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch15(&self) -> CH15_R {
        CH15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W<1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W<2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W<3> {
        CH3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch4(&mut self) -> CH4_W<4> {
        CH4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch5(&mut self) -> CH5_W<5> {
        CH5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch6(&mut self) -> CH6_W<6> {
        CH6_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch7(&mut self) -> CH7_W<7> {
        CH7_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch8(&mut self) -> CH8_W<8> {
        CH8_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch9(&mut self) -> CH9_W<9> {
        CH9_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch10(&mut self) -> CH10_W<10> {
        CH10_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch11(&mut self) -> CH11_W<11> {
        CH11_W::new(self)
    }
    #[doc = "Bit 12 - Channel 12 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch12(&mut self) -> CH12_W<12> {
        CH12_W::new(self)
    }
    #[doc = "Bit 13 - Channel 13 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch13(&mut self) -> CH13_W<13> {
        CH13_W::new(self)
    }
    #[doc = "Bit 14 - Channel 14 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch14(&mut self) -> CH14_W<14> {
        CH14_W::new(self)
    }
    #[doc = "Bit 15 - Channel 15 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch15(&mut self) -> CH15_W<15> {
        CH15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel enable set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chenset](index.html) module"]
pub struct CHENSET_SPEC;
impl crate::RegisterSpec for CHENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chenset::R](R) reader structure"]
impl crate::Readable for CHENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chenset::W](W) writer structure"]
impl crate::Writable for CHENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHENSET to value 0"]
impl crate::Resettable for CHENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
