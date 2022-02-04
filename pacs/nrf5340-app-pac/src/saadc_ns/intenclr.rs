#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Write '1' to disable interrupt for event STARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<STARTED_A> for bool {
    #[inline(always)]
    fn from(variant: STARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTED` reader - Write '1' to disable interrupt for event STARTED"]
pub struct STARTED_R(crate::FieldReader<bool, STARTED_A>);
impl STARTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STARTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == STARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == STARTED_A::ENABLED
    }
}
impl core::ops::Deref for STARTED_R {
    type Target = crate::FieldReader<bool, STARTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event STARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<STARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: STARTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTED` writer - Write '1' to disable interrupt for event STARTED"]
pub struct STARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTED_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(STARTED_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<END_A> for bool {
    #[inline(always)]
    fn from(variant: END_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` reader - Write '1' to disable interrupt for event END"]
pub struct END_R(crate::FieldReader<bool, END_A>);
impl END_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        END_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == END_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == END_A::ENABLED
    }
}
impl core::ops::Deref for END_R {
    type Target = crate::FieldReader<bool, END_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<END_AW> for bool {
    #[inline(always)]
    fn from(variant: END_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` writer - Write '1' to disable interrupt for event END"]
pub struct END_W<'a> {
    w: &'a mut W,
}
impl<'a> END_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: END_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(END_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event DONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DONE_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - Write '1' to disable interrupt for event DONE"]
pub struct DONE_R(crate::FieldReader<bool, DONE_A>);
impl DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == DONE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DONE_A::ENABLED
    }
}
impl core::ops::Deref for DONE_R {
    type Target = crate::FieldReader<bool, DONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event DONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DONE_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<DONE_AW> for bool {
    #[inline(always)]
    fn from(variant: DONE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` writer - Write '1' to disable interrupt for event DONE"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DONE_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DONE_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event RESULTDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESULTDONE_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RESULTDONE_A> for bool {
    #[inline(always)]
    fn from(variant: RESULTDONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESULTDONE` reader - Write '1' to disable interrupt for event RESULTDONE"]
pub struct RESULTDONE_R(crate::FieldReader<bool, RESULTDONE_A>);
impl RESULTDONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESULTDONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RESULTDONE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RESULTDONE_A::ENABLED
    }
}
impl core::ops::Deref for RESULTDONE_R {
    type Target = crate::FieldReader<bool, RESULTDONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event RESULTDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESULTDONE_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RESULTDONE_AW> for bool {
    #[inline(always)]
    fn from(variant: RESULTDONE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESULTDONE` writer - Write '1' to disable interrupt for event RESULTDONE"]
pub struct RESULTDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESULTDONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESULTDONE_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RESULTDONE_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event CALIBRATEDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALIBRATEDONE_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CALIBRATEDONE_A> for bool {
    #[inline(always)]
    fn from(variant: CALIBRATEDONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALIBRATEDONE` reader - Write '1' to disable interrupt for event CALIBRATEDONE"]
pub struct CALIBRATEDONE_R(crate::FieldReader<bool, CALIBRATEDONE_A>);
impl CALIBRATEDONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CALIBRATEDONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CALIBRATEDONE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CALIBRATEDONE_A::ENABLED
    }
}
impl core::ops::Deref for CALIBRATEDONE_R {
    type Target = crate::FieldReader<bool, CALIBRATEDONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CALIBRATEDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALIBRATEDONE_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CALIBRATEDONE_AW> for bool {
    #[inline(always)]
    fn from(variant: CALIBRATEDONE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALIBRATEDONE` writer - Write '1' to disable interrupt for event CALIBRATEDONE"]
pub struct CALIBRATEDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CALIBRATEDONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALIBRATEDONE_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CALIBRATEDONE_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPPED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<STOPPED_A> for bool {
    #[inline(always)]
    fn from(variant: STOPPED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` reader - Write '1' to disable interrupt for event STOPPED"]
pub struct STOPPED_R(crate::FieldReader<bool, STOPPED_A>);
impl STOPPED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOPPED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == STOPPED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == STOPPED_A::ENABLED
    }
}
impl core::ops::Deref for STOPPED_R {
    type Target = crate::FieldReader<bool, STOPPED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPPED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<STOPPED_AW> for bool {
    #[inline(always)]
    fn from(variant: STOPPED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` writer - Write '1' to disable interrupt for event STOPPED"]
pub struct STOPPED_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPPED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPPED_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(STOPPED_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event CH0LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH0LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH0LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0LIMITH` reader - Write '1' to disable interrupt for event CH0LIMITH"]
pub struct CH0LIMITH_R(crate::FieldReader<bool, CH0LIMITH_A>);
impl CH0LIMITH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0LIMITH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CH0LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH0LIMITH_A::ENABLED
    }
}
impl core::ops::Deref for CH0LIMITH_R {
    type Target = crate::FieldReader<bool, CH0LIMITH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CH0LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0LIMITH_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CH0LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH0LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0LIMITH` writer - Write '1' to disable interrupt for event CH0LIMITH"]
pub struct CH0LIMITH_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0LIMITH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0LIMITH_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH0LIMITH_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event CH0LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH0LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH0LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0LIMITL` reader - Write '1' to disable interrupt for event CH0LIMITL"]
pub struct CH0LIMITL_R(crate::FieldReader<bool, CH0LIMITL_A>);
impl CH0LIMITL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0LIMITL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CH0LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH0LIMITL_A::ENABLED
    }
}
impl core::ops::Deref for CH0LIMITL_R {
    type Target = crate::FieldReader<bool, CH0LIMITL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CH0LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0LIMITL_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CH0LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH0LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0LIMITL` writer - Write '1' to disable interrupt for event CH0LIMITL"]
pub struct CH0LIMITL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0LIMITL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0LIMITL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH0LIMITL_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event CH1LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH1LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH1LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1LIMITH` reader - Write '1' to disable interrupt for event CH1LIMITH"]
pub struct CH1LIMITH_R(crate::FieldReader<bool, CH1LIMITH_A>);
impl CH1LIMITH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1LIMITH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CH1LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH1LIMITH_A::ENABLED
    }
}
impl core::ops::Deref for CH1LIMITH_R {
    type Target = crate::FieldReader<bool, CH1LIMITH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CH1LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1LIMITH_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CH1LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH1LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1LIMITH` writer - Write '1' to disable interrupt for event CH1LIMITH"]
pub struct CH1LIMITH_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1LIMITH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1LIMITH_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH1LIMITH_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event CH1LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH1LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH1LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1LIMITL` reader - Write '1' to disable interrupt for event CH1LIMITL"]
pub struct CH1LIMITL_R(crate::FieldReader<bool, CH1LIMITL_A>);
impl CH1LIMITL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1LIMITL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CH1LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH1LIMITL_A::ENABLED
    }
}
impl core::ops::Deref for CH1LIMITL_R {
    type Target = crate::FieldReader<bool, CH1LIMITL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CH1LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1LIMITL_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CH1LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH1LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1LIMITL` writer - Write '1' to disable interrupt for event CH1LIMITL"]
pub struct CH1LIMITL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1LIMITL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1LIMITL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH1LIMITL_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event CH2LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH2LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH2LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2LIMITH` reader - Write '1' to disable interrupt for event CH2LIMITH"]
pub struct CH2LIMITH_R(crate::FieldReader<bool, CH2LIMITH_A>);
impl CH2LIMITH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2LIMITH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CH2LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH2LIMITH_A::ENABLED
    }
}
impl core::ops::Deref for CH2LIMITH_R {
    type Target = crate::FieldReader<bool, CH2LIMITH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CH2LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2LIMITH_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CH2LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH2LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2LIMITH` writer - Write '1' to disable interrupt for event CH2LIMITH"]
pub struct CH2LIMITH_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2LIMITH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2LIMITH_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH2LIMITH_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event CH2LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH2LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH2LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2LIMITL` reader - Write '1' to disable interrupt for event CH2LIMITL"]
pub struct CH2LIMITL_R(crate::FieldReader<bool, CH2LIMITL_A>);
impl CH2LIMITL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2LIMITL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CH2LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH2LIMITL_A::ENABLED
    }
}
impl core::ops::Deref for CH2LIMITL_R {
    type Target = crate::FieldReader<bool, CH2LIMITL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CH2LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2LIMITL_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CH2LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH2LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2LIMITL` writer - Write '1' to disable interrupt for event CH2LIMITL"]
pub struct CH2LIMITL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2LIMITL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2LIMITL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH2LIMITL_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event CH3LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH3LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH3LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3LIMITH` reader - Write '1' to disable interrupt for event CH3LIMITH"]
pub struct CH3LIMITH_R(crate::FieldReader<bool, CH3LIMITH_A>);
impl CH3LIMITH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3LIMITH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CH3LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH3LIMITH_A::ENABLED
    }
}
impl core::ops::Deref for CH3LIMITH_R {
    type Target = crate::FieldReader<bool, CH3LIMITH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CH3LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3LIMITH_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CH3LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH3LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3LIMITH` writer - Write '1' to disable interrupt for event CH3LIMITH"]
pub struct CH3LIMITH_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3LIMITH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3LIMITH_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH3LIMITH_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event CH3LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH3LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH3LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3LIMITL` reader - Write '1' to disable interrupt for event CH3LIMITL"]
pub struct CH3LIMITL_R(crate::FieldReader<bool, CH3LIMITL_A>);
impl CH3LIMITL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3LIMITL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CH3LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH3LIMITL_A::ENABLED
    }
}
impl core::ops::Deref for CH3LIMITL_R {
    type Target = crate::FieldReader<bool, CH3LIMITL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CH3LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3LIMITL_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CH3LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH3LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3LIMITL` writer - Write '1' to disable interrupt for event CH3LIMITL"]
pub struct CH3LIMITL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3LIMITL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3LIMITL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH3LIMITL_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event CH4LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH4LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH4LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4LIMITH` reader - Write '1' to disable interrupt for event CH4LIMITH"]
pub struct CH4LIMITH_R(crate::FieldReader<bool, CH4LIMITH_A>);
impl CH4LIMITH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH4LIMITH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CH4LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH4LIMITH_A::ENABLED
    }
}
impl core::ops::Deref for CH4LIMITH_R {
    type Target = crate::FieldReader<bool, CH4LIMITH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CH4LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4LIMITH_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CH4LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH4LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4LIMITH` writer - Write '1' to disable interrupt for event CH4LIMITH"]
pub struct CH4LIMITH_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4LIMITH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4LIMITH_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH4LIMITH_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event CH4LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH4LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH4LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4LIMITL` reader - Write '1' to disable interrupt for event CH4LIMITL"]
pub struct CH4LIMITL_R(crate::FieldReader<bool, CH4LIMITL_A>);
impl CH4LIMITL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH4LIMITL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CH4LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH4LIMITL_A::ENABLED
    }
}
impl core::ops::Deref for CH4LIMITL_R {
    type Target = crate::FieldReader<bool, CH4LIMITL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CH4LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4LIMITL_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CH4LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH4LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4LIMITL` writer - Write '1' to disable interrupt for event CH4LIMITL"]
pub struct CH4LIMITL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4LIMITL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4LIMITL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH4LIMITL_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event CH5LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH5LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH5LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5LIMITH` reader - Write '1' to disable interrupt for event CH5LIMITH"]
pub struct CH5LIMITH_R(crate::FieldReader<bool, CH5LIMITH_A>);
impl CH5LIMITH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH5LIMITH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CH5LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH5LIMITH_A::ENABLED
    }
}
impl core::ops::Deref for CH5LIMITH_R {
    type Target = crate::FieldReader<bool, CH5LIMITH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CH5LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5LIMITH_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CH5LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH5LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5LIMITH` writer - Write '1' to disable interrupt for event CH5LIMITH"]
pub struct CH5LIMITH_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5LIMITH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5LIMITH_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH5LIMITH_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event CH5LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH5LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH5LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5LIMITL` reader - Write '1' to disable interrupt for event CH5LIMITL"]
pub struct CH5LIMITL_R(crate::FieldReader<bool, CH5LIMITL_A>);
impl CH5LIMITL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH5LIMITL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CH5LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH5LIMITL_A::ENABLED
    }
}
impl core::ops::Deref for CH5LIMITL_R {
    type Target = crate::FieldReader<bool, CH5LIMITL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CH5LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5LIMITL_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CH5LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH5LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5LIMITL` writer - Write '1' to disable interrupt for event CH5LIMITL"]
pub struct CH5LIMITL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5LIMITL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5LIMITL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH5LIMITL_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event CH6LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH6LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH6LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6LIMITH` reader - Write '1' to disable interrupt for event CH6LIMITH"]
pub struct CH6LIMITH_R(crate::FieldReader<bool, CH6LIMITH_A>);
impl CH6LIMITH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH6LIMITH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CH6LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH6LIMITH_A::ENABLED
    }
}
impl core::ops::Deref for CH6LIMITH_R {
    type Target = crate::FieldReader<bool, CH6LIMITH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CH6LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6LIMITH_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CH6LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH6LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6LIMITH` writer - Write '1' to disable interrupt for event CH6LIMITH"]
pub struct CH6LIMITH_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6LIMITH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6LIMITH_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH6LIMITH_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event CH6LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH6LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH6LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6LIMITL` reader - Write '1' to disable interrupt for event CH6LIMITL"]
pub struct CH6LIMITL_R(crate::FieldReader<bool, CH6LIMITL_A>);
impl CH6LIMITL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH6LIMITL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CH6LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH6LIMITL_A::ENABLED
    }
}
impl core::ops::Deref for CH6LIMITL_R {
    type Target = crate::FieldReader<bool, CH6LIMITL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CH6LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6LIMITL_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CH6LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH6LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6LIMITL` writer - Write '1' to disable interrupt for event CH6LIMITL"]
pub struct CH6LIMITL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6LIMITL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6LIMITL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH6LIMITL_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event CH7LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH7LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH7LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7LIMITH` reader - Write '1' to disable interrupt for event CH7LIMITH"]
pub struct CH7LIMITH_R(crate::FieldReader<bool, CH7LIMITH_A>);
impl CH7LIMITH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH7LIMITH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CH7LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH7LIMITH_A::ENABLED
    }
}
impl core::ops::Deref for CH7LIMITH_R {
    type Target = crate::FieldReader<bool, CH7LIMITH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CH7LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7LIMITH_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CH7LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH7LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7LIMITH` writer - Write '1' to disable interrupt for event CH7LIMITH"]
pub struct CH7LIMITH_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7LIMITH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7LIMITH_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH7LIMITH_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event CH7LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH7LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH7LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7LIMITL` reader - Write '1' to disable interrupt for event CH7LIMITL"]
pub struct CH7LIMITL_R(crate::FieldReader<bool, CH7LIMITL_A>);
impl CH7LIMITL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH7LIMITL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CH7LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH7LIMITL_A::ENABLED
    }
}
impl core::ops::Deref for CH7LIMITL_R {
    type Target = crate::FieldReader<bool, CH7LIMITL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event CH7LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7LIMITL_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CH7LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH7LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7LIMITL` writer - Write '1' to disable interrupt for event CH7LIMITL"]
pub struct CH7LIMITL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7LIMITL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7LIMITL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH7LIMITL_AW::CLEAR)
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
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event STARTED"]
    #[inline(always)]
    pub fn started(&self) -> STARTED_R {
        STARTED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event END"]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event DONE"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event RESULTDONE"]
    #[inline(always)]
    pub fn resultdone(&self) -> RESULTDONE_R {
        RESULTDONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event CALIBRATEDONE"]
    #[inline(always)]
    pub fn calibratedone(&self) -> CALIBRATEDONE_R {
        CALIBRATEDONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&self) -> STOPPED_R {
        STOPPED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event CH0LIMITH"]
    #[inline(always)]
    pub fn ch0limith(&self) -> CH0LIMITH_R {
        CH0LIMITH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event CH0LIMITL"]
    #[inline(always)]
    pub fn ch0limitl(&self) -> CH0LIMITL_R {
        CH0LIMITL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write '1' to disable interrupt for event CH1LIMITH"]
    #[inline(always)]
    pub fn ch1limith(&self) -> CH1LIMITH_R {
        CH1LIMITH_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write '1' to disable interrupt for event CH1LIMITL"]
    #[inline(always)]
    pub fn ch1limitl(&self) -> CH1LIMITL_R {
        CH1LIMITL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Write '1' to disable interrupt for event CH2LIMITH"]
    #[inline(always)]
    pub fn ch2limith(&self) -> CH2LIMITH_R {
        CH2LIMITH_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Write '1' to disable interrupt for event CH2LIMITL"]
    #[inline(always)]
    pub fn ch2limitl(&self) -> CH2LIMITL_R {
        CH2LIMITL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Write '1' to disable interrupt for event CH3LIMITH"]
    #[inline(always)]
    pub fn ch3limith(&self) -> CH3LIMITH_R {
        CH3LIMITH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Write '1' to disable interrupt for event CH3LIMITL"]
    #[inline(always)]
    pub fn ch3limitl(&self) -> CH3LIMITL_R {
        CH3LIMITL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Write '1' to disable interrupt for event CH4LIMITH"]
    #[inline(always)]
    pub fn ch4limith(&self) -> CH4LIMITH_R {
        CH4LIMITH_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Write '1' to disable interrupt for event CH4LIMITL"]
    #[inline(always)]
    pub fn ch4limitl(&self) -> CH4LIMITL_R {
        CH4LIMITL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Write '1' to disable interrupt for event CH5LIMITH"]
    #[inline(always)]
    pub fn ch5limith(&self) -> CH5LIMITH_R {
        CH5LIMITH_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Write '1' to disable interrupt for event CH5LIMITL"]
    #[inline(always)]
    pub fn ch5limitl(&self) -> CH5LIMITL_R {
        CH5LIMITL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event CH6LIMITH"]
    #[inline(always)]
    pub fn ch6limith(&self) -> CH6LIMITH_R {
        CH6LIMITH_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event CH6LIMITL"]
    #[inline(always)]
    pub fn ch6limitl(&self) -> CH6LIMITL_R {
        CH6LIMITL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for event CH7LIMITH"]
    #[inline(always)]
    pub fn ch7limith(&self) -> CH7LIMITH_R {
        CH7LIMITH_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Write '1' to disable interrupt for event CH7LIMITL"]
    #[inline(always)]
    pub fn ch7limitl(&self) -> CH7LIMITL_R {
        CH7LIMITL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event STARTED"]
    #[inline(always)]
    pub fn started(&mut self) -> STARTED_W {
        STARTED_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event END"]
    #[inline(always)]
    pub fn end(&mut self) -> END_W {
        END_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event DONE"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event RESULTDONE"]
    #[inline(always)]
    pub fn resultdone(&mut self) -> RESULTDONE_W {
        RESULTDONE_W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event CALIBRATEDONE"]
    #[inline(always)]
    pub fn calibratedone(&mut self) -> CALIBRATEDONE_W {
        CALIBRATEDONE_W { w: self }
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&mut self) -> STOPPED_W {
        STOPPED_W { w: self }
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event CH0LIMITH"]
    #[inline(always)]
    pub fn ch0limith(&mut self) -> CH0LIMITH_W {
        CH0LIMITH_W { w: self }
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event CH0LIMITL"]
    #[inline(always)]
    pub fn ch0limitl(&mut self) -> CH0LIMITL_W {
        CH0LIMITL_W { w: self }
    }
    #[doc = "Bit 8 - Write '1' to disable interrupt for event CH1LIMITH"]
    #[inline(always)]
    pub fn ch1limith(&mut self) -> CH1LIMITH_W {
        CH1LIMITH_W { w: self }
    }
    #[doc = "Bit 9 - Write '1' to disable interrupt for event CH1LIMITL"]
    #[inline(always)]
    pub fn ch1limitl(&mut self) -> CH1LIMITL_W {
        CH1LIMITL_W { w: self }
    }
    #[doc = "Bit 10 - Write '1' to disable interrupt for event CH2LIMITH"]
    #[inline(always)]
    pub fn ch2limith(&mut self) -> CH2LIMITH_W {
        CH2LIMITH_W { w: self }
    }
    #[doc = "Bit 11 - Write '1' to disable interrupt for event CH2LIMITL"]
    #[inline(always)]
    pub fn ch2limitl(&mut self) -> CH2LIMITL_W {
        CH2LIMITL_W { w: self }
    }
    #[doc = "Bit 12 - Write '1' to disable interrupt for event CH3LIMITH"]
    #[inline(always)]
    pub fn ch3limith(&mut self) -> CH3LIMITH_W {
        CH3LIMITH_W { w: self }
    }
    #[doc = "Bit 13 - Write '1' to disable interrupt for event CH3LIMITL"]
    #[inline(always)]
    pub fn ch3limitl(&mut self) -> CH3LIMITL_W {
        CH3LIMITL_W { w: self }
    }
    #[doc = "Bit 14 - Write '1' to disable interrupt for event CH4LIMITH"]
    #[inline(always)]
    pub fn ch4limith(&mut self) -> CH4LIMITH_W {
        CH4LIMITH_W { w: self }
    }
    #[doc = "Bit 15 - Write '1' to disable interrupt for event CH4LIMITL"]
    #[inline(always)]
    pub fn ch4limitl(&mut self) -> CH4LIMITL_W {
        CH4LIMITL_W { w: self }
    }
    #[doc = "Bit 16 - Write '1' to disable interrupt for event CH5LIMITH"]
    #[inline(always)]
    pub fn ch5limith(&mut self) -> CH5LIMITH_W {
        CH5LIMITH_W { w: self }
    }
    #[doc = "Bit 17 - Write '1' to disable interrupt for event CH5LIMITL"]
    #[inline(always)]
    pub fn ch5limitl(&mut self) -> CH5LIMITL_W {
        CH5LIMITL_W { w: self }
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event CH6LIMITH"]
    #[inline(always)]
    pub fn ch6limith(&mut self) -> CH6LIMITH_W {
        CH6LIMITH_W { w: self }
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event CH6LIMITL"]
    #[inline(always)]
    pub fn ch6limitl(&mut self) -> CH6LIMITL_W {
        CH6LIMITL_W { w: self }
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for event CH7LIMITH"]
    #[inline(always)]
    pub fn ch7limith(&mut self) -> CH7LIMITH_W {
        CH7LIMITH_W { w: self }
    }
    #[doc = "Bit 21 - Write '1' to disable interrupt for event CH7LIMITL"]
    #[inline(always)]
    pub fn ch7limitl(&mut self) -> CH7LIMITL_W {
        CH7LIMITL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
