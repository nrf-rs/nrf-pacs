#[doc = "Register `EPDATASTATUS` reader"]
pub struct R(crate::R<EPDATASTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPDATASTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPDATASTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPDATASTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPDATASTATUS` writer"]
pub struct W(crate::W<EPDATASTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPDATASTATUS_SPEC>;
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
impl From<crate::W<EPDATASTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPDATASTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPIN1` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type EPIN1_R = crate::BitReader<EPIN1_A>;
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN1_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOT_DONE = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
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
            false => EPIN1_A::NOT_DONE,
            true => EPIN1_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DONE`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == EPIN1_A::NOT_DONE
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN1_A::DATA_DONE
    }
}
#[doc = "Field `EPIN1` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type EPIN1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPDATASTATUS_SPEC, EPIN1_A, O>;
impl<'a, const O: u8> EPIN1_W<'a, O> {
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN1_A::NOT_DONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN1_A::DATA_DONE)
    }
}
#[doc = "Field `EPIN2` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type EPIN2_R = crate::BitReader<EPIN2_A>;
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN2_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOT_DONE = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
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
            false => EPIN2_A::NOT_DONE,
            true => EPIN2_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DONE`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == EPIN2_A::NOT_DONE
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN2_A::DATA_DONE
    }
}
#[doc = "Field `EPIN2` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type EPIN2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPDATASTATUS_SPEC, EPIN2_A, O>;
impl<'a, const O: u8> EPIN2_W<'a, O> {
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN2_A::NOT_DONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN2_A::DATA_DONE)
    }
}
#[doc = "Field `EPIN3` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type EPIN3_R = crate::BitReader<EPIN3_A>;
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN3_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOT_DONE = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
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
            false => EPIN3_A::NOT_DONE,
            true => EPIN3_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DONE`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == EPIN3_A::NOT_DONE
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN3_A::DATA_DONE
    }
}
#[doc = "Field `EPIN3` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type EPIN3_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPDATASTATUS_SPEC, EPIN3_A, O>;
impl<'a, const O: u8> EPIN3_W<'a, O> {
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN3_A::NOT_DONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN3_A::DATA_DONE)
    }
}
#[doc = "Field `EPIN4` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type EPIN4_R = crate::BitReader<EPIN4_A>;
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN4_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOT_DONE = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
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
            false => EPIN4_A::NOT_DONE,
            true => EPIN4_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DONE`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == EPIN4_A::NOT_DONE
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN4_A::DATA_DONE
    }
}
#[doc = "Field `EPIN4` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type EPIN4_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPDATASTATUS_SPEC, EPIN4_A, O>;
impl<'a, const O: u8> EPIN4_W<'a, O> {
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN4_A::NOT_DONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN4_A::DATA_DONE)
    }
}
#[doc = "Field `EPIN5` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type EPIN5_R = crate::BitReader<EPIN5_A>;
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN5_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOT_DONE = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
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
            false => EPIN5_A::NOT_DONE,
            true => EPIN5_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DONE`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == EPIN5_A::NOT_DONE
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN5_A::DATA_DONE
    }
}
#[doc = "Field `EPIN5` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type EPIN5_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPDATASTATUS_SPEC, EPIN5_A, O>;
impl<'a, const O: u8> EPIN5_W<'a, O> {
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN5_A::NOT_DONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN5_A::DATA_DONE)
    }
}
#[doc = "Field `EPIN6` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type EPIN6_R = crate::BitReader<EPIN6_A>;
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN6_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOT_DONE = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
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
            false => EPIN6_A::NOT_DONE,
            true => EPIN6_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DONE`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == EPIN6_A::NOT_DONE
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN6_A::DATA_DONE
    }
}
#[doc = "Field `EPIN6` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type EPIN6_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPDATASTATUS_SPEC, EPIN6_A, O>;
impl<'a, const O: u8> EPIN6_W<'a, O> {
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN6_A::NOT_DONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN6_A::DATA_DONE)
    }
}
#[doc = "Field `EPIN7` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type EPIN7_R = crate::BitReader<EPIN7_A>;
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN7_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOT_DONE = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
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
            false => EPIN7_A::NOT_DONE,
            true => EPIN7_A::DATA_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DONE`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        *self == EPIN7_A::NOT_DONE
    }
    #[doc = "Checks if the value of the field is `DATA_DONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN7_A::DATA_DONE
    }
}
#[doc = "Field `EPIN7` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub type EPIN7_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPDATASTATUS_SPEC, EPIN7_A, O>;
impl<'a, const O: u8> EPIN7_W<'a, O> {
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN7_A::NOT_DONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN7_A::DATA_DONE)
    }
}
#[doc = "Field `EPOUT1` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type EPOUT1_R = crate::BitReader<EPOUT1_A>;
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT1_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOT_STARTED = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    STARTED = 1,
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
            false => EPOUT1_A::NOT_STARTED,
            true => EPOUT1_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_STARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == EPOUT1_A::NOT_STARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == EPOUT1_A::STARTED
    }
}
#[doc = "Field `EPOUT1` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type EPOUT1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPDATASTATUS_SPEC, EPOUT1_A, O>;
impl<'a, const O: u8> EPOUT1_W<'a, O> {
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT1_A::NOT_STARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT1_A::STARTED)
    }
}
#[doc = "Field `EPOUT2` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type EPOUT2_R = crate::BitReader<EPOUT2_A>;
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT2_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOT_STARTED = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    STARTED = 1,
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
            false => EPOUT2_A::NOT_STARTED,
            true => EPOUT2_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_STARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == EPOUT2_A::NOT_STARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == EPOUT2_A::STARTED
    }
}
#[doc = "Field `EPOUT2` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type EPOUT2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPDATASTATUS_SPEC, EPOUT2_A, O>;
impl<'a, const O: u8> EPOUT2_W<'a, O> {
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT2_A::NOT_STARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT2_A::STARTED)
    }
}
#[doc = "Field `EPOUT3` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type EPOUT3_R = crate::BitReader<EPOUT3_A>;
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT3_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOT_STARTED = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    STARTED = 1,
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
            false => EPOUT3_A::NOT_STARTED,
            true => EPOUT3_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_STARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == EPOUT3_A::NOT_STARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == EPOUT3_A::STARTED
    }
}
#[doc = "Field `EPOUT3` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type EPOUT3_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPDATASTATUS_SPEC, EPOUT3_A, O>;
impl<'a, const O: u8> EPOUT3_W<'a, O> {
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT3_A::NOT_STARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT3_A::STARTED)
    }
}
#[doc = "Field `EPOUT4` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type EPOUT4_R = crate::BitReader<EPOUT4_A>;
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT4_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOT_STARTED = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    STARTED = 1,
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
            false => EPOUT4_A::NOT_STARTED,
            true => EPOUT4_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_STARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == EPOUT4_A::NOT_STARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == EPOUT4_A::STARTED
    }
}
#[doc = "Field `EPOUT4` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type EPOUT4_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPDATASTATUS_SPEC, EPOUT4_A, O>;
impl<'a, const O: u8> EPOUT4_W<'a, O> {
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT4_A::NOT_STARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT4_A::STARTED)
    }
}
#[doc = "Field `EPOUT5` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type EPOUT5_R = crate::BitReader<EPOUT5_A>;
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT5_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOT_STARTED = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    STARTED = 1,
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
            false => EPOUT5_A::NOT_STARTED,
            true => EPOUT5_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_STARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == EPOUT5_A::NOT_STARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == EPOUT5_A::STARTED
    }
}
#[doc = "Field `EPOUT5` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type EPOUT5_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPDATASTATUS_SPEC, EPOUT5_A, O>;
impl<'a, const O: u8> EPOUT5_W<'a, O> {
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT5_A::NOT_STARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT5_A::STARTED)
    }
}
#[doc = "Field `EPOUT6` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type EPOUT6_R = crate::BitReader<EPOUT6_A>;
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT6_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOT_STARTED = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    STARTED = 1,
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
            false => EPOUT6_A::NOT_STARTED,
            true => EPOUT6_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_STARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == EPOUT6_A::NOT_STARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == EPOUT6_A::STARTED
    }
}
#[doc = "Field `EPOUT6` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type EPOUT6_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPDATASTATUS_SPEC, EPOUT6_A, O>;
impl<'a, const O: u8> EPOUT6_W<'a, O> {
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT6_A::NOT_STARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT6_A::STARTED)
    }
}
#[doc = "Field `EPOUT7` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type EPOUT7_R = crate::BitReader<EPOUT7_A>;
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT7_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOT_STARTED = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    STARTED = 1,
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
            false => EPOUT7_A::NOT_STARTED,
            true => EPOUT7_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_STARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == EPOUT7_A::NOT_STARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == EPOUT7_A::STARTED
    }
}
#[doc = "Field `EPOUT7` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub type EPOUT7_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, EPDATASTATUS_SPEC, EPOUT7_A, O>;
impl<'a, const O: u8> EPOUT7_W<'a, O> {
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT7_A::NOT_STARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT7_A::STARTED)
    }
}
impl R {
    #[doc = "Bit 1 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin1(&self) -> EPIN1_R {
        EPIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin2(&self) -> EPIN2_R {
        EPIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin3(&self) -> EPIN3_R {
        EPIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin4(&self) -> EPIN4_R {
        EPIN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin5(&self) -> EPIN5_R {
        EPIN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin6(&self) -> EPIN6_R {
        EPIN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin7(&self) -> EPIN7_R {
        EPIN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 17 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout1(&self) -> EPOUT1_R {
        EPOUT1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout2(&self) -> EPOUT2_R {
        EPOUT2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout3(&self) -> EPOUT3_R {
        EPOUT3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout4(&self) -> EPOUT4_R {
        EPOUT4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout5(&self) -> EPOUT5_R {
        EPOUT5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout6(&self) -> EPOUT6_R {
        EPOUT6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout7(&self) -> EPOUT7_R {
        EPOUT7_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin1(&mut self) -> EPIN1_W<1> {
        EPIN1_W::new(self)
    }
    #[doc = "Bit 2 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin2(&mut self) -> EPIN2_W<2> {
        EPIN2_W::new(self)
    }
    #[doc = "Bit 3 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin3(&mut self) -> EPIN3_W<3> {
        EPIN3_W::new(self)
    }
    #[doc = "Bit 4 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin4(&mut self) -> EPIN4_W<4> {
        EPIN4_W::new(self)
    }
    #[doc = "Bit 5 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin5(&mut self) -> EPIN5_W<5> {
        EPIN5_W::new(self)
    }
    #[doc = "Bit 6 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin6(&mut self) -> EPIN6_W<6> {
        EPIN6_W::new(self)
    }
    #[doc = "Bit 7 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin7(&mut self) -> EPIN7_W<7> {
        EPIN7_W::new(self)
    }
    #[doc = "Bit 17 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout1(&mut self) -> EPOUT1_W<17> {
        EPOUT1_W::new(self)
    }
    #[doc = "Bit 18 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout2(&mut self) -> EPOUT2_W<18> {
        EPOUT2_W::new(self)
    }
    #[doc = "Bit 19 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout3(&mut self) -> EPOUT3_W<19> {
        EPOUT3_W::new(self)
    }
    #[doc = "Bit 20 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout4(&mut self) -> EPOUT4_W<20> {
        EPOUT4_W::new(self)
    }
    #[doc = "Bit 21 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout5(&mut self) -> EPOUT5_W<21> {
        EPOUT5_W::new(self)
    }
    #[doc = "Bit 22 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout6(&mut self) -> EPOUT6_W<22> {
        EPOUT6_W::new(self)
    }
    #[doc = "Bit 23 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout7(&mut self) -> EPOUT7_W<23> {
        EPOUT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Provides information on which endpoint(s) an acknowledged data transfer has occurred (EPDATA event)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epdatastatus](index.html) module"]
pub struct EPDATASTATUS_SPEC;
impl crate::RegisterSpec for EPDATASTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epdatastatus::R](R) reader structure"]
impl crate::Readable for EPDATASTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epdatastatus::W](W) writer structure"]
impl crate::Writable for EPDATASTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPDATASTATUS to value 0"]
impl crate::Resettable for EPDATASTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
