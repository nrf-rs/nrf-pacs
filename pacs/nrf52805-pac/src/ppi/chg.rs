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
#[doc = "Field `CH20` reader - Include or exclude channel 20"]
pub type CH20_R = crate::BitReader<CH20_A>;
#[doc = "Include or exclude channel 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH20_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
}
impl From<CH20_A> for bool {
    #[inline(always)]
    fn from(variant: CH20_A) -> Self {
        variant as u8 != 0
    }
}
impl CH20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH20_A {
        match self.bits {
            false => CH20_A::EXCLUDED,
            true => CH20_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH20_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH20_A::INCLUDED
    }
}
#[doc = "Field `CH20` writer - Include or exclude channel 20"]
pub type CH20_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH20_A, O>;
impl<'a, const O: u8> CH20_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH20_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH20_A::INCLUDED)
    }
}
#[doc = "Field `CH21` reader - Include or exclude channel 21"]
pub type CH21_R = crate::BitReader<CH21_A>;
#[doc = "Include or exclude channel 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH21_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
}
impl From<CH21_A> for bool {
    #[inline(always)]
    fn from(variant: CH21_A) -> Self {
        variant as u8 != 0
    }
}
impl CH21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH21_A {
        match self.bits {
            false => CH21_A::EXCLUDED,
            true => CH21_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH21_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH21_A::INCLUDED
    }
}
#[doc = "Field `CH21` writer - Include or exclude channel 21"]
pub type CH21_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH21_A, O>;
impl<'a, const O: u8> CH21_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH21_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH21_A::INCLUDED)
    }
}
#[doc = "Field `CH22` reader - Include or exclude channel 22"]
pub type CH22_R = crate::BitReader<CH22_A>;
#[doc = "Include or exclude channel 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH22_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
}
impl From<CH22_A> for bool {
    #[inline(always)]
    fn from(variant: CH22_A) -> Self {
        variant as u8 != 0
    }
}
impl CH22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH22_A {
        match self.bits {
            false => CH22_A::EXCLUDED,
            true => CH22_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH22_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH22_A::INCLUDED
    }
}
#[doc = "Field `CH22` writer - Include or exclude channel 22"]
pub type CH22_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH22_A, O>;
impl<'a, const O: u8> CH22_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH22_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH22_A::INCLUDED)
    }
}
#[doc = "Field `CH23` reader - Include or exclude channel 23"]
pub type CH23_R = crate::BitReader<CH23_A>;
#[doc = "Include or exclude channel 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH23_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
}
impl From<CH23_A> for bool {
    #[inline(always)]
    fn from(variant: CH23_A) -> Self {
        variant as u8 != 0
    }
}
impl CH23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH23_A {
        match self.bits {
            false => CH23_A::EXCLUDED,
            true => CH23_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH23_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH23_A::INCLUDED
    }
}
#[doc = "Field `CH23` writer - Include or exclude channel 23"]
pub type CH23_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH23_A, O>;
impl<'a, const O: u8> CH23_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH23_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH23_A::INCLUDED)
    }
}
#[doc = "Field `CH24` reader - Include or exclude channel 24"]
pub type CH24_R = crate::BitReader<CH24_A>;
#[doc = "Include or exclude channel 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH24_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
}
impl From<CH24_A> for bool {
    #[inline(always)]
    fn from(variant: CH24_A) -> Self {
        variant as u8 != 0
    }
}
impl CH24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH24_A {
        match self.bits {
            false => CH24_A::EXCLUDED,
            true => CH24_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH24_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH24_A::INCLUDED
    }
}
#[doc = "Field `CH24` writer - Include or exclude channel 24"]
pub type CH24_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH24_A, O>;
impl<'a, const O: u8> CH24_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH24_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH24_A::INCLUDED)
    }
}
#[doc = "Field `CH25` reader - Include or exclude channel 25"]
pub type CH25_R = crate::BitReader<CH25_A>;
#[doc = "Include or exclude channel 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH25_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
}
impl From<CH25_A> for bool {
    #[inline(always)]
    fn from(variant: CH25_A) -> Self {
        variant as u8 != 0
    }
}
impl CH25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH25_A {
        match self.bits {
            false => CH25_A::EXCLUDED,
            true => CH25_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH25_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH25_A::INCLUDED
    }
}
#[doc = "Field `CH25` writer - Include or exclude channel 25"]
pub type CH25_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH25_A, O>;
impl<'a, const O: u8> CH25_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH25_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH25_A::INCLUDED)
    }
}
#[doc = "Field `CH26` reader - Include or exclude channel 26"]
pub type CH26_R = crate::BitReader<CH26_A>;
#[doc = "Include or exclude channel 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH26_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
}
impl From<CH26_A> for bool {
    #[inline(always)]
    fn from(variant: CH26_A) -> Self {
        variant as u8 != 0
    }
}
impl CH26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH26_A {
        match self.bits {
            false => CH26_A::EXCLUDED,
            true => CH26_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH26_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH26_A::INCLUDED
    }
}
#[doc = "Field `CH26` writer - Include or exclude channel 26"]
pub type CH26_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH26_A, O>;
impl<'a, const O: u8> CH26_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH26_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH26_A::INCLUDED)
    }
}
#[doc = "Field `CH27` reader - Include or exclude channel 27"]
pub type CH27_R = crate::BitReader<CH27_A>;
#[doc = "Include or exclude channel 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH27_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
}
impl From<CH27_A> for bool {
    #[inline(always)]
    fn from(variant: CH27_A) -> Self {
        variant as u8 != 0
    }
}
impl CH27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH27_A {
        match self.bits {
            false => CH27_A::EXCLUDED,
            true => CH27_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH27_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH27_A::INCLUDED
    }
}
#[doc = "Field `CH27` writer - Include or exclude channel 27"]
pub type CH27_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH27_A, O>;
impl<'a, const O: u8> CH27_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH27_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH27_A::INCLUDED)
    }
}
#[doc = "Field `CH28` reader - Include or exclude channel 28"]
pub type CH28_R = crate::BitReader<CH28_A>;
#[doc = "Include or exclude channel 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH28_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
}
impl From<CH28_A> for bool {
    #[inline(always)]
    fn from(variant: CH28_A) -> Self {
        variant as u8 != 0
    }
}
impl CH28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH28_A {
        match self.bits {
            false => CH28_A::EXCLUDED,
            true => CH28_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH28_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH28_A::INCLUDED
    }
}
#[doc = "Field `CH28` writer - Include or exclude channel 28"]
pub type CH28_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH28_A, O>;
impl<'a, const O: u8> CH28_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH28_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH28_A::INCLUDED)
    }
}
#[doc = "Field `CH29` reader - Include or exclude channel 29"]
pub type CH29_R = crate::BitReader<CH29_A>;
#[doc = "Include or exclude channel 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH29_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
}
impl From<CH29_A> for bool {
    #[inline(always)]
    fn from(variant: CH29_A) -> Self {
        variant as u8 != 0
    }
}
impl CH29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH29_A {
        match self.bits {
            false => CH29_A::EXCLUDED,
            true => CH29_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH29_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH29_A::INCLUDED
    }
}
#[doc = "Field `CH29` writer - Include or exclude channel 29"]
pub type CH29_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH29_A, O>;
impl<'a, const O: u8> CH29_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH29_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH29_A::INCLUDED)
    }
}
#[doc = "Field `CH30` reader - Include or exclude channel 30"]
pub type CH30_R = crate::BitReader<CH30_A>;
#[doc = "Include or exclude channel 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH30_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
}
impl From<CH30_A> for bool {
    #[inline(always)]
    fn from(variant: CH30_A) -> Self {
        variant as u8 != 0
    }
}
impl CH30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH30_A {
        match self.bits {
            false => CH30_A::EXCLUDED,
            true => CH30_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH30_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH30_A::INCLUDED
    }
}
#[doc = "Field `CH30` writer - Include or exclude channel 30"]
pub type CH30_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH30_A, O>;
impl<'a, const O: u8> CH30_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH30_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH30_A::INCLUDED)
    }
}
#[doc = "Field `CH31` reader - Include or exclude channel 31"]
pub type CH31_R = crate::BitReader<CH31_A>;
#[doc = "Include or exclude channel 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH31_A {
    #[doc = "0: Exclude"]
    EXCLUDED = 0,
    #[doc = "1: Include"]
    INCLUDED = 1,
}
impl From<CH31_A> for bool {
    #[inline(always)]
    fn from(variant: CH31_A) -> Self {
        variant as u8 != 0
    }
}
impl CH31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH31_A {
        match self.bits {
            false => CH31_A::EXCLUDED,
            true => CH31_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == CH31_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == CH31_A::INCLUDED
    }
}
#[doc = "Field `CH31` writer - Include or exclude channel 31"]
pub type CH31_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHG_SPEC, CH31_A, O>;
impl<'a, const O: u8> CH31_W<'a, O> {
    #[doc = "Exclude"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH31_A::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH31_A::INCLUDED)
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
    #[doc = "Bit 20 - Include or exclude channel 20"]
    #[inline(always)]
    pub fn ch20(&self) -> CH20_R {
        CH20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Include or exclude channel 21"]
    #[inline(always)]
    pub fn ch21(&self) -> CH21_R {
        CH21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Include or exclude channel 22"]
    #[inline(always)]
    pub fn ch22(&self) -> CH22_R {
        CH22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Include or exclude channel 23"]
    #[inline(always)]
    pub fn ch23(&self) -> CH23_R {
        CH23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Include or exclude channel 24"]
    #[inline(always)]
    pub fn ch24(&self) -> CH24_R {
        CH24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Include or exclude channel 25"]
    #[inline(always)]
    pub fn ch25(&self) -> CH25_R {
        CH25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Include or exclude channel 26"]
    #[inline(always)]
    pub fn ch26(&self) -> CH26_R {
        CH26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Include or exclude channel 27"]
    #[inline(always)]
    pub fn ch27(&self) -> CH27_R {
        CH27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Include or exclude channel 28"]
    #[inline(always)]
    pub fn ch28(&self) -> CH28_R {
        CH28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Include or exclude channel 29"]
    #[inline(always)]
    pub fn ch29(&self) -> CH29_R {
        CH29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Include or exclude channel 30"]
    #[inline(always)]
    pub fn ch30(&self) -> CH30_R {
        CH30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Include or exclude channel 31"]
    #[inline(always)]
    pub fn ch31(&self) -> CH31_R {
        CH31_R::new(((self.bits >> 31) & 1) != 0)
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
    #[doc = "Bit 20 - Include or exclude channel 20"]
    #[inline(always)]
    pub fn ch20(&mut self) -> CH20_W<20> {
        CH20_W::new(self)
    }
    #[doc = "Bit 21 - Include or exclude channel 21"]
    #[inline(always)]
    pub fn ch21(&mut self) -> CH21_W<21> {
        CH21_W::new(self)
    }
    #[doc = "Bit 22 - Include or exclude channel 22"]
    #[inline(always)]
    pub fn ch22(&mut self) -> CH22_W<22> {
        CH22_W::new(self)
    }
    #[doc = "Bit 23 - Include or exclude channel 23"]
    #[inline(always)]
    pub fn ch23(&mut self) -> CH23_W<23> {
        CH23_W::new(self)
    }
    #[doc = "Bit 24 - Include or exclude channel 24"]
    #[inline(always)]
    pub fn ch24(&mut self) -> CH24_W<24> {
        CH24_W::new(self)
    }
    #[doc = "Bit 25 - Include or exclude channel 25"]
    #[inline(always)]
    pub fn ch25(&mut self) -> CH25_W<25> {
        CH25_W::new(self)
    }
    #[doc = "Bit 26 - Include or exclude channel 26"]
    #[inline(always)]
    pub fn ch26(&mut self) -> CH26_W<26> {
        CH26_W::new(self)
    }
    #[doc = "Bit 27 - Include or exclude channel 27"]
    #[inline(always)]
    pub fn ch27(&mut self) -> CH27_W<27> {
        CH27_W::new(self)
    }
    #[doc = "Bit 28 - Include or exclude channel 28"]
    #[inline(always)]
    pub fn ch28(&mut self) -> CH28_W<28> {
        CH28_W::new(self)
    }
    #[doc = "Bit 29 - Include or exclude channel 29"]
    #[inline(always)]
    pub fn ch29(&mut self) -> CH29_W<29> {
        CH29_W::new(self)
    }
    #[doc = "Bit 30 - Include or exclude channel 30"]
    #[inline(always)]
    pub fn ch30(&mut self) -> CH30_W<30> {
        CH30_W::new(self)
    }
    #[doc = "Bit 31 - Include or exclude channel 31"]
    #[inline(always)]
    pub fn ch31(&mut self) -> CH31_W<31> {
        CH31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Channel group n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chg](index.html) module"]
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
