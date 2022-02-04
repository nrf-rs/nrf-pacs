#[doc = "Register `CONFIG3` reader"]
pub struct R(crate::R<CONFIG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG3` writer"]
pub struct W(crate::W<CONFIG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG3_SPEC>;
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
impl From<crate::W<CONFIG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable protection for region 96. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION96_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION96_A> for bool {
    #[inline(always)]
    fn from(variant: REGION96_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION96` reader - Enable protection for region 96. Write '0' has no effect."]
pub struct REGION96_R(crate::FieldReader<bool, REGION96_A>);
impl REGION96_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION96_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION96_A {
        match self.bits {
            false => REGION96_A::DISABLED,
            true => REGION96_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION96_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION96_A::ENABLED
    }
}
impl core::ops::Deref for REGION96_R {
    type Target = crate::FieldReader<bool, REGION96_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION96` writer - Enable protection for region 96. Write '0' has no effect."]
pub struct REGION96_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION96_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION96_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION96_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION96_A::ENABLED)
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
#[doc = "Enable protection for region 97. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION97_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION97_A> for bool {
    #[inline(always)]
    fn from(variant: REGION97_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION97` reader - Enable protection for region 97. Write '0' has no effect."]
pub struct REGION97_R(crate::FieldReader<bool, REGION97_A>);
impl REGION97_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION97_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION97_A {
        match self.bits {
            false => REGION97_A::DISABLED,
            true => REGION97_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION97_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION97_A::ENABLED
    }
}
impl core::ops::Deref for REGION97_R {
    type Target = crate::FieldReader<bool, REGION97_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION97` writer - Enable protection for region 97. Write '0' has no effect."]
pub struct REGION97_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION97_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION97_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION97_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION97_A::ENABLED)
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
#[doc = "Enable protection for region 98. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION98_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION98_A> for bool {
    #[inline(always)]
    fn from(variant: REGION98_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION98` reader - Enable protection for region 98. Write '0' has no effect."]
pub struct REGION98_R(crate::FieldReader<bool, REGION98_A>);
impl REGION98_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION98_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION98_A {
        match self.bits {
            false => REGION98_A::DISABLED,
            true => REGION98_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION98_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION98_A::ENABLED
    }
}
impl core::ops::Deref for REGION98_R {
    type Target = crate::FieldReader<bool, REGION98_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION98` writer - Enable protection for region 98. Write '0' has no effect."]
pub struct REGION98_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION98_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION98_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION98_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION98_A::ENABLED)
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
#[doc = "Enable protection for region 99. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION99_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION99_A> for bool {
    #[inline(always)]
    fn from(variant: REGION99_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION99` reader - Enable protection for region 99. Write '0' has no effect."]
pub struct REGION99_R(crate::FieldReader<bool, REGION99_A>);
impl REGION99_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION99_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION99_A {
        match self.bits {
            false => REGION99_A::DISABLED,
            true => REGION99_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION99_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION99_A::ENABLED
    }
}
impl core::ops::Deref for REGION99_R {
    type Target = crate::FieldReader<bool, REGION99_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION99` writer - Enable protection for region 99. Write '0' has no effect."]
pub struct REGION99_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION99_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION99_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION99_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION99_A::ENABLED)
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
#[doc = "Enable protection for region 100. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION100_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION100_A> for bool {
    #[inline(always)]
    fn from(variant: REGION100_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION100` reader - Enable protection for region 100. Write '0' has no effect."]
pub struct REGION100_R(crate::FieldReader<bool, REGION100_A>);
impl REGION100_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION100_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION100_A {
        match self.bits {
            false => REGION100_A::DISABLED,
            true => REGION100_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION100_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION100_A::ENABLED
    }
}
impl core::ops::Deref for REGION100_R {
    type Target = crate::FieldReader<bool, REGION100_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION100` writer - Enable protection for region 100. Write '0' has no effect."]
pub struct REGION100_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION100_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION100_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION100_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION100_A::ENABLED)
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
#[doc = "Enable protection for region 101. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION101_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION101_A> for bool {
    #[inline(always)]
    fn from(variant: REGION101_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION101` reader - Enable protection for region 101. Write '0' has no effect."]
pub struct REGION101_R(crate::FieldReader<bool, REGION101_A>);
impl REGION101_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION101_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION101_A {
        match self.bits {
            false => REGION101_A::DISABLED,
            true => REGION101_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION101_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION101_A::ENABLED
    }
}
impl core::ops::Deref for REGION101_R {
    type Target = crate::FieldReader<bool, REGION101_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION101` writer - Enable protection for region 101. Write '0' has no effect."]
pub struct REGION101_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION101_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION101_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION101_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION101_A::ENABLED)
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
#[doc = "Enable protection for region 102. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION102_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION102_A> for bool {
    #[inline(always)]
    fn from(variant: REGION102_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION102` reader - Enable protection for region 102. Write '0' has no effect."]
pub struct REGION102_R(crate::FieldReader<bool, REGION102_A>);
impl REGION102_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION102_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION102_A {
        match self.bits {
            false => REGION102_A::DISABLED,
            true => REGION102_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION102_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION102_A::ENABLED
    }
}
impl core::ops::Deref for REGION102_R {
    type Target = crate::FieldReader<bool, REGION102_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION102` writer - Enable protection for region 102. Write '0' has no effect."]
pub struct REGION102_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION102_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION102_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION102_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION102_A::ENABLED)
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
#[doc = "Enable protection for region 103. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION103_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION103_A> for bool {
    #[inline(always)]
    fn from(variant: REGION103_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION103` reader - Enable protection for region 103. Write '0' has no effect."]
pub struct REGION103_R(crate::FieldReader<bool, REGION103_A>);
impl REGION103_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION103_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION103_A {
        match self.bits {
            false => REGION103_A::DISABLED,
            true => REGION103_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION103_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION103_A::ENABLED
    }
}
impl core::ops::Deref for REGION103_R {
    type Target = crate::FieldReader<bool, REGION103_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION103` writer - Enable protection for region 103. Write '0' has no effect."]
pub struct REGION103_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION103_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION103_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION103_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION103_A::ENABLED)
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
#[doc = "Enable protection for region 104. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION104_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION104_A> for bool {
    #[inline(always)]
    fn from(variant: REGION104_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION104` reader - Enable protection for region 104. Write '0' has no effect."]
pub struct REGION104_R(crate::FieldReader<bool, REGION104_A>);
impl REGION104_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION104_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION104_A {
        match self.bits {
            false => REGION104_A::DISABLED,
            true => REGION104_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION104_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION104_A::ENABLED
    }
}
impl core::ops::Deref for REGION104_R {
    type Target = crate::FieldReader<bool, REGION104_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION104` writer - Enable protection for region 104. Write '0' has no effect."]
pub struct REGION104_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION104_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION104_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION104_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION104_A::ENABLED)
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
#[doc = "Enable protection for region 105. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION105_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION105_A> for bool {
    #[inline(always)]
    fn from(variant: REGION105_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION105` reader - Enable protection for region 105. Write '0' has no effect."]
pub struct REGION105_R(crate::FieldReader<bool, REGION105_A>);
impl REGION105_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION105_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION105_A {
        match self.bits {
            false => REGION105_A::DISABLED,
            true => REGION105_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION105_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION105_A::ENABLED
    }
}
impl core::ops::Deref for REGION105_R {
    type Target = crate::FieldReader<bool, REGION105_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION105` writer - Enable protection for region 105. Write '0' has no effect."]
pub struct REGION105_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION105_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION105_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION105_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION105_A::ENABLED)
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
#[doc = "Enable protection for region 106. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION106_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION106_A> for bool {
    #[inline(always)]
    fn from(variant: REGION106_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION106` reader - Enable protection for region 106. Write '0' has no effect."]
pub struct REGION106_R(crate::FieldReader<bool, REGION106_A>);
impl REGION106_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION106_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION106_A {
        match self.bits {
            false => REGION106_A::DISABLED,
            true => REGION106_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION106_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION106_A::ENABLED
    }
}
impl core::ops::Deref for REGION106_R {
    type Target = crate::FieldReader<bool, REGION106_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION106` writer - Enable protection for region 106. Write '0' has no effect."]
pub struct REGION106_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION106_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION106_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION106_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION106_A::ENABLED)
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
#[doc = "Enable protection for region 107. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION107_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION107_A> for bool {
    #[inline(always)]
    fn from(variant: REGION107_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION107` reader - Enable protection for region 107. Write '0' has no effect."]
pub struct REGION107_R(crate::FieldReader<bool, REGION107_A>);
impl REGION107_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION107_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION107_A {
        match self.bits {
            false => REGION107_A::DISABLED,
            true => REGION107_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION107_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION107_A::ENABLED
    }
}
impl core::ops::Deref for REGION107_R {
    type Target = crate::FieldReader<bool, REGION107_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION107` writer - Enable protection for region 107. Write '0' has no effect."]
pub struct REGION107_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION107_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION107_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION107_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION107_A::ENABLED)
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
#[doc = "Enable protection for region 108. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION108_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION108_A> for bool {
    #[inline(always)]
    fn from(variant: REGION108_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION108` reader - Enable protection for region 108. Write '0' has no effect."]
pub struct REGION108_R(crate::FieldReader<bool, REGION108_A>);
impl REGION108_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION108_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION108_A {
        match self.bits {
            false => REGION108_A::DISABLED,
            true => REGION108_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION108_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION108_A::ENABLED
    }
}
impl core::ops::Deref for REGION108_R {
    type Target = crate::FieldReader<bool, REGION108_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION108` writer - Enable protection for region 108. Write '0' has no effect."]
pub struct REGION108_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION108_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION108_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION108_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION108_A::ENABLED)
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
#[doc = "Enable protection for region 109. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION109_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION109_A> for bool {
    #[inline(always)]
    fn from(variant: REGION109_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION109` reader - Enable protection for region 109. Write '0' has no effect."]
pub struct REGION109_R(crate::FieldReader<bool, REGION109_A>);
impl REGION109_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION109_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION109_A {
        match self.bits {
            false => REGION109_A::DISABLED,
            true => REGION109_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION109_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION109_A::ENABLED
    }
}
impl core::ops::Deref for REGION109_R {
    type Target = crate::FieldReader<bool, REGION109_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION109` writer - Enable protection for region 109. Write '0' has no effect."]
pub struct REGION109_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION109_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION109_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION109_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION109_A::ENABLED)
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
#[doc = "Enable protection for region 110. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION110_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION110_A> for bool {
    #[inline(always)]
    fn from(variant: REGION110_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION110` reader - Enable protection for region 110. Write '0' has no effect."]
pub struct REGION110_R(crate::FieldReader<bool, REGION110_A>);
impl REGION110_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION110_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION110_A {
        match self.bits {
            false => REGION110_A::DISABLED,
            true => REGION110_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION110_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION110_A::ENABLED
    }
}
impl core::ops::Deref for REGION110_R {
    type Target = crate::FieldReader<bool, REGION110_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION110` writer - Enable protection for region 110. Write '0' has no effect."]
pub struct REGION110_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION110_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION110_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION110_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION110_A::ENABLED)
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
#[doc = "Enable protection for region 111. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION111_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION111_A> for bool {
    #[inline(always)]
    fn from(variant: REGION111_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION111` reader - Enable protection for region 111. Write '0' has no effect."]
pub struct REGION111_R(crate::FieldReader<bool, REGION111_A>);
impl REGION111_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION111_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION111_A {
        match self.bits {
            false => REGION111_A::DISABLED,
            true => REGION111_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION111_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION111_A::ENABLED
    }
}
impl core::ops::Deref for REGION111_R {
    type Target = crate::FieldReader<bool, REGION111_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION111` writer - Enable protection for region 111. Write '0' has no effect."]
pub struct REGION111_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION111_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION111_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION111_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION111_A::ENABLED)
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
#[doc = "Enable protection for region 112. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION112_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION112_A> for bool {
    #[inline(always)]
    fn from(variant: REGION112_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION112` reader - Enable protection for region 112. Write '0' has no effect."]
pub struct REGION112_R(crate::FieldReader<bool, REGION112_A>);
impl REGION112_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION112_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION112_A {
        match self.bits {
            false => REGION112_A::DISABLED,
            true => REGION112_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION112_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION112_A::ENABLED
    }
}
impl core::ops::Deref for REGION112_R {
    type Target = crate::FieldReader<bool, REGION112_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION112` writer - Enable protection for region 112. Write '0' has no effect."]
pub struct REGION112_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION112_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION112_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION112_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION112_A::ENABLED)
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
#[doc = "Enable protection for region 113. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION113_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION113_A> for bool {
    #[inline(always)]
    fn from(variant: REGION113_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION113` reader - Enable protection for region 113. Write '0' has no effect."]
pub struct REGION113_R(crate::FieldReader<bool, REGION113_A>);
impl REGION113_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION113_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION113_A {
        match self.bits {
            false => REGION113_A::DISABLED,
            true => REGION113_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION113_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION113_A::ENABLED
    }
}
impl core::ops::Deref for REGION113_R {
    type Target = crate::FieldReader<bool, REGION113_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION113` writer - Enable protection for region 113. Write '0' has no effect."]
pub struct REGION113_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION113_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION113_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION113_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION113_A::ENABLED)
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
#[doc = "Enable protection for region 114. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION114_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION114_A> for bool {
    #[inline(always)]
    fn from(variant: REGION114_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION114` reader - Enable protection for region 114. Write '0' has no effect."]
pub struct REGION114_R(crate::FieldReader<bool, REGION114_A>);
impl REGION114_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION114_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION114_A {
        match self.bits {
            false => REGION114_A::DISABLED,
            true => REGION114_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION114_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION114_A::ENABLED
    }
}
impl core::ops::Deref for REGION114_R {
    type Target = crate::FieldReader<bool, REGION114_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION114` writer - Enable protection for region 114. Write '0' has no effect."]
pub struct REGION114_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION114_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION114_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION114_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION114_A::ENABLED)
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
#[doc = "Enable protection for region 115. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION115_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION115_A> for bool {
    #[inline(always)]
    fn from(variant: REGION115_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION115` reader - Enable protection for region 115. Write '0' has no effect."]
pub struct REGION115_R(crate::FieldReader<bool, REGION115_A>);
impl REGION115_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION115_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION115_A {
        match self.bits {
            false => REGION115_A::DISABLED,
            true => REGION115_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION115_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION115_A::ENABLED
    }
}
impl core::ops::Deref for REGION115_R {
    type Target = crate::FieldReader<bool, REGION115_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION115` writer - Enable protection for region 115. Write '0' has no effect."]
pub struct REGION115_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION115_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION115_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION115_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION115_A::ENABLED)
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
#[doc = "Enable protection for region 116. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION116_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION116_A> for bool {
    #[inline(always)]
    fn from(variant: REGION116_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION116` reader - Enable protection for region 116. Write '0' has no effect."]
pub struct REGION116_R(crate::FieldReader<bool, REGION116_A>);
impl REGION116_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION116_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION116_A {
        match self.bits {
            false => REGION116_A::DISABLED,
            true => REGION116_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION116_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION116_A::ENABLED
    }
}
impl core::ops::Deref for REGION116_R {
    type Target = crate::FieldReader<bool, REGION116_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION116` writer - Enable protection for region 116. Write '0' has no effect."]
pub struct REGION116_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION116_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION116_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION116_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION116_A::ENABLED)
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
#[doc = "Enable protection for region 117. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION117_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION117_A> for bool {
    #[inline(always)]
    fn from(variant: REGION117_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION117` reader - Enable protection for region 117. Write '0' has no effect."]
pub struct REGION117_R(crate::FieldReader<bool, REGION117_A>);
impl REGION117_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION117_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION117_A {
        match self.bits {
            false => REGION117_A::DISABLED,
            true => REGION117_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION117_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION117_A::ENABLED
    }
}
impl core::ops::Deref for REGION117_R {
    type Target = crate::FieldReader<bool, REGION117_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION117` writer - Enable protection for region 117. Write '0' has no effect."]
pub struct REGION117_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION117_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION117_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION117_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION117_A::ENABLED)
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
#[doc = "Enable protection for region 118. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION118_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION118_A> for bool {
    #[inline(always)]
    fn from(variant: REGION118_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION118` reader - Enable protection for region 118. Write '0' has no effect."]
pub struct REGION118_R(crate::FieldReader<bool, REGION118_A>);
impl REGION118_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION118_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION118_A {
        match self.bits {
            false => REGION118_A::DISABLED,
            true => REGION118_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION118_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION118_A::ENABLED
    }
}
impl core::ops::Deref for REGION118_R {
    type Target = crate::FieldReader<bool, REGION118_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION118` writer - Enable protection for region 118. Write '0' has no effect."]
pub struct REGION118_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION118_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION118_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION118_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION118_A::ENABLED)
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
#[doc = "Enable protection for region 119. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION119_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION119_A> for bool {
    #[inline(always)]
    fn from(variant: REGION119_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION119` reader - Enable protection for region 119. Write '0' has no effect."]
pub struct REGION119_R(crate::FieldReader<bool, REGION119_A>);
impl REGION119_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION119_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION119_A {
        match self.bits {
            false => REGION119_A::DISABLED,
            true => REGION119_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION119_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION119_A::ENABLED
    }
}
impl core::ops::Deref for REGION119_R {
    type Target = crate::FieldReader<bool, REGION119_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION119` writer - Enable protection for region 119. Write '0' has no effect."]
pub struct REGION119_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION119_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION119_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION119_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION119_A::ENABLED)
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
#[doc = "Enable protection for region 120. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION120_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION120_A> for bool {
    #[inline(always)]
    fn from(variant: REGION120_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION120` reader - Enable protection for region 120. Write '0' has no effect."]
pub struct REGION120_R(crate::FieldReader<bool, REGION120_A>);
impl REGION120_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION120_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION120_A {
        match self.bits {
            false => REGION120_A::DISABLED,
            true => REGION120_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION120_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION120_A::ENABLED
    }
}
impl core::ops::Deref for REGION120_R {
    type Target = crate::FieldReader<bool, REGION120_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION120` writer - Enable protection for region 120. Write '0' has no effect."]
pub struct REGION120_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION120_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION120_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION120_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION120_A::ENABLED)
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
#[doc = "Enable protection for region 121. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION121_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION121_A> for bool {
    #[inline(always)]
    fn from(variant: REGION121_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION121` reader - Enable protection for region 121. Write '0' has no effect."]
pub struct REGION121_R(crate::FieldReader<bool, REGION121_A>);
impl REGION121_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION121_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION121_A {
        match self.bits {
            false => REGION121_A::DISABLED,
            true => REGION121_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION121_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION121_A::ENABLED
    }
}
impl core::ops::Deref for REGION121_R {
    type Target = crate::FieldReader<bool, REGION121_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION121` writer - Enable protection for region 121. Write '0' has no effect."]
pub struct REGION121_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION121_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION121_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION121_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION121_A::ENABLED)
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
#[doc = "Enable protection for region 122. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION122_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION122_A> for bool {
    #[inline(always)]
    fn from(variant: REGION122_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION122` reader - Enable protection for region 122. Write '0' has no effect."]
pub struct REGION122_R(crate::FieldReader<bool, REGION122_A>);
impl REGION122_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION122_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION122_A {
        match self.bits {
            false => REGION122_A::DISABLED,
            true => REGION122_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION122_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION122_A::ENABLED
    }
}
impl core::ops::Deref for REGION122_R {
    type Target = crate::FieldReader<bool, REGION122_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION122` writer - Enable protection for region 122. Write '0' has no effect."]
pub struct REGION122_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION122_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION122_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION122_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION122_A::ENABLED)
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
#[doc = "Enable protection for region 123. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION123_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION123_A> for bool {
    #[inline(always)]
    fn from(variant: REGION123_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION123` reader - Enable protection for region 123. Write '0' has no effect."]
pub struct REGION123_R(crate::FieldReader<bool, REGION123_A>);
impl REGION123_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION123_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION123_A {
        match self.bits {
            false => REGION123_A::DISABLED,
            true => REGION123_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION123_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION123_A::ENABLED
    }
}
impl core::ops::Deref for REGION123_R {
    type Target = crate::FieldReader<bool, REGION123_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION123` writer - Enable protection for region 123. Write '0' has no effect."]
pub struct REGION123_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION123_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION123_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION123_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION123_A::ENABLED)
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
#[doc = "Enable protection for region 124. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION124_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION124_A> for bool {
    #[inline(always)]
    fn from(variant: REGION124_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION124` reader - Enable protection for region 124. Write '0' has no effect."]
pub struct REGION124_R(crate::FieldReader<bool, REGION124_A>);
impl REGION124_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION124_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION124_A {
        match self.bits {
            false => REGION124_A::DISABLED,
            true => REGION124_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION124_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION124_A::ENABLED
    }
}
impl core::ops::Deref for REGION124_R {
    type Target = crate::FieldReader<bool, REGION124_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION124` writer - Enable protection for region 124. Write '0' has no effect."]
pub struct REGION124_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION124_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION124_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION124_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION124_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Enable protection for region 125. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION125_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION125_A> for bool {
    #[inline(always)]
    fn from(variant: REGION125_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION125` reader - Enable protection for region 125. Write '0' has no effect."]
pub struct REGION125_R(crate::FieldReader<bool, REGION125_A>);
impl REGION125_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION125_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION125_A {
        match self.bits {
            false => REGION125_A::DISABLED,
            true => REGION125_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION125_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION125_A::ENABLED
    }
}
impl core::ops::Deref for REGION125_R {
    type Target = crate::FieldReader<bool, REGION125_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION125` writer - Enable protection for region 125. Write '0' has no effect."]
pub struct REGION125_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION125_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION125_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION125_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION125_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Enable protection for region 126. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION126_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION126_A> for bool {
    #[inline(always)]
    fn from(variant: REGION126_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION126` reader - Enable protection for region 126. Write '0' has no effect."]
pub struct REGION126_R(crate::FieldReader<bool, REGION126_A>);
impl REGION126_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION126_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION126_A {
        match self.bits {
            false => REGION126_A::DISABLED,
            true => REGION126_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION126_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION126_A::ENABLED
    }
}
impl core::ops::Deref for REGION126_R {
    type Target = crate::FieldReader<bool, REGION126_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION126` writer - Enable protection for region 126. Write '0' has no effect."]
pub struct REGION126_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION126_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION126_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION126_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION126_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Enable protection for region 127. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION127_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION127_A> for bool {
    #[inline(always)]
    fn from(variant: REGION127_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION127` reader - Enable protection for region 127. Write '0' has no effect."]
pub struct REGION127_R(crate::FieldReader<bool, REGION127_A>);
impl REGION127_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGION127_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION127_A {
        match self.bits {
            false => REGION127_A::DISABLED,
            true => REGION127_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REGION127_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REGION127_A::ENABLED
    }
}
impl core::ops::Deref for REGION127_R {
    type Target = crate::FieldReader<bool, REGION127_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION127` writer - Enable protection for region 127. Write '0' has no effect."]
pub struct REGION127_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION127_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION127_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION127_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION127_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable protection for region 96. Write '0' has no effect."]
    #[inline(always)]
    pub fn region96(&self) -> REGION96_R {
        REGION96_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable protection for region 97. Write '0' has no effect."]
    #[inline(always)]
    pub fn region97(&self) -> REGION97_R {
        REGION97_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable protection for region 98. Write '0' has no effect."]
    #[inline(always)]
    pub fn region98(&self) -> REGION98_R {
        REGION98_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable protection for region 99. Write '0' has no effect."]
    #[inline(always)]
    pub fn region99(&self) -> REGION99_R {
        REGION99_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable protection for region 100. Write '0' has no effect."]
    #[inline(always)]
    pub fn region100(&self) -> REGION100_R {
        REGION100_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable protection for region 101. Write '0' has no effect."]
    #[inline(always)]
    pub fn region101(&self) -> REGION101_R {
        REGION101_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable protection for region 102. Write '0' has no effect."]
    #[inline(always)]
    pub fn region102(&self) -> REGION102_R {
        REGION102_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable protection for region 103. Write '0' has no effect."]
    #[inline(always)]
    pub fn region103(&self) -> REGION103_R {
        REGION103_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable protection for region 104. Write '0' has no effect."]
    #[inline(always)]
    pub fn region104(&self) -> REGION104_R {
        REGION104_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable protection for region 105. Write '0' has no effect."]
    #[inline(always)]
    pub fn region105(&self) -> REGION105_R {
        REGION105_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable protection for region 106. Write '0' has no effect."]
    #[inline(always)]
    pub fn region106(&self) -> REGION106_R {
        REGION106_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable protection for region 107. Write '0' has no effect."]
    #[inline(always)]
    pub fn region107(&self) -> REGION107_R {
        REGION107_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable protection for region 108. Write '0' has no effect."]
    #[inline(always)]
    pub fn region108(&self) -> REGION108_R {
        REGION108_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable protection for region 109. Write '0' has no effect."]
    #[inline(always)]
    pub fn region109(&self) -> REGION109_R {
        REGION109_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable protection for region 110. Write '0' has no effect."]
    #[inline(always)]
    pub fn region110(&self) -> REGION110_R {
        REGION110_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable protection for region 111. Write '0' has no effect."]
    #[inline(always)]
    pub fn region111(&self) -> REGION111_R {
        REGION111_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable protection for region 112. Write '0' has no effect."]
    #[inline(always)]
    pub fn region112(&self) -> REGION112_R {
        REGION112_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable protection for region 113. Write '0' has no effect."]
    #[inline(always)]
    pub fn region113(&self) -> REGION113_R {
        REGION113_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable protection for region 114. Write '0' has no effect."]
    #[inline(always)]
    pub fn region114(&self) -> REGION114_R {
        REGION114_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable protection for region 115. Write '0' has no effect."]
    #[inline(always)]
    pub fn region115(&self) -> REGION115_R {
        REGION115_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable protection for region 116. Write '0' has no effect."]
    #[inline(always)]
    pub fn region116(&self) -> REGION116_R {
        REGION116_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable protection for region 117. Write '0' has no effect."]
    #[inline(always)]
    pub fn region117(&self) -> REGION117_R {
        REGION117_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable protection for region 118. Write '0' has no effect."]
    #[inline(always)]
    pub fn region118(&self) -> REGION118_R {
        REGION118_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable protection for region 119. Write '0' has no effect."]
    #[inline(always)]
    pub fn region119(&self) -> REGION119_R {
        REGION119_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable protection for region 120. Write '0' has no effect."]
    #[inline(always)]
    pub fn region120(&self) -> REGION120_R {
        REGION120_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable protection for region 121. Write '0' has no effect."]
    #[inline(always)]
    pub fn region121(&self) -> REGION121_R {
        REGION121_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable protection for region 122. Write '0' has no effect."]
    #[inline(always)]
    pub fn region122(&self) -> REGION122_R {
        REGION122_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable protection for region 123. Write '0' has no effect."]
    #[inline(always)]
    pub fn region123(&self) -> REGION123_R {
        REGION123_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable protection for region 124. Write '0' has no effect."]
    #[inline(always)]
    pub fn region124(&self) -> REGION124_R {
        REGION124_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable protection for region 125. Write '0' has no effect."]
    #[inline(always)]
    pub fn region125(&self) -> REGION125_R {
        REGION125_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable protection for region 126. Write '0' has no effect."]
    #[inline(always)]
    pub fn region126(&self) -> REGION126_R {
        REGION126_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable protection for region 127. Write '0' has no effect."]
    #[inline(always)]
    pub fn region127(&self) -> REGION127_R {
        REGION127_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable protection for region 96. Write '0' has no effect."]
    #[inline(always)]
    pub fn region96(&mut self) -> REGION96_W {
        REGION96_W { w: self }
    }
    #[doc = "Bit 1 - Enable protection for region 97. Write '0' has no effect."]
    #[inline(always)]
    pub fn region97(&mut self) -> REGION97_W {
        REGION97_W { w: self }
    }
    #[doc = "Bit 2 - Enable protection for region 98. Write '0' has no effect."]
    #[inline(always)]
    pub fn region98(&mut self) -> REGION98_W {
        REGION98_W { w: self }
    }
    #[doc = "Bit 3 - Enable protection for region 99. Write '0' has no effect."]
    #[inline(always)]
    pub fn region99(&mut self) -> REGION99_W {
        REGION99_W { w: self }
    }
    #[doc = "Bit 4 - Enable protection for region 100. Write '0' has no effect."]
    #[inline(always)]
    pub fn region100(&mut self) -> REGION100_W {
        REGION100_W { w: self }
    }
    #[doc = "Bit 5 - Enable protection for region 101. Write '0' has no effect."]
    #[inline(always)]
    pub fn region101(&mut self) -> REGION101_W {
        REGION101_W { w: self }
    }
    #[doc = "Bit 6 - Enable protection for region 102. Write '0' has no effect."]
    #[inline(always)]
    pub fn region102(&mut self) -> REGION102_W {
        REGION102_W { w: self }
    }
    #[doc = "Bit 7 - Enable protection for region 103. Write '0' has no effect."]
    #[inline(always)]
    pub fn region103(&mut self) -> REGION103_W {
        REGION103_W { w: self }
    }
    #[doc = "Bit 8 - Enable protection for region 104. Write '0' has no effect."]
    #[inline(always)]
    pub fn region104(&mut self) -> REGION104_W {
        REGION104_W { w: self }
    }
    #[doc = "Bit 9 - Enable protection for region 105. Write '0' has no effect."]
    #[inline(always)]
    pub fn region105(&mut self) -> REGION105_W {
        REGION105_W { w: self }
    }
    #[doc = "Bit 10 - Enable protection for region 106. Write '0' has no effect."]
    #[inline(always)]
    pub fn region106(&mut self) -> REGION106_W {
        REGION106_W { w: self }
    }
    #[doc = "Bit 11 - Enable protection for region 107. Write '0' has no effect."]
    #[inline(always)]
    pub fn region107(&mut self) -> REGION107_W {
        REGION107_W { w: self }
    }
    #[doc = "Bit 12 - Enable protection for region 108. Write '0' has no effect."]
    #[inline(always)]
    pub fn region108(&mut self) -> REGION108_W {
        REGION108_W { w: self }
    }
    #[doc = "Bit 13 - Enable protection for region 109. Write '0' has no effect."]
    #[inline(always)]
    pub fn region109(&mut self) -> REGION109_W {
        REGION109_W { w: self }
    }
    #[doc = "Bit 14 - Enable protection for region 110. Write '0' has no effect."]
    #[inline(always)]
    pub fn region110(&mut self) -> REGION110_W {
        REGION110_W { w: self }
    }
    #[doc = "Bit 15 - Enable protection for region 111. Write '0' has no effect."]
    #[inline(always)]
    pub fn region111(&mut self) -> REGION111_W {
        REGION111_W { w: self }
    }
    #[doc = "Bit 16 - Enable protection for region 112. Write '0' has no effect."]
    #[inline(always)]
    pub fn region112(&mut self) -> REGION112_W {
        REGION112_W { w: self }
    }
    #[doc = "Bit 17 - Enable protection for region 113. Write '0' has no effect."]
    #[inline(always)]
    pub fn region113(&mut self) -> REGION113_W {
        REGION113_W { w: self }
    }
    #[doc = "Bit 18 - Enable protection for region 114. Write '0' has no effect."]
    #[inline(always)]
    pub fn region114(&mut self) -> REGION114_W {
        REGION114_W { w: self }
    }
    #[doc = "Bit 19 - Enable protection for region 115. Write '0' has no effect."]
    #[inline(always)]
    pub fn region115(&mut self) -> REGION115_W {
        REGION115_W { w: self }
    }
    #[doc = "Bit 20 - Enable protection for region 116. Write '0' has no effect."]
    #[inline(always)]
    pub fn region116(&mut self) -> REGION116_W {
        REGION116_W { w: self }
    }
    #[doc = "Bit 21 - Enable protection for region 117. Write '0' has no effect."]
    #[inline(always)]
    pub fn region117(&mut self) -> REGION117_W {
        REGION117_W { w: self }
    }
    #[doc = "Bit 22 - Enable protection for region 118. Write '0' has no effect."]
    #[inline(always)]
    pub fn region118(&mut self) -> REGION118_W {
        REGION118_W { w: self }
    }
    #[doc = "Bit 23 - Enable protection for region 119. Write '0' has no effect."]
    #[inline(always)]
    pub fn region119(&mut self) -> REGION119_W {
        REGION119_W { w: self }
    }
    #[doc = "Bit 24 - Enable protection for region 120. Write '0' has no effect."]
    #[inline(always)]
    pub fn region120(&mut self) -> REGION120_W {
        REGION120_W { w: self }
    }
    #[doc = "Bit 25 - Enable protection for region 121. Write '0' has no effect."]
    #[inline(always)]
    pub fn region121(&mut self) -> REGION121_W {
        REGION121_W { w: self }
    }
    #[doc = "Bit 26 - Enable protection for region 122. Write '0' has no effect."]
    #[inline(always)]
    pub fn region122(&mut self) -> REGION122_W {
        REGION122_W { w: self }
    }
    #[doc = "Bit 27 - Enable protection for region 123. Write '0' has no effect."]
    #[inline(always)]
    pub fn region123(&mut self) -> REGION123_W {
        REGION123_W { w: self }
    }
    #[doc = "Bit 28 - Enable protection for region 124. Write '0' has no effect."]
    #[inline(always)]
    pub fn region124(&mut self) -> REGION124_W {
        REGION124_W { w: self }
    }
    #[doc = "Bit 29 - Enable protection for region 125. Write '0' has no effect."]
    #[inline(always)]
    pub fn region125(&mut self) -> REGION125_W {
        REGION125_W { w: self }
    }
    #[doc = "Bit 30 - Enable protection for region 126. Write '0' has no effect."]
    #[inline(always)]
    pub fn region126(&mut self) -> REGION126_W {
        REGION126_W { w: self }
    }
    #[doc = "Bit 31 - Enable protection for region 127. Write '0' has no effect."]
    #[inline(always)]
    pub fn region127(&mut self) -> REGION127_W {
        REGION127_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block protect configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config3](index.html) module"]
pub struct CONFIG3_SPEC;
impl crate::RegisterSpec for CONFIG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config3::R](R) reader structure"]
impl crate::Readable for CONFIG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config3::W](W) writer structure"]
impl crate::Writable for CONFIG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG3 to value 0"]
impl crate::Resettable for CONFIG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
