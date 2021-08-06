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
#[doc = "Write '1' to disable interrupt for event RECEIVE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE0_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RECEIVE0_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE0` reader - Write '1' to disable interrupt for event RECEIVE\\[0\\]"]
pub struct RECEIVE0_R(crate::FieldReader<bool, RECEIVE0_A>);
impl RECEIVE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RECEIVE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RECEIVE0_A::ENABLED
    }
}
impl core::ops::Deref for RECEIVE0_R {
    type Target = crate::FieldReader<bool, RECEIVE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event RECEIVE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE0_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RECEIVE0_AW> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE0` writer - Write '1' to disable interrupt for event RECEIVE\\[0\\]"]
pub struct RECEIVE0_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RECEIVE0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RECEIVE0_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event RECEIVE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE1_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RECEIVE1_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE1` reader - Write '1' to disable interrupt for event RECEIVE\\[1\\]"]
pub struct RECEIVE1_R(crate::FieldReader<bool, RECEIVE1_A>);
impl RECEIVE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RECEIVE1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RECEIVE1_A::ENABLED
    }
}
impl core::ops::Deref for RECEIVE1_R {
    type Target = crate::FieldReader<bool, RECEIVE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event RECEIVE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE1_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RECEIVE1_AW> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE1` writer - Write '1' to disable interrupt for event RECEIVE\\[1\\]"]
pub struct RECEIVE1_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RECEIVE1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RECEIVE1_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event RECEIVE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE2_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RECEIVE2_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE2` reader - Write '1' to disable interrupt for event RECEIVE\\[2\\]"]
pub struct RECEIVE2_R(crate::FieldReader<bool, RECEIVE2_A>);
impl RECEIVE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RECEIVE2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RECEIVE2_A::ENABLED
    }
}
impl core::ops::Deref for RECEIVE2_R {
    type Target = crate::FieldReader<bool, RECEIVE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event RECEIVE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE2_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RECEIVE2_AW> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE2` writer - Write '1' to disable interrupt for event RECEIVE\\[2\\]"]
pub struct RECEIVE2_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RECEIVE2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RECEIVE2_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event RECEIVE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE3_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RECEIVE3_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE3` reader - Write '1' to disable interrupt for event RECEIVE\\[3\\]"]
pub struct RECEIVE3_R(crate::FieldReader<bool, RECEIVE3_A>);
impl RECEIVE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RECEIVE3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RECEIVE3_A::ENABLED
    }
}
impl core::ops::Deref for RECEIVE3_R {
    type Target = crate::FieldReader<bool, RECEIVE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event RECEIVE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE3_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RECEIVE3_AW> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE3` writer - Write '1' to disable interrupt for event RECEIVE\\[3\\]"]
pub struct RECEIVE3_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RECEIVE3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RECEIVE3_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event RECEIVE\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE4_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RECEIVE4_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE4` reader - Write '1' to disable interrupt for event RECEIVE\\[4\\]"]
pub struct RECEIVE4_R(crate::FieldReader<bool, RECEIVE4_A>);
impl RECEIVE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RECEIVE4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RECEIVE4_A::ENABLED
    }
}
impl core::ops::Deref for RECEIVE4_R {
    type Target = crate::FieldReader<bool, RECEIVE4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event RECEIVE\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE4_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RECEIVE4_AW> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE4` writer - Write '1' to disable interrupt for event RECEIVE\\[4\\]"]
pub struct RECEIVE4_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RECEIVE4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RECEIVE4_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event RECEIVE\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE5_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RECEIVE5_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE5` reader - Write '1' to disable interrupt for event RECEIVE\\[5\\]"]
pub struct RECEIVE5_R(crate::FieldReader<bool, RECEIVE5_A>);
impl RECEIVE5_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RECEIVE5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RECEIVE5_A::ENABLED
    }
}
impl core::ops::Deref for RECEIVE5_R {
    type Target = crate::FieldReader<bool, RECEIVE5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event RECEIVE\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE5_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RECEIVE5_AW> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE5` writer - Write '1' to disable interrupt for event RECEIVE\\[5\\]"]
pub struct RECEIVE5_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RECEIVE5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RECEIVE5_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event RECEIVE\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE6_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RECEIVE6_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE6` reader - Write '1' to disable interrupt for event RECEIVE\\[6\\]"]
pub struct RECEIVE6_R(crate::FieldReader<bool, RECEIVE6_A>);
impl RECEIVE6_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RECEIVE6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RECEIVE6_A::ENABLED
    }
}
impl core::ops::Deref for RECEIVE6_R {
    type Target = crate::FieldReader<bool, RECEIVE6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event RECEIVE\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE6_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RECEIVE6_AW> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE6` writer - Write '1' to disable interrupt for event RECEIVE\\[6\\]"]
pub struct RECEIVE6_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RECEIVE6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RECEIVE6_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event RECEIVE\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE7_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RECEIVE7_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE7` reader - Write '1' to disable interrupt for event RECEIVE\\[7\\]"]
pub struct RECEIVE7_R(crate::FieldReader<bool, RECEIVE7_A>);
impl RECEIVE7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECEIVE7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RECEIVE7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RECEIVE7_A::ENABLED
    }
}
impl core::ops::Deref for RECEIVE7_R {
    type Target = crate::FieldReader<bool, RECEIVE7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event RECEIVE\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE7_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RECEIVE7_AW> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE7` writer - Write '1' to disable interrupt for event RECEIVE\\[7\\]"]
pub struct RECEIVE7_W<'a> {
    w: &'a mut W,
}
impl<'a> RECEIVE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RECEIVE7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RECEIVE7_AW::CLEAR)
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
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event RECEIVE\\[0\\]"]
    #[inline(always)]
    pub fn receive0(&self) -> RECEIVE0_R {
        RECEIVE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event RECEIVE\\[1\\]"]
    #[inline(always)]
    pub fn receive1(&self) -> RECEIVE1_R {
        RECEIVE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event RECEIVE\\[2\\]"]
    #[inline(always)]
    pub fn receive2(&self) -> RECEIVE2_R {
        RECEIVE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event RECEIVE\\[3\\]"]
    #[inline(always)]
    pub fn receive3(&self) -> RECEIVE3_R {
        RECEIVE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event RECEIVE\\[4\\]"]
    #[inline(always)]
    pub fn receive4(&self) -> RECEIVE4_R {
        RECEIVE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event RECEIVE\\[5\\]"]
    #[inline(always)]
    pub fn receive5(&self) -> RECEIVE5_R {
        RECEIVE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event RECEIVE\\[6\\]"]
    #[inline(always)]
    pub fn receive6(&self) -> RECEIVE6_R {
        RECEIVE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event RECEIVE\\[7\\]"]
    #[inline(always)]
    pub fn receive7(&self) -> RECEIVE7_R {
        RECEIVE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event RECEIVE\\[0\\]"]
    #[inline(always)]
    pub fn receive0(&mut self) -> RECEIVE0_W {
        RECEIVE0_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event RECEIVE\\[1\\]"]
    #[inline(always)]
    pub fn receive1(&mut self) -> RECEIVE1_W {
        RECEIVE1_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event RECEIVE\\[2\\]"]
    #[inline(always)]
    pub fn receive2(&mut self) -> RECEIVE2_W {
        RECEIVE2_W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event RECEIVE\\[3\\]"]
    #[inline(always)]
    pub fn receive3(&mut self) -> RECEIVE3_W {
        RECEIVE3_W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event RECEIVE\\[4\\]"]
    #[inline(always)]
    pub fn receive4(&mut self) -> RECEIVE4_W {
        RECEIVE4_W { w: self }
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event RECEIVE\\[5\\]"]
    #[inline(always)]
    pub fn receive5(&mut self) -> RECEIVE5_W {
        RECEIVE5_W { w: self }
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event RECEIVE\\[6\\]"]
    #[inline(always)]
    pub fn receive6(&mut self) -> RECEIVE6_W {
        RECEIVE6_W { w: self }
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event RECEIVE\\[7\\]"]
    #[inline(always)]
    pub fn receive7(&mut self) -> RECEIVE7_W {
        RECEIVE7_W { w: self }
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
