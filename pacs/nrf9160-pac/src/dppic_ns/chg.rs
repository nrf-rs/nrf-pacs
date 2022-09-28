#[doc = "Register `CHG[%s]` reader"]
pub struct R(crate::R<CHG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHG[%s]` writer"]
pub struct W(crate::W<CHG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHG_SPEC>;
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
impl From<crate::W<CHG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` reader - Include or exclude channel 0"]
pub type CH0_R = crate::BitReader<CH0_A>;
#[doc = "Include or exclude channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
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
            false => CH0_A::EXCLUDED,
            true => CH0_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH0_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH0_A::INCLUDED
    }
}
#[doc = "Field `CH0` writer - Include or exclude channel 0"]
pub type CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH0_A, O>;
impl<'a, const O: u8> CH0_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH0_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH0_A::INCLUDED)
    }
}
#[doc = "Field `CH1` reader - Include or exclude channel 1"]
pub type CH1_R = crate::BitReader<CH1_A>;
#[doc = "Include or exclude channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
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
            false => CH1_A::EXCLUDED,
            true => CH1_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH1_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH1_A::INCLUDED
    }
}
#[doc = "Field `CH1` writer - Include or exclude channel 1"]
pub type CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH1_A, O>;
impl<'a, const O: u8> CH1_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH1_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH1_A::INCLUDED)
    }
}
#[doc = "Field `CH2` reader - Include or exclude channel 2"]
pub type CH2_R = crate::BitReader<CH2_A>;
#[doc = "Include or exclude channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
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
            false => CH2_A::EXCLUDED,
            true => CH2_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH2_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH2_A::INCLUDED
    }
}
#[doc = "Field `CH2` writer - Include or exclude channel 2"]
pub type CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH2_A, O>;
impl<'a, const O: u8> CH2_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH2_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH2_A::INCLUDED)
    }
}
#[doc = "Field `CH3` reader - Include or exclude channel 3"]
pub type CH3_R = crate::BitReader<CH3_A>;
#[doc = "Include or exclude channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
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
            false => CH3_A::EXCLUDED,
            true => CH3_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH3_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH3_A::INCLUDED
    }
}
#[doc = "Field `CH3` writer - Include or exclude channel 3"]
pub type CH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH3_A, O>;
impl<'a, const O: u8> CH3_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH3_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH3_A::INCLUDED)
    }
}
#[doc = "Field `CH4` reader - Include or exclude channel 4"]
pub type CH4_R = crate::BitReader<CH4_A>;
#[doc = "Include or exclude channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
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
            false => CH4_A::EXCLUDED,
            true => CH4_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH4_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH4_A::INCLUDED
    }
}
#[doc = "Field `CH4` writer - Include or exclude channel 4"]
pub type CH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH4_A, O>;
impl<'a, const O: u8> CH4_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH4_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH4_A::INCLUDED)
    }
}
#[doc = "Field `CH5` reader - Include or exclude channel 5"]
pub type CH5_R = crate::BitReader<CH5_A>;
#[doc = "Include or exclude channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
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
            false => CH5_A::EXCLUDED,
            true => CH5_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH5_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH5_A::INCLUDED
    }
}
#[doc = "Field `CH5` writer - Include or exclude channel 5"]
pub type CH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH5_A, O>;
impl<'a, const O: u8> CH5_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH5_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH5_A::INCLUDED)
    }
}
#[doc = "Field `CH6` reader - Include or exclude channel 6"]
pub type CH6_R = crate::BitReader<CH6_A>;
#[doc = "Include or exclude channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
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
            false => CH6_A::EXCLUDED,
            true => CH6_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH6_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH6_A::INCLUDED
    }
}
#[doc = "Field `CH6` writer - Include or exclude channel 6"]
pub type CH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH6_A, O>;
impl<'a, const O: u8> CH6_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH6_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH6_A::INCLUDED)
    }
}
#[doc = "Field `CH7` reader - Include or exclude channel 7"]
pub type CH7_R = crate::BitReader<CH7_A>;
#[doc = "Include or exclude channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
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
            false => CH7_A::EXCLUDED,
            true => CH7_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH7_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH7_A::INCLUDED
    }
}
#[doc = "Field `CH7` writer - Include or exclude channel 7"]
pub type CH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH7_A, O>;
impl<'a, const O: u8> CH7_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH7_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH7_A::INCLUDED)
    }
}
#[doc = "Field `CH8` reader - Include or exclude channel 8"]
pub type CH8_R = crate::BitReader<CH8_A>;
#[doc = "Include or exclude channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
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
            false => CH8_A::EXCLUDED,
            true => CH8_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH8_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH8_A::INCLUDED
    }
}
#[doc = "Field `CH8` writer - Include or exclude channel 8"]
pub type CH8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH8_A, O>;
impl<'a, const O: u8> CH8_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH8_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH8_A::INCLUDED)
    }
}
#[doc = "Field `CH9` reader - Include or exclude channel 9"]
pub type CH9_R = crate::BitReader<CH9_A>;
#[doc = "Include or exclude channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH9_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
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
            false => CH9_A::EXCLUDED,
            true => CH9_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH9_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH9_A::INCLUDED
    }
}
#[doc = "Field `CH9` writer - Include or exclude channel 9"]
pub type CH9_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH9_A, O>;
impl<'a, const O: u8> CH9_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH9_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH9_A::INCLUDED)
    }
}
#[doc = "Field `CH10` reader - Include or exclude channel 10"]
pub type CH10_R = crate::BitReader<CH10_A>;
#[doc = "Include or exclude channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH10_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
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
            false => CH10_A::EXCLUDED,
            true => CH10_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH10_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH10_A::INCLUDED
    }
}
#[doc = "Field `CH10` writer - Include or exclude channel 10"]
pub type CH10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH10_A, O>;
impl<'a, const O: u8> CH10_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH10_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH10_A::INCLUDED)
    }
}
#[doc = "Field `CH11` reader - Include or exclude channel 11"]
pub type CH11_R = crate::BitReader<CH11_A>;
#[doc = "Include or exclude channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH11_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
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
            false => CH11_A::EXCLUDED,
            true => CH11_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH11_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH11_A::INCLUDED
    }
}
#[doc = "Field `CH11` writer - Include or exclude channel 11"]
pub type CH11_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH11_A, O>;
impl<'a, const O: u8> CH11_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH11_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH11_A::INCLUDED)
    }
}
#[doc = "Field `CH12` reader - Include or exclude channel 12"]
pub type CH12_R = crate::BitReader<CH12_A>;
#[doc = "Include or exclude channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH12_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
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
            false => CH12_A::EXCLUDED,
            true => CH12_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH12_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH12_A::INCLUDED
    }
}
#[doc = "Field `CH12` writer - Include or exclude channel 12"]
pub type CH12_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH12_A, O>;
impl<'a, const O: u8> CH12_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH12_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH12_A::INCLUDED)
    }
}
#[doc = "Field `CH13` reader - Include or exclude channel 13"]
pub type CH13_R = crate::BitReader<CH13_A>;
#[doc = "Include or exclude channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH13_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
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
            false => CH13_A::EXCLUDED,
            true => CH13_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH13_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH13_A::INCLUDED
    }
}
#[doc = "Field `CH13` writer - Include or exclude channel 13"]
pub type CH13_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH13_A, O>;
impl<'a, const O: u8> CH13_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH13_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH13_A::INCLUDED)
    }
}
#[doc = "Field `CH14` reader - Include or exclude channel 14"]
pub type CH14_R = crate::BitReader<CH14_A>;
#[doc = "Include or exclude channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH14_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
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
            false => CH14_A::EXCLUDED,
            true => CH14_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH14_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH14_A::INCLUDED
    }
}
#[doc = "Field `CH14` writer - Include or exclude channel 14"]
pub type CH14_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH14_A, O>;
impl<'a, const O: u8> CH14_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH14_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH14_A::INCLUDED)
    }
}
#[doc = "Field `CH15` reader - Include or exclude channel 15"]
pub type CH15_R = crate::BitReader<CH15_A>;
#[doc = "Include or exclude channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH15_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
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
            false => CH15_A::EXCLUDED,
            true => CH15_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH15_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH15_A::INCLUDED
    }
}
#[doc = "Field `CH15` writer - Include or exclude channel 15"]
pub type CH15_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH15_A, O>;
impl<'a, const O: u8> CH15_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH15_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH15_A::INCLUDED)
    }
}
impl R {
    #[doc = "Bit 0 - Include or exclude channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Include or exclude channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Include or exclude channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Include or exclude channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Include or exclude channel 4"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Include or exclude channel 5"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Include or exclude channel 6"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Include or exclude channel 7"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Include or exclude channel 8"]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Include or exclude channel 9"]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Include or exclude channel 10"]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Include or exclude channel 11"]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Include or exclude channel 12"]
    #[inline(always)]
    pub fn ch12(&self) -> CH12_R {
        CH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Include or exclude channel 13"]
    #[inline(always)]
    pub fn ch13(&self) -> CH13_R {
        CH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Include or exclude channel 14"]
    #[inline(always)]
    pub fn ch14(&self) -> CH14_R {
        CH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Include or exclude channel 15"]
    #[inline(always)]
    pub fn ch15(&self) -> CH15_R {
        CH15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Include or exclude channel 0"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Include or exclude channel 1"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W<1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - Include or exclude channel 2"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W<2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - Include or exclude channel 3"]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W<3> {
        CH3_W::new(self)
    }
    #[doc = "Bit 4 - Include or exclude channel 4"]
    #[inline(always)]
    pub fn ch4(&mut self) -> CH4_W<4> {
        CH4_W::new(self)
    }
    #[doc = "Bit 5 - Include or exclude channel 5"]
    #[inline(always)]
    pub fn ch5(&mut self) -> CH5_W<5> {
        CH5_W::new(self)
    }
    #[doc = "Bit 6 - Include or exclude channel 6"]
    #[inline(always)]
    pub fn ch6(&mut self) -> CH6_W<6> {
        CH6_W::new(self)
    }
    #[doc = "Bit 7 - Include or exclude channel 7"]
    #[inline(always)]
    pub fn ch7(&mut self) -> CH7_W<7> {
        CH7_W::new(self)
    }
    #[doc = "Bit 8 - Include or exclude channel 8"]
    #[inline(always)]
    pub fn ch8(&mut self) -> CH8_W<8> {
        CH8_W::new(self)
    }
    #[doc = "Bit 9 - Include or exclude channel 9"]
    #[inline(always)]
    pub fn ch9(&mut self) -> CH9_W<9> {
        CH9_W::new(self)
    }
    #[doc = "Bit 10 - Include or exclude channel 10"]
    #[inline(always)]
    pub fn ch10(&mut self) -> CH10_W<10> {
        CH10_W::new(self)
    }
    #[doc = "Bit 11 - Include or exclude channel 11"]
    #[inline(always)]
    pub fn ch11(&mut self) -> CH11_W<11> {
        CH11_W::new(self)
    }
    #[doc = "Bit 12 - Include or exclude channel 12"]
    #[inline(always)]
    pub fn ch12(&mut self) -> CH12_W<12> {
        CH12_W::new(self)
    }
    #[doc = "Bit 13 - Include or exclude channel 13"]
    #[inline(always)]
    pub fn ch13(&mut self) -> CH13_W<13> {
        CH13_W::new(self)
    }
    #[doc = "Bit 14 - Include or exclude channel 14"]
    #[inline(always)]
    pub fn ch14(&mut self) -> CH14_W<14> {
        CH14_W::new(self)
    }
    #[doc = "Bit 15 - Include or exclude channel 15"]
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
#[doc = "Description collection: Channel group n Note: Writes to this register is ignored if either SUBSCRIBE_CHG\\[n\\].EN/DIS are enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chg](index.html) module"]
pub struct CHG_SPEC;
impl crate::RegisterSpec for CHG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chg::R](R) reader structure"]
impl crate::Readable for CHG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chg::W](W) writer structure"]
impl crate::Writable for CHG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHG[%s]
to value 0"]
impl crate::Resettable for CHG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
