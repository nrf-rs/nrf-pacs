#[doc = "Register `EPSTATUS` reader"]
pub struct R(crate::R<EPSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPSTATUS` writer"]
pub struct W(crate::W<EPSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPSTATUS_SPEC>;
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
impl From<crate::W<EPSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPIN0` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPIN0_R = crate::BitReader<EPIN0_A>;
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN0_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NO_DATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATA_DONE = 1,
}
impl From<EPIN0_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN0_A) -> Self {
        variant as u8 != 0
    }
}
impl EPIN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN0_A {
        match self.bits {
            false => EPIN0_A::NO_DATA,
            true => EPIN0_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN0_A::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN0_A::DATA_DONE
    }
}
#[doc = "Field `EPIN0` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPIN0_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPSTATUS_SPEC, EPIN0_A, O>;
impl<'a, const O: u8> EPIN0_W<'a, O> {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN0_A::NO_DATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN0_A::DATA_DONE)
    }
}
#[doc = "Field `EPIN1` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPIN1_R = crate::BitReader<EPIN1_A>;
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN1_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NO_DATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATA_DONE = 1,
}
impl From<EPIN1_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN1_A) -> Self {
        variant as u8 != 0
    }
}
impl EPIN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN1_A {
        match self.bits {
            false => EPIN1_A::NO_DATA,
            true => EPIN1_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN1_A::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN1_A::DATA_DONE
    }
}
#[doc = "Field `EPIN1` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPIN1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPSTATUS_SPEC, EPIN1_A, O>;
impl<'a, const O: u8> EPIN1_W<'a, O> {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN1_A::NO_DATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN1_A::DATA_DONE)
    }
}
#[doc = "Field `EPIN2` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPIN2_R = crate::BitReader<EPIN2_A>;
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN2_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NO_DATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATA_DONE = 1,
}
impl From<EPIN2_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN2_A) -> Self {
        variant as u8 != 0
    }
}
impl EPIN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN2_A {
        match self.bits {
            false => EPIN2_A::NO_DATA,
            true => EPIN2_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN2_A::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN2_A::DATA_DONE
    }
}
#[doc = "Field `EPIN2` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPIN2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPSTATUS_SPEC, EPIN2_A, O>;
impl<'a, const O: u8> EPIN2_W<'a, O> {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN2_A::NO_DATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN2_A::DATA_DONE)
    }
}
#[doc = "Field `EPIN3` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPIN3_R = crate::BitReader<EPIN3_A>;
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN3_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NO_DATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATA_DONE = 1,
}
impl From<EPIN3_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN3_A) -> Self {
        variant as u8 != 0
    }
}
impl EPIN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN3_A {
        match self.bits {
            false => EPIN3_A::NO_DATA,
            true => EPIN3_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN3_A::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN3_A::DATA_DONE
    }
}
#[doc = "Field `EPIN3` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPIN3_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPSTATUS_SPEC, EPIN3_A, O>;
impl<'a, const O: u8> EPIN3_W<'a, O> {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN3_A::NO_DATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN3_A::DATA_DONE)
    }
}
#[doc = "Field `EPIN4` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPIN4_R = crate::BitReader<EPIN4_A>;
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN4_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NO_DATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATA_DONE = 1,
}
impl From<EPIN4_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN4_A) -> Self {
        variant as u8 != 0
    }
}
impl EPIN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN4_A {
        match self.bits {
            false => EPIN4_A::NO_DATA,
            true => EPIN4_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN4_A::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN4_A::DATA_DONE
    }
}
#[doc = "Field `EPIN4` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPIN4_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPSTATUS_SPEC, EPIN4_A, O>;
impl<'a, const O: u8> EPIN4_W<'a, O> {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN4_A::NO_DATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN4_A::DATA_DONE)
    }
}
#[doc = "Field `EPIN5` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPIN5_R = crate::BitReader<EPIN5_A>;
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN5_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NO_DATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATA_DONE = 1,
}
impl From<EPIN5_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN5_A) -> Self {
        variant as u8 != 0
    }
}
impl EPIN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN5_A {
        match self.bits {
            false => EPIN5_A::NO_DATA,
            true => EPIN5_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN5_A::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN5_A::DATA_DONE
    }
}
#[doc = "Field `EPIN5` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPIN5_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPSTATUS_SPEC, EPIN5_A, O>;
impl<'a, const O: u8> EPIN5_W<'a, O> {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN5_A::NO_DATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN5_A::DATA_DONE)
    }
}
#[doc = "Field `EPIN6` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPIN6_R = crate::BitReader<EPIN6_A>;
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN6_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NO_DATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATA_DONE = 1,
}
impl From<EPIN6_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN6_A) -> Self {
        variant as u8 != 0
    }
}
impl EPIN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN6_A {
        match self.bits {
            false => EPIN6_A::NO_DATA,
            true => EPIN6_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN6_A::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN6_A::DATA_DONE
    }
}
#[doc = "Field `EPIN6` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPIN6_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPSTATUS_SPEC, EPIN6_A, O>;
impl<'a, const O: u8> EPIN6_W<'a, O> {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN6_A::NO_DATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN6_A::DATA_DONE)
    }
}
#[doc = "Field `EPIN7` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPIN7_R = crate::BitReader<EPIN7_A>;
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN7_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NO_DATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATA_DONE = 1,
}
impl From<EPIN7_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN7_A) -> Self {
        variant as u8 != 0
    }
}
impl EPIN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN7_A {
        match self.bits {
            false => EPIN7_A::NO_DATA,
            true => EPIN7_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN7_A::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN7_A::DATA_DONE
    }
}
#[doc = "Field `EPIN7` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPIN7_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPSTATUS_SPEC, EPIN7_A, O>;
impl<'a, const O: u8> EPIN7_W<'a, O> {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN7_A::NO_DATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN7_A::DATA_DONE)
    }
}
#[doc = "Field `EPIN8` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPIN8_R = crate::BitReader<EPIN8_A>;
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN8_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NO_DATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATA_DONE = 1,
}
impl From<EPIN8_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN8_A) -> Self {
        variant as u8 != 0
    }
}
impl EPIN8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN8_A {
        match self.bits {
            false => EPIN8_A::NO_DATA,
            true => EPIN8_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN8_A::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN8_A::DATA_DONE
    }
}
#[doc = "Field `EPIN8` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPIN8_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPSTATUS_SPEC, EPIN8_A, O>;
impl<'a, const O: u8> EPIN8_W<'a, O> {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN8_A::NO_DATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN8_A::DATA_DONE)
    }
}
#[doc = "Field `EPOUT0` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPOUT0_R = crate::BitReader<EPOUT0_A>;
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT0_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NO_DATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATA_DONE = 1,
}
impl From<EPOUT0_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT0_A) -> Self {
        variant as u8 != 0
    }
}
impl EPOUT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT0_A {
        match self.bits {
            false => EPOUT0_A::NO_DATA,
            true => EPOUT0_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT0_A::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT0_A::DATA_DONE
    }
}
#[doc = "Field `EPOUT0` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPOUT0_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPSTATUS_SPEC, EPOUT0_A, O>;
impl<'a, const O: u8> EPOUT0_W<'a, O> {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT0_A::NO_DATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT0_A::DATA_DONE)
    }
}
#[doc = "Field `EPOUT1` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPOUT1_R = crate::BitReader<EPOUT1_A>;
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT1_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NO_DATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATA_DONE = 1,
}
impl From<EPOUT1_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT1_A) -> Self {
        variant as u8 != 0
    }
}
impl EPOUT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT1_A {
        match self.bits {
            false => EPOUT1_A::NO_DATA,
            true => EPOUT1_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT1_A::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT1_A::DATA_DONE
    }
}
#[doc = "Field `EPOUT1` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPOUT1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPSTATUS_SPEC, EPOUT1_A, O>;
impl<'a, const O: u8> EPOUT1_W<'a, O> {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT1_A::NO_DATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT1_A::DATA_DONE)
    }
}
#[doc = "Field `EPOUT2` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPOUT2_R = crate::BitReader<EPOUT2_A>;
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT2_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NO_DATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATA_DONE = 1,
}
impl From<EPOUT2_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT2_A) -> Self {
        variant as u8 != 0
    }
}
impl EPOUT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT2_A {
        match self.bits {
            false => EPOUT2_A::NO_DATA,
            true => EPOUT2_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT2_A::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT2_A::DATA_DONE
    }
}
#[doc = "Field `EPOUT2` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPOUT2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPSTATUS_SPEC, EPOUT2_A, O>;
impl<'a, const O: u8> EPOUT2_W<'a, O> {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT2_A::NO_DATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT2_A::DATA_DONE)
    }
}
#[doc = "Field `EPOUT3` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPOUT3_R = crate::BitReader<EPOUT3_A>;
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT3_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NO_DATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATA_DONE = 1,
}
impl From<EPOUT3_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT3_A) -> Self {
        variant as u8 != 0
    }
}
impl EPOUT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT3_A {
        match self.bits {
            false => EPOUT3_A::NO_DATA,
            true => EPOUT3_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT3_A::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT3_A::DATA_DONE
    }
}
#[doc = "Field `EPOUT3` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPOUT3_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPSTATUS_SPEC, EPOUT3_A, O>;
impl<'a, const O: u8> EPOUT3_W<'a, O> {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT3_A::NO_DATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT3_A::DATA_DONE)
    }
}
#[doc = "Field `EPOUT4` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPOUT4_R = crate::BitReader<EPOUT4_A>;
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT4_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NO_DATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATA_DONE = 1,
}
impl From<EPOUT4_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT4_A) -> Self {
        variant as u8 != 0
    }
}
impl EPOUT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT4_A {
        match self.bits {
            false => EPOUT4_A::NO_DATA,
            true => EPOUT4_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT4_A::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT4_A::DATA_DONE
    }
}
#[doc = "Field `EPOUT4` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPOUT4_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPSTATUS_SPEC, EPOUT4_A, O>;
impl<'a, const O: u8> EPOUT4_W<'a, O> {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT4_A::NO_DATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT4_A::DATA_DONE)
    }
}
#[doc = "Field `EPOUT5` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPOUT5_R = crate::BitReader<EPOUT5_A>;
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT5_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NO_DATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATA_DONE = 1,
}
impl From<EPOUT5_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT5_A) -> Self {
        variant as u8 != 0
    }
}
impl EPOUT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT5_A {
        match self.bits {
            false => EPOUT5_A::NO_DATA,
            true => EPOUT5_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT5_A::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT5_A::DATA_DONE
    }
}
#[doc = "Field `EPOUT5` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPOUT5_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPSTATUS_SPEC, EPOUT5_A, O>;
impl<'a, const O: u8> EPOUT5_W<'a, O> {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT5_A::NO_DATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT5_A::DATA_DONE)
    }
}
#[doc = "Field `EPOUT6` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPOUT6_R = crate::BitReader<EPOUT6_A>;
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT6_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NO_DATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATA_DONE = 1,
}
impl From<EPOUT6_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT6_A) -> Self {
        variant as u8 != 0
    }
}
impl EPOUT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT6_A {
        match self.bits {
            false => EPOUT6_A::NO_DATA,
            true => EPOUT6_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT6_A::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT6_A::DATA_DONE
    }
}
#[doc = "Field `EPOUT6` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPOUT6_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPSTATUS_SPEC, EPOUT6_A, O>;
impl<'a, const O: u8> EPOUT6_W<'a, O> {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT6_A::NO_DATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT6_A::DATA_DONE)
    }
}
#[doc = "Field `EPOUT7` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPOUT7_R = crate::BitReader<EPOUT7_A>;
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT7_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NO_DATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATA_DONE = 1,
}
impl From<EPOUT7_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT7_A) -> Self {
        variant as u8 != 0
    }
}
impl EPOUT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT7_A {
        match self.bits {
            false => EPOUT7_A::NO_DATA,
            true => EPOUT7_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT7_A::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT7_A::DATA_DONE
    }
}
#[doc = "Field `EPOUT7` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPOUT7_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPSTATUS_SPEC, EPOUT7_A, O>;
impl<'a, const O: u8> EPOUT7_W<'a, O> {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT7_A::NO_DATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT7_A::DATA_DONE)
    }
}
#[doc = "Field `EPOUT8` reader - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPOUT8_R = crate::BitReader<EPOUT8_A>;
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT8_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NO_DATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATA_DONE = 1,
}
impl From<EPOUT8_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT8_A) -> Self {
        variant as u8 != 0
    }
}
impl EPOUT8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT8_A {
        match self.bits {
            false => EPOUT8_A::NO_DATA,
            true => EPOUT8_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT8_A::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT8_A::DATA_DONE
    }
}
#[doc = "Field `EPOUT8` writer - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
pub type EPOUT8_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPSTATUS_SPEC, EPOUT8_A, O>;
impl<'a, const O: u8> EPOUT8_W<'a, O> {
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT8_A::NO_DATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT8_A::DATA_DONE)
    }
}
impl R {
    #[doc = "Bit 0 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin0(&self) -> EPIN0_R {
        EPIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin1(&self) -> EPIN1_R {
        EPIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin2(&self) -> EPIN2_R {
        EPIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin3(&self) -> EPIN3_R {
        EPIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin4(&self) -> EPIN4_R {
        EPIN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin5(&self) -> EPIN5_R {
        EPIN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin6(&self) -> EPIN6_R {
        EPIN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin7(&self) -> EPIN7_R {
        EPIN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin8(&self) -> EPIN8_R {
        EPIN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout0(&self) -> EPOUT0_R {
        EPOUT0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout1(&self) -> EPOUT1_R {
        EPOUT1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout2(&self) -> EPOUT2_R {
        EPOUT2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout3(&self) -> EPOUT3_R {
        EPOUT3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout4(&self) -> EPOUT4_R {
        EPOUT4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout5(&self) -> EPOUT5_R {
        EPOUT5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout6(&self) -> EPOUT6_R {
        EPOUT6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout7(&self) -> EPOUT7_R {
        EPOUT7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout8(&self) -> EPOUT8_R {
        EPOUT8_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin0(&mut self) -> EPIN0_W<0> {
        EPIN0_W::new(self)
    }
    #[doc = "Bit 1 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin1(&mut self) -> EPIN1_W<1> {
        EPIN1_W::new(self)
    }
    #[doc = "Bit 2 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin2(&mut self) -> EPIN2_W<2> {
        EPIN2_W::new(self)
    }
    #[doc = "Bit 3 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin3(&mut self) -> EPIN3_W<3> {
        EPIN3_W::new(self)
    }
    #[doc = "Bit 4 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin4(&mut self) -> EPIN4_W<4> {
        EPIN4_W::new(self)
    }
    #[doc = "Bit 5 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin5(&mut self) -> EPIN5_W<5> {
        EPIN5_W::new(self)
    }
    #[doc = "Bit 6 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin6(&mut self) -> EPIN6_W<6> {
        EPIN6_W::new(self)
    }
    #[doc = "Bit 7 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin7(&mut self) -> EPIN7_W<7> {
        EPIN7_W::new(self)
    }
    #[doc = "Bit 8 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin8(&mut self) -> EPIN8_W<8> {
        EPIN8_W::new(self)
    }
    #[doc = "Bit 16 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout0(&mut self) -> EPOUT0_W<16> {
        EPOUT0_W::new(self)
    }
    #[doc = "Bit 17 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout1(&mut self) -> EPOUT1_W<17> {
        EPOUT1_W::new(self)
    }
    #[doc = "Bit 18 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout2(&mut self) -> EPOUT2_W<18> {
        EPOUT2_W::new(self)
    }
    #[doc = "Bit 19 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout3(&mut self) -> EPOUT3_W<19> {
        EPOUT3_W::new(self)
    }
    #[doc = "Bit 20 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout4(&mut self) -> EPOUT4_W<20> {
        EPOUT4_W::new(self)
    }
    #[doc = "Bit 21 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout5(&mut self) -> EPOUT5_W<21> {
        EPOUT5_W::new(self)
    }
    #[doc = "Bit 22 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout6(&mut self) -> EPOUT6_W<22> {
        EPOUT6_W::new(self)
    }
    #[doc = "Bit 23 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout7(&mut self) -> EPOUT7_W<23> {
        EPOUT7_W::new(self)
    }
    #[doc = "Bit 24 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout8(&mut self) -> EPOUT8_W<24> {
        EPOUT8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Provides information on which endpoint's EasyDMA registers have been captured\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epstatus](index.html) module"]
pub struct EPSTATUS_SPEC;
impl crate::RegisterSpec for EPSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epstatus::R](R) reader structure"]
impl crate::Readable for EPSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epstatus::W](W) writer structure"]
impl crate::Writable for EPSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPSTATUS to value 0"]
impl crate::Resettable for EPSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
