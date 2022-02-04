#[doc = "Register `NMIENCLR` reader"]
pub struct R(crate::R<NMIENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMIENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NMIENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NMIENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NMIENCLR` writer"]
pub struct W(crate::W<NMIENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NMIENCLR_SPEC>;
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
impl From<crate::W<NMIENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NMIENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Write '1' to disable interrupt for event REGION0WA\n\nValue on reset: 0"]
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
#[doc = "Field `REGION0WA` reader - Write '1' to disable interrupt for event REGION0WA"]
pub struct REGION0WA_R(crate::FieldReader<bool, REGION0WA_A>);
impl REGION0WA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION0WA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == REGION0WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION0WA_A::ENABLED
    }
}
impl core::ops::Deref for REGION0WA_R {
    type Target = crate::FieldReader<bool, REGION0WA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event REGION0WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION0WA_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<REGION0WA_AW> for bool {
    #[inline(always)]
    fn from(variant: REGION0WA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION0WA` writer - Write '1' to disable interrupt for event REGION0WA"]
pub struct REGION0WA_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION0WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION0WA_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REGION0WA_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event REGION0RA\n\nValue on reset: 0"]
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
#[doc = "Field `REGION0RA` reader - Write '1' to disable interrupt for event REGION0RA"]
pub struct REGION0RA_R(crate::FieldReader<bool, REGION0RA_A>);
impl REGION0RA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION0RA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == REGION0RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION0RA_A::ENABLED
    }
}
impl core::ops::Deref for REGION0RA_R {
    type Target = crate::FieldReader<bool, REGION0RA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event REGION0RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION0RA_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<REGION0RA_AW> for bool {
    #[inline(always)]
    fn from(variant: REGION0RA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION0RA` writer - Write '1' to disable interrupt for event REGION0RA"]
pub struct REGION0RA_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION0RA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION0RA_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REGION0RA_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event REGION1WA\n\nValue on reset: 0"]
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
#[doc = "Field `REGION1WA` reader - Write '1' to disable interrupt for event REGION1WA"]
pub struct REGION1WA_R(crate::FieldReader<bool, REGION1WA_A>);
impl REGION1WA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION1WA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == REGION1WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION1WA_A::ENABLED
    }
}
impl core::ops::Deref for REGION1WA_R {
    type Target = crate::FieldReader<bool, REGION1WA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event REGION1WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION1WA_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<REGION1WA_AW> for bool {
    #[inline(always)]
    fn from(variant: REGION1WA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION1WA` writer - Write '1' to disable interrupt for event REGION1WA"]
pub struct REGION1WA_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION1WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION1WA_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REGION1WA_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event REGION1RA\n\nValue on reset: 0"]
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
#[doc = "Field `REGION1RA` reader - Write '1' to disable interrupt for event REGION1RA"]
pub struct REGION1RA_R(crate::FieldReader<bool, REGION1RA_A>);
impl REGION1RA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION1RA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == REGION1RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION1RA_A::ENABLED
    }
}
impl core::ops::Deref for REGION1RA_R {
    type Target = crate::FieldReader<bool, REGION1RA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event REGION1RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION1RA_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<REGION1RA_AW> for bool {
    #[inline(always)]
    fn from(variant: REGION1RA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION1RA` writer - Write '1' to disable interrupt for event REGION1RA"]
pub struct REGION1RA_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION1RA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION1RA_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REGION1RA_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event REGION2WA\n\nValue on reset: 0"]
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
#[doc = "Field `REGION2WA` reader - Write '1' to disable interrupt for event REGION2WA"]
pub struct REGION2WA_R(crate::FieldReader<bool, REGION2WA_A>);
impl REGION2WA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION2WA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == REGION2WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION2WA_A::ENABLED
    }
}
impl core::ops::Deref for REGION2WA_R {
    type Target = crate::FieldReader<bool, REGION2WA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event REGION2WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION2WA_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<REGION2WA_AW> for bool {
    #[inline(always)]
    fn from(variant: REGION2WA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION2WA` writer - Write '1' to disable interrupt for event REGION2WA"]
pub struct REGION2WA_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION2WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION2WA_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REGION2WA_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event REGION2RA\n\nValue on reset: 0"]
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
#[doc = "Field `REGION2RA` reader - Write '1' to disable interrupt for event REGION2RA"]
pub struct REGION2RA_R(crate::FieldReader<bool, REGION2RA_A>);
impl REGION2RA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION2RA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == REGION2RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION2RA_A::ENABLED
    }
}
impl core::ops::Deref for REGION2RA_R {
    type Target = crate::FieldReader<bool, REGION2RA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event REGION2RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION2RA_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<REGION2RA_AW> for bool {
    #[inline(always)]
    fn from(variant: REGION2RA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION2RA` writer - Write '1' to disable interrupt for event REGION2RA"]
pub struct REGION2RA_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION2RA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION2RA_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REGION2RA_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event REGION3WA\n\nValue on reset: 0"]
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
#[doc = "Field `REGION3WA` reader - Write '1' to disable interrupt for event REGION3WA"]
pub struct REGION3WA_R(crate::FieldReader<bool, REGION3WA_A>);
impl REGION3WA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION3WA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == REGION3WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION3WA_A::ENABLED
    }
}
impl core::ops::Deref for REGION3WA_R {
    type Target = crate::FieldReader<bool, REGION3WA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event REGION3WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION3WA_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<REGION3WA_AW> for bool {
    #[inline(always)]
    fn from(variant: REGION3WA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION3WA` writer - Write '1' to disable interrupt for event REGION3WA"]
pub struct REGION3WA_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION3WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION3WA_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REGION3WA_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event REGION3RA\n\nValue on reset: 0"]
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
#[doc = "Field `REGION3RA` reader - Write '1' to disable interrupt for event REGION3RA"]
pub struct REGION3RA_R(crate::FieldReader<bool, REGION3RA_A>);
impl REGION3RA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION3RA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == REGION3RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION3RA_A::ENABLED
    }
}
impl core::ops::Deref for REGION3RA_R {
    type Target = crate::FieldReader<bool, REGION3RA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event REGION3RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION3RA_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<REGION3RA_AW> for bool {
    #[inline(always)]
    fn from(variant: REGION3RA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION3RA` writer - Write '1' to disable interrupt for event REGION3RA"]
pub struct REGION3RA_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION3RA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION3RA_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REGION3RA_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event PREGION0WA\n\nValue on reset: 0"]
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
#[doc = "Field `PREGION0WA` reader - Write '1' to disable interrupt for event PREGION0WA"]
pub struct PREGION0WA_R(crate::FieldReader<bool, PREGION0WA_A>);
impl PREGION0WA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PREGION0WA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PREGION0WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PREGION0WA_A::ENABLED
    }
}
impl core::ops::Deref for PREGION0WA_R {
    type Target = crate::FieldReader<bool, PREGION0WA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event PREGION0WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREGION0WA_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<PREGION0WA_AW> for bool {
    #[inline(always)]
    fn from(variant: PREGION0WA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION0WA` writer - Write '1' to disable interrupt for event PREGION0WA"]
pub struct PREGION0WA_W<'a> {
    w: &'a mut W,
}
impl<'a> PREGION0WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREGION0WA_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PREGION0WA_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event PREGION0RA\n\nValue on reset: 0"]
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
#[doc = "Field `PREGION0RA` reader - Write '1' to disable interrupt for event PREGION0RA"]
pub struct PREGION0RA_R(crate::FieldReader<bool, PREGION0RA_A>);
impl PREGION0RA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PREGION0RA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PREGION0RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PREGION0RA_A::ENABLED
    }
}
impl core::ops::Deref for PREGION0RA_R {
    type Target = crate::FieldReader<bool, PREGION0RA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event PREGION0RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREGION0RA_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<PREGION0RA_AW> for bool {
    #[inline(always)]
    fn from(variant: PREGION0RA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION0RA` writer - Write '1' to disable interrupt for event PREGION0RA"]
pub struct PREGION0RA_W<'a> {
    w: &'a mut W,
}
impl<'a> PREGION0RA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREGION0RA_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PREGION0RA_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event PREGION1WA\n\nValue on reset: 0"]
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
#[doc = "Field `PREGION1WA` reader - Write '1' to disable interrupt for event PREGION1WA"]
pub struct PREGION1WA_R(crate::FieldReader<bool, PREGION1WA_A>);
impl PREGION1WA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PREGION1WA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PREGION1WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PREGION1WA_A::ENABLED
    }
}
impl core::ops::Deref for PREGION1WA_R {
    type Target = crate::FieldReader<bool, PREGION1WA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event PREGION1WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREGION1WA_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<PREGION1WA_AW> for bool {
    #[inline(always)]
    fn from(variant: PREGION1WA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION1WA` writer - Write '1' to disable interrupt for event PREGION1WA"]
pub struct PREGION1WA_W<'a> {
    w: &'a mut W,
}
impl<'a> PREGION1WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREGION1WA_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PREGION1WA_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event PREGION1RA\n\nValue on reset: 0"]
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
#[doc = "Field `PREGION1RA` reader - Write '1' to disable interrupt for event PREGION1RA"]
pub struct PREGION1RA_R(crate::FieldReader<bool, PREGION1RA_A>);
impl PREGION1RA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PREGION1RA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PREGION1RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PREGION1RA_A::ENABLED
    }
}
impl core::ops::Deref for PREGION1RA_R {
    type Target = crate::FieldReader<bool, PREGION1RA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event PREGION1RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREGION1RA_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<PREGION1RA_AW> for bool {
    #[inline(always)]
    fn from(variant: PREGION1RA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION1RA` writer - Write '1' to disable interrupt for event PREGION1RA"]
pub struct PREGION1RA_W<'a> {
    w: &'a mut W,
}
impl<'a> PREGION1RA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREGION1RA_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PREGION1RA_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event REGION0WA"]
    #[inline(always)]
    pub fn region0wa(&self) -> REGION0WA_R {
        REGION0WA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event REGION0RA"]
    #[inline(always)]
    pub fn region0ra(&self) -> REGION0RA_R {
        REGION0RA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event REGION1WA"]
    #[inline(always)]
    pub fn region1wa(&self) -> REGION1WA_R {
        REGION1WA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event REGION1RA"]
    #[inline(always)]
    pub fn region1ra(&self) -> REGION1RA_R {
        REGION1RA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event REGION2WA"]
    #[inline(always)]
    pub fn region2wa(&self) -> REGION2WA_R {
        REGION2WA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event REGION2RA"]
    #[inline(always)]
    pub fn region2ra(&self) -> REGION2RA_R {
        REGION2RA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event REGION3WA"]
    #[inline(always)]
    pub fn region3wa(&self) -> REGION3WA_R {
        REGION3WA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event REGION3RA"]
    #[inline(always)]
    pub fn region3ra(&self) -> REGION3RA_R {
        REGION3RA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Write '1' to disable interrupt for event PREGION0WA"]
    #[inline(always)]
    pub fn pregion0wa(&self) -> PREGION0WA_R {
        PREGION0WA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Write '1' to disable interrupt for event PREGION0RA"]
    #[inline(always)]
    pub fn pregion0ra(&self) -> PREGION0RA_R {
        PREGION0RA_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Write '1' to disable interrupt for event PREGION1WA"]
    #[inline(always)]
    pub fn pregion1wa(&self) -> PREGION1WA_R {
        PREGION1WA_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Write '1' to disable interrupt for event PREGION1RA"]
    #[inline(always)]
    pub fn pregion1ra(&self) -> PREGION1RA_R {
        PREGION1RA_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event REGION0WA"]
    #[inline(always)]
    pub fn region0wa(&mut self) -> REGION0WA_W {
        REGION0WA_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event REGION0RA"]
    #[inline(always)]
    pub fn region0ra(&mut self) -> REGION0RA_W {
        REGION0RA_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event REGION1WA"]
    #[inline(always)]
    pub fn region1wa(&mut self) -> REGION1WA_W {
        REGION1WA_W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event REGION1RA"]
    #[inline(always)]
    pub fn region1ra(&mut self) -> REGION1RA_W {
        REGION1RA_W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event REGION2WA"]
    #[inline(always)]
    pub fn region2wa(&mut self) -> REGION2WA_W {
        REGION2WA_W { w: self }
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event REGION2RA"]
    #[inline(always)]
    pub fn region2ra(&mut self) -> REGION2RA_W {
        REGION2RA_W { w: self }
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event REGION3WA"]
    #[inline(always)]
    pub fn region3wa(&mut self) -> REGION3WA_W {
        REGION3WA_W { w: self }
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event REGION3RA"]
    #[inline(always)]
    pub fn region3ra(&mut self) -> REGION3RA_W {
        REGION3RA_W { w: self }
    }
    #[doc = "Bit 24 - Write '1' to disable interrupt for event PREGION0WA"]
    #[inline(always)]
    pub fn pregion0wa(&mut self) -> PREGION0WA_W {
        PREGION0WA_W { w: self }
    }
    #[doc = "Bit 25 - Write '1' to disable interrupt for event PREGION0RA"]
    #[inline(always)]
    pub fn pregion0ra(&mut self) -> PREGION0RA_W {
        PREGION0RA_W { w: self }
    }
    #[doc = "Bit 26 - Write '1' to disable interrupt for event PREGION1WA"]
    #[inline(always)]
    pub fn pregion1wa(&mut self) -> PREGION1WA_W {
        PREGION1WA_W { w: self }
    }
    #[doc = "Bit 27 - Write '1' to disable interrupt for event PREGION1RA"]
    #[inline(always)]
    pub fn pregion1ra(&mut self) -> PREGION1RA_W {
        PREGION1RA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmienclr](index.html) module"]
pub struct NMIENCLR_SPEC;
impl crate::RegisterSpec for NMIENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nmienclr::R](R) reader structure"]
impl crate::Readable for NMIENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nmienclr::W](W) writer structure"]
impl crate::Writable for NMIENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NMIENCLR to value 0"]
impl crate::Resettable for NMIENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
