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
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN1_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOTDONE = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    DATADONE = 1,
}
impl From<EPIN1_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN1` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub struct EPIN1_R(crate::FieldReader<bool, EPIN1_A>);
impl EPIN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPIN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN1_A {
        match self.bits {
            false => EPIN1_A::NOTDONE,
            true => EPIN1_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDONE`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        **self == EPIN1_A::NOTDONE
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        **self == EPIN1_A::DATADONE
    }
}
impl core::ops::Deref for EPIN1_R {
    type Target = crate::FieldReader<bool, EPIN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPIN1` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub struct EPIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPIN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN1_A::NOTDONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN1_A::DATADONE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN2_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOTDONE = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    DATADONE = 1,
}
impl From<EPIN2_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN2` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub struct EPIN2_R(crate::FieldReader<bool, EPIN2_A>);
impl EPIN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPIN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN2_A {
        match self.bits {
            false => EPIN2_A::NOTDONE,
            true => EPIN2_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDONE`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        **self == EPIN2_A::NOTDONE
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        **self == EPIN2_A::DATADONE
    }
}
impl core::ops::Deref for EPIN2_R {
    type Target = crate::FieldReader<bool, EPIN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPIN2` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub struct EPIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPIN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN2_A::NOTDONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN2_A::DATADONE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN3_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOTDONE = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    DATADONE = 1,
}
impl From<EPIN3_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN3` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub struct EPIN3_R(crate::FieldReader<bool, EPIN3_A>);
impl EPIN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPIN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN3_A {
        match self.bits {
            false => EPIN3_A::NOTDONE,
            true => EPIN3_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDONE`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        **self == EPIN3_A::NOTDONE
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        **self == EPIN3_A::DATADONE
    }
}
impl core::ops::Deref for EPIN3_R {
    type Target = crate::FieldReader<bool, EPIN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPIN3` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub struct EPIN3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPIN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN3_A::NOTDONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN3_A::DATADONE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN4_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOTDONE = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    DATADONE = 1,
}
impl From<EPIN4_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN4` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub struct EPIN4_R(crate::FieldReader<bool, EPIN4_A>);
impl EPIN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPIN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN4_A {
        match self.bits {
            false => EPIN4_A::NOTDONE,
            true => EPIN4_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDONE`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        **self == EPIN4_A::NOTDONE
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        **self == EPIN4_A::DATADONE
    }
}
impl core::ops::Deref for EPIN4_R {
    type Target = crate::FieldReader<bool, EPIN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPIN4` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub struct EPIN4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPIN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN4_A::NOTDONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN4_A::DATADONE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN5_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOTDONE = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    DATADONE = 1,
}
impl From<EPIN5_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN5` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub struct EPIN5_R(crate::FieldReader<bool, EPIN5_A>);
impl EPIN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPIN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN5_A {
        match self.bits {
            false => EPIN5_A::NOTDONE,
            true => EPIN5_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDONE`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        **self == EPIN5_A::NOTDONE
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        **self == EPIN5_A::DATADONE
    }
}
impl core::ops::Deref for EPIN5_R {
    type Target = crate::FieldReader<bool, EPIN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPIN5` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub struct EPIN5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPIN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN5_A::NOTDONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN5_A::DATADONE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN6_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOTDONE = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    DATADONE = 1,
}
impl From<EPIN6_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN6` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub struct EPIN6_R(crate::FieldReader<bool, EPIN6_A>);
impl EPIN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPIN6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN6_A {
        match self.bits {
            false => EPIN6_A::NOTDONE,
            true => EPIN6_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDONE`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        **self == EPIN6_A::NOTDONE
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        **self == EPIN6_A::DATADONE
    }
}
impl core::ops::Deref for EPIN6_R {
    type Target = crate::FieldReader<bool, EPIN6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPIN6` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub struct EPIN6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPIN6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN6_A::NOTDONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN6_A::DATADONE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Acknowledged data transfer on this IN endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN7_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOTDONE = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    DATADONE = 1,
}
impl From<EPIN7_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIN7` reader - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub struct EPIN7_R(crate::FieldReader<bool, EPIN7_A>);
impl EPIN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPIN7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN7_A {
        match self.bits {
            false => EPIN7_A::NOTDONE,
            true => EPIN7_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDONE`"]
    #[inline(always)]
    pub fn is_not_done(&self) -> bool {
        **self == EPIN7_A::NOTDONE
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        **self == EPIN7_A::DATADONE
    }
}
impl core::ops::Deref for EPIN7_R {
    type Target = crate::FieldReader<bool, EPIN7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPIN7` writer - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
pub struct EPIN7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPIN7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_done(self) -> &'a mut W {
        self.variant(EPIN7_A::NOTDONE)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN7_A::DATADONE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT1_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOTSTARTED = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    STARTED = 1,
}
impl From<EPOUT1_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT1` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub struct EPOUT1_R(crate::FieldReader<bool, EPOUT1_A>);
impl EPOUT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPOUT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT1_A {
        match self.bits {
            false => EPOUT1_A::NOTSTARTED,
            true => EPOUT1_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        **self == EPOUT1_A::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        **self == EPOUT1_A::STARTED
    }
}
impl core::ops::Deref for EPOUT1_R {
    type Target = crate::FieldReader<bool, EPOUT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPOUT1` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub struct EPOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPOUT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPOUT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT1_A::NOTSTARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT1_A::STARTED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT2_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOTSTARTED = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    STARTED = 1,
}
impl From<EPOUT2_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT2` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub struct EPOUT2_R(crate::FieldReader<bool, EPOUT2_A>);
impl EPOUT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPOUT2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT2_A {
        match self.bits {
            false => EPOUT2_A::NOTSTARTED,
            true => EPOUT2_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        **self == EPOUT2_A::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        **self == EPOUT2_A::STARTED
    }
}
impl core::ops::Deref for EPOUT2_R {
    type Target = crate::FieldReader<bool, EPOUT2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPOUT2` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub struct EPOUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPOUT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPOUT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT2_A::NOTSTARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT2_A::STARTED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT3_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOTSTARTED = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    STARTED = 1,
}
impl From<EPOUT3_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT3` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub struct EPOUT3_R(crate::FieldReader<bool, EPOUT3_A>);
impl EPOUT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPOUT3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT3_A {
        match self.bits {
            false => EPOUT3_A::NOTSTARTED,
            true => EPOUT3_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        **self == EPOUT3_A::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        **self == EPOUT3_A::STARTED
    }
}
impl core::ops::Deref for EPOUT3_R {
    type Target = crate::FieldReader<bool, EPOUT3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPOUT3` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub struct EPOUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPOUT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPOUT3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT3_A::NOTSTARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT3_A::STARTED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT4_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOTSTARTED = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    STARTED = 1,
}
impl From<EPOUT4_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT4` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub struct EPOUT4_R(crate::FieldReader<bool, EPOUT4_A>);
impl EPOUT4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPOUT4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT4_A {
        match self.bits {
            false => EPOUT4_A::NOTSTARTED,
            true => EPOUT4_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        **self == EPOUT4_A::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        **self == EPOUT4_A::STARTED
    }
}
impl core::ops::Deref for EPOUT4_R {
    type Target = crate::FieldReader<bool, EPOUT4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPOUT4` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub struct EPOUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPOUT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPOUT4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT4_A::NOTSTARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT4_A::STARTED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT5_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOTSTARTED = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    STARTED = 1,
}
impl From<EPOUT5_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT5` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub struct EPOUT5_R(crate::FieldReader<bool, EPOUT5_A>);
impl EPOUT5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPOUT5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT5_A {
        match self.bits {
            false => EPOUT5_A::NOTSTARTED,
            true => EPOUT5_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        **self == EPOUT5_A::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        **self == EPOUT5_A::STARTED
    }
}
impl core::ops::Deref for EPOUT5_R {
    type Target = crate::FieldReader<bool, EPOUT5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPOUT5` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub struct EPOUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPOUT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPOUT5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT5_A::NOTSTARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT5_A::STARTED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT6_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOTSTARTED = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    STARTED = 1,
}
impl From<EPOUT6_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT6` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub struct EPOUT6_R(crate::FieldReader<bool, EPOUT6_A>);
impl EPOUT6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPOUT6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT6_A {
        match self.bits {
            false => EPOUT6_A::NOTSTARTED,
            true => EPOUT6_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        **self == EPOUT6_A::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        **self == EPOUT6_A::STARTED
    }
}
impl core::ops::Deref for EPOUT6_R {
    type Target = crate::FieldReader<bool, EPOUT6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPOUT6` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub struct EPOUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPOUT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPOUT6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT6_A::NOTSTARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT6_A::STARTED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Acknowledged data transfer on this OUT endpoint. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT7_A {
    #[doc = "0: No acknowledged data transfer on this endpoint"]
    NOTSTARTED = 0,
    #[doc = "1: Acknowledged data transfer on this endpoint has occurred"]
    STARTED = 1,
}
impl From<EPOUT7_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPOUT7` reader - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub struct EPOUT7_R(crate::FieldReader<bool, EPOUT7_A>);
impl EPOUT7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPOUT7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT7_A {
        match self.bits {
            false => EPOUT7_A::NOTSTARTED,
            true => EPOUT7_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        **self == EPOUT7_A::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        **self == EPOUT7_A::STARTED
    }
}
impl core::ops::Deref for EPOUT7_R {
    type Target = crate::FieldReader<bool, EPOUT7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPOUT7` writer - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
pub struct EPOUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPOUT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPOUT7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No acknowledged data transfer on this endpoint"]
    #[inline(always)]
    pub fn not_started(self) -> &'a mut W {
        self.variant(EPOUT7_A::NOTSTARTED)
    }
    #[doc = "Acknowledged data transfer on this endpoint has occurred"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(EPOUT7_A::STARTED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin1(&self) -> EPIN1_R {
        EPIN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin2(&self) -> EPIN2_R {
        EPIN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin3(&self) -> EPIN3_R {
        EPIN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin4(&self) -> EPIN4_R {
        EPIN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin5(&self) -> EPIN5_R {
        EPIN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin6(&self) -> EPIN6_R {
        EPIN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin7(&self) -> EPIN7_R {
        EPIN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout1(&self) -> EPOUT1_R {
        EPOUT1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout2(&self) -> EPOUT2_R {
        EPOUT2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout3(&self) -> EPOUT3_R {
        EPOUT3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout4(&self) -> EPOUT4_R {
        EPOUT4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout5(&self) -> EPOUT5_R {
        EPOUT5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout6(&self) -> EPOUT6_R {
        EPOUT6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout7(&self) -> EPOUT7_R {
        EPOUT7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin1(&mut self) -> EPIN1_W {
        EPIN1_W { w: self }
    }
    #[doc = "Bit 2 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin2(&mut self) -> EPIN2_W {
        EPIN2_W { w: self }
    }
    #[doc = "Bit 3 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin3(&mut self) -> EPIN3_W {
        EPIN3_W { w: self }
    }
    #[doc = "Bit 4 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin4(&mut self) -> EPIN4_W {
        EPIN4_W { w: self }
    }
    #[doc = "Bit 5 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin5(&mut self) -> EPIN5_W {
        EPIN5_W { w: self }
    }
    #[doc = "Bit 6 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin6(&mut self) -> EPIN6_W {
        EPIN6_W { w: self }
    }
    #[doc = "Bit 7 - Acknowledged data transfer on this IN endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epin7(&mut self) -> EPIN7_W {
        EPIN7_W { w: self }
    }
    #[doc = "Bit 17 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout1(&mut self) -> EPOUT1_W {
        EPOUT1_W { w: self }
    }
    #[doc = "Bit 18 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout2(&mut self) -> EPOUT2_W {
        EPOUT2_W { w: self }
    }
    #[doc = "Bit 19 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout3(&mut self) -> EPOUT3_W {
        EPOUT3_W { w: self }
    }
    #[doc = "Bit 20 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout4(&mut self) -> EPOUT4_W {
        EPOUT4_W { w: self }
    }
    #[doc = "Bit 21 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout5(&mut self) -> EPOUT5_W {
        EPOUT5_W { w: self }
    }
    #[doc = "Bit 22 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout6(&mut self) -> EPOUT6_W {
        EPOUT6_W { w: self }
    }
    #[doc = "Bit 23 - Acknowledged data transfer on this OUT endpoint. Write '1' to clear."]
    #[inline(always)]
    pub fn epout7(&mut self) -> EPOUT7_W {
        EPOUT7_W { w: self }
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
