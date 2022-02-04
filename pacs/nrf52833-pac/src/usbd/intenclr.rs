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
#[doc = "Write '1' to disable interrupt for event USBRESET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRESET_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<USBRESET_A> for bool {
    #[inline(always)]
    fn from(variant: USBRESET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRESET` reader - Write '1' to disable interrupt for event USBRESET"]
pub struct USBRESET_R(crate::FieldReader<bool, USBRESET_A>);
impl USBRESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBRESET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBRESET_A {
        match self.bits {
            false => USBRESET_A::DISABLED,
            true => USBRESET_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == USBRESET_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == USBRESET_A::ENABLED
    }
}
impl core::ops::Deref for USBRESET_R {
    type Target = crate::FieldReader<bool, USBRESET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event USBRESET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRESET_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<USBRESET_AW> for bool {
    #[inline(always)]
    fn from(variant: USBRESET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRESET` writer - Write '1' to disable interrupt for event USBRESET"]
pub struct USBRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBRESET_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(USBRESET_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event ENDEPIN\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN0_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDEPIN0_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN0` reader - Write '1' to disable interrupt for event ENDEPIN\\[0\\]"]
pub struct ENDEPIN0_R(crate::FieldReader<bool, ENDEPIN0_A>);
impl ENDEPIN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDEPIN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN0_A {
        match self.bits {
            false => ENDEPIN0_A::DISABLED,
            true => ENDEPIN0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDEPIN0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDEPIN0_A::ENABLED
    }
}
impl core::ops::Deref for ENDEPIN0_R {
    type Target = crate::FieldReader<bool, ENDEPIN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event ENDEPIN\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN0_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDEPIN0_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN0` writer - Write '1' to disable interrupt for event ENDEPIN\\[0\\]"]
pub struct ENDEPIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPIN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPIN0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPIN0_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event ENDEPIN\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN1_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDEPIN1_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN1` reader - Write '1' to disable interrupt for event ENDEPIN\\[1\\]"]
pub struct ENDEPIN1_R(crate::FieldReader<bool, ENDEPIN1_A>);
impl ENDEPIN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDEPIN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN1_A {
        match self.bits {
            false => ENDEPIN1_A::DISABLED,
            true => ENDEPIN1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDEPIN1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDEPIN1_A::ENABLED
    }
}
impl core::ops::Deref for ENDEPIN1_R {
    type Target = crate::FieldReader<bool, ENDEPIN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event ENDEPIN\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN1_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDEPIN1_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN1` writer - Write '1' to disable interrupt for event ENDEPIN\\[1\\]"]
pub struct ENDEPIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPIN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPIN1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPIN1_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event ENDEPIN\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN2_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDEPIN2_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN2` reader - Write '1' to disable interrupt for event ENDEPIN\\[2\\]"]
pub struct ENDEPIN2_R(crate::FieldReader<bool, ENDEPIN2_A>);
impl ENDEPIN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDEPIN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN2_A {
        match self.bits {
            false => ENDEPIN2_A::DISABLED,
            true => ENDEPIN2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDEPIN2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDEPIN2_A::ENABLED
    }
}
impl core::ops::Deref for ENDEPIN2_R {
    type Target = crate::FieldReader<bool, ENDEPIN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event ENDEPIN\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN2_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDEPIN2_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN2` writer - Write '1' to disable interrupt for event ENDEPIN\\[2\\]"]
pub struct ENDEPIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPIN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPIN2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPIN2_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event ENDEPIN\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN3_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDEPIN3_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN3` reader - Write '1' to disable interrupt for event ENDEPIN\\[3\\]"]
pub struct ENDEPIN3_R(crate::FieldReader<bool, ENDEPIN3_A>);
impl ENDEPIN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDEPIN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN3_A {
        match self.bits {
            false => ENDEPIN3_A::DISABLED,
            true => ENDEPIN3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDEPIN3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDEPIN3_A::ENABLED
    }
}
impl core::ops::Deref for ENDEPIN3_R {
    type Target = crate::FieldReader<bool, ENDEPIN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event ENDEPIN\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN3_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDEPIN3_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN3` writer - Write '1' to disable interrupt for event ENDEPIN\\[3\\]"]
pub struct ENDEPIN3_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPIN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPIN3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPIN3_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event ENDEPIN\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN4_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDEPIN4_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN4` reader - Write '1' to disable interrupt for event ENDEPIN\\[4\\]"]
pub struct ENDEPIN4_R(crate::FieldReader<bool, ENDEPIN4_A>);
impl ENDEPIN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDEPIN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN4_A {
        match self.bits {
            false => ENDEPIN4_A::DISABLED,
            true => ENDEPIN4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDEPIN4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDEPIN4_A::ENABLED
    }
}
impl core::ops::Deref for ENDEPIN4_R {
    type Target = crate::FieldReader<bool, ENDEPIN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event ENDEPIN\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN4_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDEPIN4_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN4` writer - Write '1' to disable interrupt for event ENDEPIN\\[4\\]"]
pub struct ENDEPIN4_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPIN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPIN4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPIN4_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event ENDEPIN\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN5_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDEPIN5_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN5` reader - Write '1' to disable interrupt for event ENDEPIN\\[5\\]"]
pub struct ENDEPIN5_R(crate::FieldReader<bool, ENDEPIN5_A>);
impl ENDEPIN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDEPIN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN5_A {
        match self.bits {
            false => ENDEPIN5_A::DISABLED,
            true => ENDEPIN5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDEPIN5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDEPIN5_A::ENABLED
    }
}
impl core::ops::Deref for ENDEPIN5_R {
    type Target = crate::FieldReader<bool, ENDEPIN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event ENDEPIN\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN5_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDEPIN5_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN5` writer - Write '1' to disable interrupt for event ENDEPIN\\[5\\]"]
pub struct ENDEPIN5_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPIN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPIN5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPIN5_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event ENDEPIN\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN6_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDEPIN6_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN6` reader - Write '1' to disable interrupt for event ENDEPIN\\[6\\]"]
pub struct ENDEPIN6_R(crate::FieldReader<bool, ENDEPIN6_A>);
impl ENDEPIN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDEPIN6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN6_A {
        match self.bits {
            false => ENDEPIN6_A::DISABLED,
            true => ENDEPIN6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDEPIN6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDEPIN6_A::ENABLED
    }
}
impl core::ops::Deref for ENDEPIN6_R {
    type Target = crate::FieldReader<bool, ENDEPIN6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event ENDEPIN\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN6_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDEPIN6_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN6` writer - Write '1' to disable interrupt for event ENDEPIN\\[6\\]"]
pub struct ENDEPIN6_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPIN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPIN6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPIN6_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event ENDEPIN\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN7_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDEPIN7_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN7` reader - Write '1' to disable interrupt for event ENDEPIN\\[7\\]"]
pub struct ENDEPIN7_R(crate::FieldReader<bool, ENDEPIN7_A>);
impl ENDEPIN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDEPIN7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN7_A {
        match self.bits {
            false => ENDEPIN7_A::DISABLED,
            true => ENDEPIN7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDEPIN7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDEPIN7_A::ENABLED
    }
}
impl core::ops::Deref for ENDEPIN7_R {
    type Target = crate::FieldReader<bool, ENDEPIN7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event ENDEPIN\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN7_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDEPIN7_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPIN7` writer - Write '1' to disable interrupt for event ENDEPIN\\[7\\]"]
pub struct ENDEPIN7_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPIN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPIN7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPIN7_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event EP0DATADONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0DATADONE_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<EP0DATADONE_A> for bool {
    #[inline(always)]
    fn from(variant: EP0DATADONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0DATADONE` reader - Write '1' to disable interrupt for event EP0DATADONE"]
pub struct EP0DATADONE_R(crate::FieldReader<bool, EP0DATADONE_A>);
impl EP0DATADONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0DATADONE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP0DATADONE_A {
        match self.bits {
            false => EP0DATADONE_A::DISABLED,
            true => EP0DATADONE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EP0DATADONE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EP0DATADONE_A::ENABLED
    }
}
impl core::ops::Deref for EP0DATADONE_R {
    type Target = crate::FieldReader<bool, EP0DATADONE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event EP0DATADONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0DATADONE_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<EP0DATADONE_AW> for bool {
    #[inline(always)]
    fn from(variant: EP0DATADONE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0DATADONE` writer - Write '1' to disable interrupt for event EP0DATADONE"]
pub struct EP0DATADONE_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0DATADONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP0DATADONE_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EP0DATADONE_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event ENDISOIN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDISOIN_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDISOIN_A> for bool {
    #[inline(always)]
    fn from(variant: ENDISOIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDISOIN` reader - Write '1' to disable interrupt for event ENDISOIN"]
pub struct ENDISOIN_R(crate::FieldReader<bool, ENDISOIN_A>);
impl ENDISOIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDISOIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDISOIN_A {
        match self.bits {
            false => ENDISOIN_A::DISABLED,
            true => ENDISOIN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDISOIN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDISOIN_A::ENABLED
    }
}
impl core::ops::Deref for ENDISOIN_R {
    type Target = crate::FieldReader<bool, ENDISOIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event ENDISOIN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDISOIN_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDISOIN_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDISOIN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDISOIN` writer - Write '1' to disable interrupt for event ENDISOIN"]
pub struct ENDISOIN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDISOIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDISOIN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDISOIN_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event ENDEPOUT\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT0_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDEPOUT0_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT0` reader - Write '1' to disable interrupt for event ENDEPOUT\\[0\\]"]
pub struct ENDEPOUT0_R(crate::FieldReader<bool, ENDEPOUT0_A>);
impl ENDEPOUT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDEPOUT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT0_A {
        match self.bits {
            false => ENDEPOUT0_A::DISABLED,
            true => ENDEPOUT0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDEPOUT0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDEPOUT0_A::ENABLED
    }
}
impl core::ops::Deref for ENDEPOUT0_R {
    type Target = crate::FieldReader<bool, ENDEPOUT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event ENDEPOUT\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT0_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDEPOUT0_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT0` writer - Write '1' to disable interrupt for event ENDEPOUT\\[0\\]"]
pub struct ENDEPOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPOUT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPOUT0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPOUT0_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event ENDEPOUT\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT1_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDEPOUT1_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT1` reader - Write '1' to disable interrupt for event ENDEPOUT\\[1\\]"]
pub struct ENDEPOUT1_R(crate::FieldReader<bool, ENDEPOUT1_A>);
impl ENDEPOUT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDEPOUT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT1_A {
        match self.bits {
            false => ENDEPOUT1_A::DISABLED,
            true => ENDEPOUT1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDEPOUT1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDEPOUT1_A::ENABLED
    }
}
impl core::ops::Deref for ENDEPOUT1_R {
    type Target = crate::FieldReader<bool, ENDEPOUT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event ENDEPOUT\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT1_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDEPOUT1_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT1` writer - Write '1' to disable interrupt for event ENDEPOUT\\[1\\]"]
pub struct ENDEPOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPOUT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPOUT1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPOUT1_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event ENDEPOUT\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT2_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDEPOUT2_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT2` reader - Write '1' to disable interrupt for event ENDEPOUT\\[2\\]"]
pub struct ENDEPOUT2_R(crate::FieldReader<bool, ENDEPOUT2_A>);
impl ENDEPOUT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDEPOUT2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT2_A {
        match self.bits {
            false => ENDEPOUT2_A::DISABLED,
            true => ENDEPOUT2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDEPOUT2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDEPOUT2_A::ENABLED
    }
}
impl core::ops::Deref for ENDEPOUT2_R {
    type Target = crate::FieldReader<bool, ENDEPOUT2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event ENDEPOUT\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT2_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDEPOUT2_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT2` writer - Write '1' to disable interrupt for event ENDEPOUT\\[2\\]"]
pub struct ENDEPOUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPOUT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPOUT2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPOUT2_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event ENDEPOUT\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT3_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDEPOUT3_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT3` reader - Write '1' to disable interrupt for event ENDEPOUT\\[3\\]"]
pub struct ENDEPOUT3_R(crate::FieldReader<bool, ENDEPOUT3_A>);
impl ENDEPOUT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDEPOUT3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT3_A {
        match self.bits {
            false => ENDEPOUT3_A::DISABLED,
            true => ENDEPOUT3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDEPOUT3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDEPOUT3_A::ENABLED
    }
}
impl core::ops::Deref for ENDEPOUT3_R {
    type Target = crate::FieldReader<bool, ENDEPOUT3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event ENDEPOUT\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT3_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDEPOUT3_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT3` writer - Write '1' to disable interrupt for event ENDEPOUT\\[3\\]"]
pub struct ENDEPOUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPOUT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPOUT3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPOUT3_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event ENDEPOUT\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT4_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDEPOUT4_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT4` reader - Write '1' to disable interrupt for event ENDEPOUT\\[4\\]"]
pub struct ENDEPOUT4_R(crate::FieldReader<bool, ENDEPOUT4_A>);
impl ENDEPOUT4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDEPOUT4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT4_A {
        match self.bits {
            false => ENDEPOUT4_A::DISABLED,
            true => ENDEPOUT4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDEPOUT4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDEPOUT4_A::ENABLED
    }
}
impl core::ops::Deref for ENDEPOUT4_R {
    type Target = crate::FieldReader<bool, ENDEPOUT4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event ENDEPOUT\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT4_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDEPOUT4_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT4` writer - Write '1' to disable interrupt for event ENDEPOUT\\[4\\]"]
pub struct ENDEPOUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPOUT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPOUT4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPOUT4_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event ENDEPOUT\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT5_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDEPOUT5_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT5` reader - Write '1' to disable interrupt for event ENDEPOUT\\[5\\]"]
pub struct ENDEPOUT5_R(crate::FieldReader<bool, ENDEPOUT5_A>);
impl ENDEPOUT5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDEPOUT5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT5_A {
        match self.bits {
            false => ENDEPOUT5_A::DISABLED,
            true => ENDEPOUT5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDEPOUT5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDEPOUT5_A::ENABLED
    }
}
impl core::ops::Deref for ENDEPOUT5_R {
    type Target = crate::FieldReader<bool, ENDEPOUT5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event ENDEPOUT\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT5_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDEPOUT5_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT5` writer - Write '1' to disable interrupt for event ENDEPOUT\\[5\\]"]
pub struct ENDEPOUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPOUT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPOUT5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPOUT5_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event ENDEPOUT\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT6_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDEPOUT6_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT6` reader - Write '1' to disable interrupt for event ENDEPOUT\\[6\\]"]
pub struct ENDEPOUT6_R(crate::FieldReader<bool, ENDEPOUT6_A>);
impl ENDEPOUT6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDEPOUT6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT6_A {
        match self.bits {
            false => ENDEPOUT6_A::DISABLED,
            true => ENDEPOUT6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDEPOUT6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDEPOUT6_A::ENABLED
    }
}
impl core::ops::Deref for ENDEPOUT6_R {
    type Target = crate::FieldReader<bool, ENDEPOUT6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event ENDEPOUT\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT6_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDEPOUT6_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT6` writer - Write '1' to disable interrupt for event ENDEPOUT\\[6\\]"]
pub struct ENDEPOUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPOUT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPOUT6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPOUT6_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event ENDEPOUT\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT7_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDEPOUT7_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT7` reader - Write '1' to disable interrupt for event ENDEPOUT\\[7\\]"]
pub struct ENDEPOUT7_R(crate::FieldReader<bool, ENDEPOUT7_A>);
impl ENDEPOUT7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDEPOUT7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT7_A {
        match self.bits {
            false => ENDEPOUT7_A::DISABLED,
            true => ENDEPOUT7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDEPOUT7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDEPOUT7_A::ENABLED
    }
}
impl core::ops::Deref for ENDEPOUT7_R {
    type Target = crate::FieldReader<bool, ENDEPOUT7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event ENDEPOUT\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT7_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDEPOUT7_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDEPOUT7` writer - Write '1' to disable interrupt for event ENDEPOUT\\[7\\]"]
pub struct ENDEPOUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPOUT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPOUT7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDEPOUT7_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event ENDISOOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDISOOUT_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDISOOUT_A> for bool {
    #[inline(always)]
    fn from(variant: ENDISOOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDISOOUT` reader - Write '1' to disable interrupt for event ENDISOOUT"]
pub struct ENDISOOUT_R(crate::FieldReader<bool, ENDISOOUT_A>);
impl ENDISOOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENDISOOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDISOOUT_A {
        match self.bits {
            false => ENDISOOUT_A::DISABLED,
            true => ENDISOOUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDISOOUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDISOOUT_A::ENABLED
    }
}
impl core::ops::Deref for ENDISOOUT_R {
    type Target = crate::FieldReader<bool, ENDISOOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event ENDISOOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDISOOUT_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDISOOUT_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDISOOUT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDISOOUT` writer - Write '1' to disable interrupt for event ENDISOOUT"]
pub struct ENDISOOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDISOOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDISOOUT_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDISOOUT_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event SOF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<SOF_A> for bool {
    #[inline(always)]
    fn from(variant: SOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF` reader - Write '1' to disable interrupt for event SOF"]
pub struct SOF_R(crate::FieldReader<bool, SOF_A>);
impl SOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOF_A {
        match self.bits {
            false => SOF_A::DISABLED,
            true => SOF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SOF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SOF_A::ENABLED
    }
}
impl core::ops::Deref for SOF_R {
    type Target = crate::FieldReader<bool, SOF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event SOF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<SOF_AW> for bool {
    #[inline(always)]
    fn from(variant: SOF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF` writer - Write '1' to disable interrupt for event SOF"]
pub struct SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SOF_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event USBEVENT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBEVENT_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<USBEVENT_A> for bool {
    #[inline(always)]
    fn from(variant: USBEVENT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBEVENT` reader - Write '1' to disable interrupt for event USBEVENT"]
pub struct USBEVENT_R(crate::FieldReader<bool, USBEVENT_A>);
impl USBEVENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBEVENT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBEVENT_A {
        match self.bits {
            false => USBEVENT_A::DISABLED,
            true => USBEVENT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == USBEVENT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == USBEVENT_A::ENABLED
    }
}
impl core::ops::Deref for USBEVENT_R {
    type Target = crate::FieldReader<bool, USBEVENT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event USBEVENT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBEVENT_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<USBEVENT_AW> for bool {
    #[inline(always)]
    fn from(variant: USBEVENT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBEVENT` writer - Write '1' to disable interrupt for event USBEVENT"]
pub struct USBEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> USBEVENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBEVENT_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(USBEVENT_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event EP0SETUP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0SETUP_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<EP0SETUP_A> for bool {
    #[inline(always)]
    fn from(variant: EP0SETUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0SETUP` reader - Write '1' to disable interrupt for event EP0SETUP"]
pub struct EP0SETUP_R(crate::FieldReader<bool, EP0SETUP_A>);
impl EP0SETUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0SETUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP0SETUP_A {
        match self.bits {
            false => EP0SETUP_A::DISABLED,
            true => EP0SETUP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EP0SETUP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EP0SETUP_A::ENABLED
    }
}
impl core::ops::Deref for EP0SETUP_R {
    type Target = crate::FieldReader<bool, EP0SETUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event EP0SETUP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0SETUP_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<EP0SETUP_AW> for bool {
    #[inline(always)]
    fn from(variant: EP0SETUP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0SETUP` writer - Write '1' to disable interrupt for event EP0SETUP"]
pub struct EP0SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0SETUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP0SETUP_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EP0SETUP_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event EPDATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPDATA_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<EPDATA_A> for bool {
    #[inline(always)]
    fn from(variant: EPDATA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPDATA` reader - Write '1' to disable interrupt for event EPDATA"]
pub struct EPDATA_R(crate::FieldReader<bool, EPDATA_A>);
impl EPDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPDATA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPDATA_A {
        match self.bits {
            false => EPDATA_A::DISABLED,
            true => EPDATA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EPDATA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EPDATA_A::ENABLED
    }
}
impl core::ops::Deref for EPDATA_R {
    type Target = crate::FieldReader<bool, EPDATA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event EPDATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPDATA_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<EPDATA_AW> for bool {
    #[inline(always)]
    fn from(variant: EPDATA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPDATA` writer - Write '1' to disable interrupt for event EPDATA"]
pub struct EPDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPDATA_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EPDATA_AW::CLEAR)
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
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event USBRESET"]
    #[inline(always)]
    pub fn usbreset(&self) -> USBRESET_R {
        USBRESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event STARTED"]
    #[inline(always)]
    pub fn started(&self) -> STARTED_R {
        STARTED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event ENDEPIN\\[0\\]"]
    #[inline(always)]
    pub fn endepin0(&self) -> ENDEPIN0_R {
        ENDEPIN0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event ENDEPIN\\[1\\]"]
    #[inline(always)]
    pub fn endepin1(&self) -> ENDEPIN1_R {
        ENDEPIN1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event ENDEPIN\\[2\\]"]
    #[inline(always)]
    pub fn endepin2(&self) -> ENDEPIN2_R {
        ENDEPIN2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event ENDEPIN\\[3\\]"]
    #[inline(always)]
    pub fn endepin3(&self) -> ENDEPIN3_R {
        ENDEPIN3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event ENDEPIN\\[4\\]"]
    #[inline(always)]
    pub fn endepin4(&self) -> ENDEPIN4_R {
        ENDEPIN4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event ENDEPIN\\[5\\]"]
    #[inline(always)]
    pub fn endepin5(&self) -> ENDEPIN5_R {
        ENDEPIN5_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write '1' to disable interrupt for event ENDEPIN\\[6\\]"]
    #[inline(always)]
    pub fn endepin6(&self) -> ENDEPIN6_R {
        ENDEPIN6_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write '1' to disable interrupt for event ENDEPIN\\[7\\]"]
    #[inline(always)]
    pub fn endepin7(&self) -> ENDEPIN7_R {
        ENDEPIN7_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Write '1' to disable interrupt for event EP0DATADONE"]
    #[inline(always)]
    pub fn ep0datadone(&self) -> EP0DATADONE_R {
        EP0DATADONE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Write '1' to disable interrupt for event ENDISOIN"]
    #[inline(always)]
    pub fn endisoin(&self) -> ENDISOIN_R {
        ENDISOIN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Write '1' to disable interrupt for event ENDEPOUT\\[0\\]"]
    #[inline(always)]
    pub fn endepout0(&self) -> ENDEPOUT0_R {
        ENDEPOUT0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Write '1' to disable interrupt for event ENDEPOUT\\[1\\]"]
    #[inline(always)]
    pub fn endepout1(&self) -> ENDEPOUT1_R {
        ENDEPOUT1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Write '1' to disable interrupt for event ENDEPOUT\\[2\\]"]
    #[inline(always)]
    pub fn endepout2(&self) -> ENDEPOUT2_R {
        ENDEPOUT2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Write '1' to disable interrupt for event ENDEPOUT\\[3\\]"]
    #[inline(always)]
    pub fn endepout3(&self) -> ENDEPOUT3_R {
        ENDEPOUT3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Write '1' to disable interrupt for event ENDEPOUT\\[4\\]"]
    #[inline(always)]
    pub fn endepout4(&self) -> ENDEPOUT4_R {
        ENDEPOUT4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Write '1' to disable interrupt for event ENDEPOUT\\[5\\]"]
    #[inline(always)]
    pub fn endepout5(&self) -> ENDEPOUT5_R {
        ENDEPOUT5_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event ENDEPOUT\\[6\\]"]
    #[inline(always)]
    pub fn endepout6(&self) -> ENDEPOUT6_R {
        ENDEPOUT6_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event ENDEPOUT\\[7\\]"]
    #[inline(always)]
    pub fn endepout7(&self) -> ENDEPOUT7_R {
        ENDEPOUT7_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for event ENDISOOUT"]
    #[inline(always)]
    pub fn endisoout(&self) -> ENDISOOUT_R {
        ENDISOOUT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Write '1' to disable interrupt for event SOF"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Write '1' to disable interrupt for event USBEVENT"]
    #[inline(always)]
    pub fn usbevent(&self) -> USBEVENT_R {
        USBEVENT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Write '1' to disable interrupt for event EP0SETUP"]
    #[inline(always)]
    pub fn ep0setup(&self) -> EP0SETUP_R {
        EP0SETUP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Write '1' to disable interrupt for event EPDATA"]
    #[inline(always)]
    pub fn epdata(&self) -> EPDATA_R {
        EPDATA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event USBRESET"]
    #[inline(always)]
    pub fn usbreset(&mut self) -> USBRESET_W {
        USBRESET_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event STARTED"]
    #[inline(always)]
    pub fn started(&mut self) -> STARTED_W {
        STARTED_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event ENDEPIN\\[0\\]"]
    #[inline(always)]
    pub fn endepin0(&mut self) -> ENDEPIN0_W {
        ENDEPIN0_W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event ENDEPIN\\[1\\]"]
    #[inline(always)]
    pub fn endepin1(&mut self) -> ENDEPIN1_W {
        ENDEPIN1_W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event ENDEPIN\\[2\\]"]
    #[inline(always)]
    pub fn endepin2(&mut self) -> ENDEPIN2_W {
        ENDEPIN2_W { w: self }
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event ENDEPIN\\[3\\]"]
    #[inline(always)]
    pub fn endepin3(&mut self) -> ENDEPIN3_W {
        ENDEPIN3_W { w: self }
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event ENDEPIN\\[4\\]"]
    #[inline(always)]
    pub fn endepin4(&mut self) -> ENDEPIN4_W {
        ENDEPIN4_W { w: self }
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event ENDEPIN\\[5\\]"]
    #[inline(always)]
    pub fn endepin5(&mut self) -> ENDEPIN5_W {
        ENDEPIN5_W { w: self }
    }
    #[doc = "Bit 8 - Write '1' to disable interrupt for event ENDEPIN\\[6\\]"]
    #[inline(always)]
    pub fn endepin6(&mut self) -> ENDEPIN6_W {
        ENDEPIN6_W { w: self }
    }
    #[doc = "Bit 9 - Write '1' to disable interrupt for event ENDEPIN\\[7\\]"]
    #[inline(always)]
    pub fn endepin7(&mut self) -> ENDEPIN7_W {
        ENDEPIN7_W { w: self }
    }
    #[doc = "Bit 10 - Write '1' to disable interrupt for event EP0DATADONE"]
    #[inline(always)]
    pub fn ep0datadone(&mut self) -> EP0DATADONE_W {
        EP0DATADONE_W { w: self }
    }
    #[doc = "Bit 11 - Write '1' to disable interrupt for event ENDISOIN"]
    #[inline(always)]
    pub fn endisoin(&mut self) -> ENDISOIN_W {
        ENDISOIN_W { w: self }
    }
    #[doc = "Bit 12 - Write '1' to disable interrupt for event ENDEPOUT\\[0\\]"]
    #[inline(always)]
    pub fn endepout0(&mut self) -> ENDEPOUT0_W {
        ENDEPOUT0_W { w: self }
    }
    #[doc = "Bit 13 - Write '1' to disable interrupt for event ENDEPOUT\\[1\\]"]
    #[inline(always)]
    pub fn endepout1(&mut self) -> ENDEPOUT1_W {
        ENDEPOUT1_W { w: self }
    }
    #[doc = "Bit 14 - Write '1' to disable interrupt for event ENDEPOUT\\[2\\]"]
    #[inline(always)]
    pub fn endepout2(&mut self) -> ENDEPOUT2_W {
        ENDEPOUT2_W { w: self }
    }
    #[doc = "Bit 15 - Write '1' to disable interrupt for event ENDEPOUT\\[3\\]"]
    #[inline(always)]
    pub fn endepout3(&mut self) -> ENDEPOUT3_W {
        ENDEPOUT3_W { w: self }
    }
    #[doc = "Bit 16 - Write '1' to disable interrupt for event ENDEPOUT\\[4\\]"]
    #[inline(always)]
    pub fn endepout4(&mut self) -> ENDEPOUT4_W {
        ENDEPOUT4_W { w: self }
    }
    #[doc = "Bit 17 - Write '1' to disable interrupt for event ENDEPOUT\\[5\\]"]
    #[inline(always)]
    pub fn endepout5(&mut self) -> ENDEPOUT5_W {
        ENDEPOUT5_W { w: self }
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event ENDEPOUT\\[6\\]"]
    #[inline(always)]
    pub fn endepout6(&mut self) -> ENDEPOUT6_W {
        ENDEPOUT6_W { w: self }
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event ENDEPOUT\\[7\\]"]
    #[inline(always)]
    pub fn endepout7(&mut self) -> ENDEPOUT7_W {
        ENDEPOUT7_W { w: self }
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for event ENDISOOUT"]
    #[inline(always)]
    pub fn endisoout(&mut self) -> ENDISOOUT_W {
        ENDISOOUT_W { w: self }
    }
    #[doc = "Bit 21 - Write '1' to disable interrupt for event SOF"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W { w: self }
    }
    #[doc = "Bit 22 - Write '1' to disable interrupt for event USBEVENT"]
    #[inline(always)]
    pub fn usbevent(&mut self) -> USBEVENT_W {
        USBEVENT_W { w: self }
    }
    #[doc = "Bit 23 - Write '1' to disable interrupt for event EP0SETUP"]
    #[inline(always)]
    pub fn ep0setup(&mut self) -> EP0SETUP_W {
        EP0SETUP_W { w: self }
    }
    #[doc = "Bit 24 - Write '1' to disable interrupt for event EPDATA"]
    #[inline(always)]
    pub fn epdata(&mut self) -> EPDATA_W {
        EPDATA_W { w: self }
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
