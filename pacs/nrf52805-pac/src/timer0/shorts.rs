#[doc = "Register `SHORTS` reader"]
pub struct R(crate::R<SHORTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHORTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHORTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHORTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHORTS` writer"]
pub struct W(crate::W<SHORTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHORTS_SPEC>;
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
impl From<crate::W<SHORTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHORTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Shortcut between event COMPARE\\[0\\]
and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE0_CLEAR_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE0_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE0_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE0_CLEAR` reader - Shortcut between event COMPARE\\[0\\]
and task CLEAR"]
pub struct COMPARE0_CLEAR_R(crate::FieldReader<bool, COMPARE0_CLEAR_A>);
impl COMPARE0_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE0_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE0_CLEAR_A {
        match self.bits {
            false => COMPARE0_CLEAR_A::DISABLED,
            true => COMPARE0_CLEAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE0_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE0_CLEAR_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE0_CLEAR_R {
    type Target = crate::FieldReader<bool, COMPARE0_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE0_CLEAR` writer - Shortcut between event COMPARE\\[0\\]
and task CLEAR"]
pub struct COMPARE0_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE0_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE0_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE0_CLEAR_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE0_CLEAR_A::ENABLED)
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
#[doc = "Shortcut between event COMPARE\\[1\\]
and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE1_CLEAR_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE1_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE1_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE1_CLEAR` reader - Shortcut between event COMPARE\\[1\\]
and task CLEAR"]
pub struct COMPARE1_CLEAR_R(crate::FieldReader<bool, COMPARE1_CLEAR_A>);
impl COMPARE1_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE1_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE1_CLEAR_A {
        match self.bits {
            false => COMPARE1_CLEAR_A::DISABLED,
            true => COMPARE1_CLEAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE1_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE1_CLEAR_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE1_CLEAR_R {
    type Target = crate::FieldReader<bool, COMPARE1_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE1_CLEAR` writer - Shortcut between event COMPARE\\[1\\]
and task CLEAR"]
pub struct COMPARE1_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE1_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE1_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE1_CLEAR_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE1_CLEAR_A::ENABLED)
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
#[doc = "Shortcut between event COMPARE\\[2\\]
and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE2_CLEAR_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE2_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE2_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE2_CLEAR` reader - Shortcut between event COMPARE\\[2\\]
and task CLEAR"]
pub struct COMPARE2_CLEAR_R(crate::FieldReader<bool, COMPARE2_CLEAR_A>);
impl COMPARE2_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE2_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE2_CLEAR_A {
        match self.bits {
            false => COMPARE2_CLEAR_A::DISABLED,
            true => COMPARE2_CLEAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE2_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE2_CLEAR_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE2_CLEAR_R {
    type Target = crate::FieldReader<bool, COMPARE2_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE2_CLEAR` writer - Shortcut between event COMPARE\\[2\\]
and task CLEAR"]
pub struct COMPARE2_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE2_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE2_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE2_CLEAR_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE2_CLEAR_A::ENABLED)
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
#[doc = "Shortcut between event COMPARE\\[3\\]
and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE3_CLEAR_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE3_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE3_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE3_CLEAR` reader - Shortcut between event COMPARE\\[3\\]
and task CLEAR"]
pub struct COMPARE3_CLEAR_R(crate::FieldReader<bool, COMPARE3_CLEAR_A>);
impl COMPARE3_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE3_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE3_CLEAR_A {
        match self.bits {
            false => COMPARE3_CLEAR_A::DISABLED,
            true => COMPARE3_CLEAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE3_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE3_CLEAR_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE3_CLEAR_R {
    type Target = crate::FieldReader<bool, COMPARE3_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE3_CLEAR` writer - Shortcut between event COMPARE\\[3\\]
and task CLEAR"]
pub struct COMPARE3_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE3_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE3_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE3_CLEAR_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE3_CLEAR_A::ENABLED)
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
#[doc = "Shortcut between event COMPARE\\[4\\]
and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE4_CLEAR_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE4_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE4_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE4_CLEAR` reader - Shortcut between event COMPARE\\[4\\]
and task CLEAR"]
pub struct COMPARE4_CLEAR_R(crate::FieldReader<bool, COMPARE4_CLEAR_A>);
impl COMPARE4_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE4_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE4_CLEAR_A {
        match self.bits {
            false => COMPARE4_CLEAR_A::DISABLED,
            true => COMPARE4_CLEAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE4_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE4_CLEAR_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE4_CLEAR_R {
    type Target = crate::FieldReader<bool, COMPARE4_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE4_CLEAR` writer - Shortcut between event COMPARE\\[4\\]
and task CLEAR"]
pub struct COMPARE4_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE4_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE4_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE4_CLEAR_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE4_CLEAR_A::ENABLED)
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
#[doc = "Shortcut between event COMPARE\\[5\\]
and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE5_CLEAR_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE5_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE5_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE5_CLEAR` reader - Shortcut between event COMPARE\\[5\\]
and task CLEAR"]
pub struct COMPARE5_CLEAR_R(crate::FieldReader<bool, COMPARE5_CLEAR_A>);
impl COMPARE5_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE5_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE5_CLEAR_A {
        match self.bits {
            false => COMPARE5_CLEAR_A::DISABLED,
            true => COMPARE5_CLEAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE5_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE5_CLEAR_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE5_CLEAR_R {
    type Target = crate::FieldReader<bool, COMPARE5_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE5_CLEAR` writer - Shortcut between event COMPARE\\[5\\]
and task CLEAR"]
pub struct COMPARE5_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE5_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE5_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE5_CLEAR_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE5_CLEAR_A::ENABLED)
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
#[doc = "Shortcut between event COMPARE\\[0\\]
and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE0_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE0_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE0_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE0_STOP` reader - Shortcut between event COMPARE\\[0\\]
and task STOP"]
pub struct COMPARE0_STOP_R(crate::FieldReader<bool, COMPARE0_STOP_A>);
impl COMPARE0_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE0_STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE0_STOP_A {
        match self.bits {
            false => COMPARE0_STOP_A::DISABLED,
            true => COMPARE0_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE0_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE0_STOP_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE0_STOP_R {
    type Target = crate::FieldReader<bool, COMPARE0_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE0_STOP` writer - Shortcut between event COMPARE\\[0\\]
and task STOP"]
pub struct COMPARE0_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE0_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE0_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE0_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE0_STOP_A::ENABLED)
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
#[doc = "Shortcut between event COMPARE\\[1\\]
and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE1_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE1_STOP` reader - Shortcut between event COMPARE\\[1\\]
and task STOP"]
pub struct COMPARE1_STOP_R(crate::FieldReader<bool, COMPARE1_STOP_A>);
impl COMPARE1_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE1_STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE1_STOP_A {
        match self.bits {
            false => COMPARE1_STOP_A::DISABLED,
            true => COMPARE1_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE1_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE1_STOP_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE1_STOP_R {
    type Target = crate::FieldReader<bool, COMPARE1_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE1_STOP` writer - Shortcut between event COMPARE\\[1\\]
and task STOP"]
pub struct COMPARE1_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE1_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE1_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE1_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE1_STOP_A::ENABLED)
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
#[doc = "Shortcut between event COMPARE\\[2\\]
and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE2_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE2_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE2_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE2_STOP` reader - Shortcut between event COMPARE\\[2\\]
and task STOP"]
pub struct COMPARE2_STOP_R(crate::FieldReader<bool, COMPARE2_STOP_A>);
impl COMPARE2_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE2_STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE2_STOP_A {
        match self.bits {
            false => COMPARE2_STOP_A::DISABLED,
            true => COMPARE2_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE2_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE2_STOP_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE2_STOP_R {
    type Target = crate::FieldReader<bool, COMPARE2_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE2_STOP` writer - Shortcut between event COMPARE\\[2\\]
and task STOP"]
pub struct COMPARE2_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE2_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE2_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE2_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE2_STOP_A::ENABLED)
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
#[doc = "Shortcut between event COMPARE\\[3\\]
and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE3_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE3_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE3_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE3_STOP` reader - Shortcut between event COMPARE\\[3\\]
and task STOP"]
pub struct COMPARE3_STOP_R(crate::FieldReader<bool, COMPARE3_STOP_A>);
impl COMPARE3_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE3_STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE3_STOP_A {
        match self.bits {
            false => COMPARE3_STOP_A::DISABLED,
            true => COMPARE3_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE3_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE3_STOP_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE3_STOP_R {
    type Target = crate::FieldReader<bool, COMPARE3_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE3_STOP` writer - Shortcut between event COMPARE\\[3\\]
and task STOP"]
pub struct COMPARE3_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE3_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE3_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE3_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE3_STOP_A::ENABLED)
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
#[doc = "Shortcut between event COMPARE\\[4\\]
and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE4_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE4_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE4_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE4_STOP` reader - Shortcut between event COMPARE\\[4\\]
and task STOP"]
pub struct COMPARE4_STOP_R(crate::FieldReader<bool, COMPARE4_STOP_A>);
impl COMPARE4_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE4_STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE4_STOP_A {
        match self.bits {
            false => COMPARE4_STOP_A::DISABLED,
            true => COMPARE4_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE4_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE4_STOP_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE4_STOP_R {
    type Target = crate::FieldReader<bool, COMPARE4_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE4_STOP` writer - Shortcut between event COMPARE\\[4\\]
and task STOP"]
pub struct COMPARE4_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE4_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE4_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE4_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE4_STOP_A::ENABLED)
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
#[doc = "Shortcut between event COMPARE\\[5\\]
and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE5_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE5_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE5_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE5_STOP` reader - Shortcut between event COMPARE\\[5\\]
and task STOP"]
pub struct COMPARE5_STOP_R(crate::FieldReader<bool, COMPARE5_STOP_A>);
impl COMPARE5_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE5_STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE5_STOP_A {
        match self.bits {
            false => COMPARE5_STOP_A::DISABLED,
            true => COMPARE5_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE5_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE5_STOP_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE5_STOP_R {
    type Target = crate::FieldReader<bool, COMPARE5_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE5_STOP` writer - Shortcut between event COMPARE\\[5\\]
and task STOP"]
pub struct COMPARE5_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE5_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE5_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE5_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE5_STOP_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Shortcut between event COMPARE\\[0\\]
and task CLEAR"]
    #[inline(always)]
    pub fn compare0_clear(&self) -> COMPARE0_CLEAR_R {
        COMPARE0_CLEAR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event COMPARE\\[1\\]
and task CLEAR"]
    #[inline(always)]
    pub fn compare1_clear(&self) -> COMPARE1_CLEAR_R {
        COMPARE1_CLEAR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Shortcut between event COMPARE\\[2\\]
and task CLEAR"]
    #[inline(always)]
    pub fn compare2_clear(&self) -> COMPARE2_CLEAR_R {
        COMPARE2_CLEAR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Shortcut between event COMPARE\\[3\\]
and task CLEAR"]
    #[inline(always)]
    pub fn compare3_clear(&self) -> COMPARE3_CLEAR_R {
        COMPARE3_CLEAR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Shortcut between event COMPARE\\[4\\]
and task CLEAR"]
    #[inline(always)]
    pub fn compare4_clear(&self) -> COMPARE4_CLEAR_R {
        COMPARE4_CLEAR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Shortcut between event COMPARE\\[5\\]
and task CLEAR"]
    #[inline(always)]
    pub fn compare5_clear(&self) -> COMPARE5_CLEAR_R {
        COMPARE5_CLEAR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Shortcut between event COMPARE\\[0\\]
and task STOP"]
    #[inline(always)]
    pub fn compare0_stop(&self) -> COMPARE0_STOP_R {
        COMPARE0_STOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Shortcut between event COMPARE\\[1\\]
and task STOP"]
    #[inline(always)]
    pub fn compare1_stop(&self) -> COMPARE1_STOP_R {
        COMPARE1_STOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Shortcut between event COMPARE\\[2\\]
and task STOP"]
    #[inline(always)]
    pub fn compare2_stop(&self) -> COMPARE2_STOP_R {
        COMPARE2_STOP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Shortcut between event COMPARE\\[3\\]
and task STOP"]
    #[inline(always)]
    pub fn compare3_stop(&self) -> COMPARE3_STOP_R {
        COMPARE3_STOP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Shortcut between event COMPARE\\[4\\]
and task STOP"]
    #[inline(always)]
    pub fn compare4_stop(&self) -> COMPARE4_STOP_R {
        COMPARE4_STOP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Shortcut between event COMPARE\\[5\\]
and task STOP"]
    #[inline(always)]
    pub fn compare5_stop(&self) -> COMPARE5_STOP_R {
        COMPARE5_STOP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event COMPARE\\[0\\]
and task CLEAR"]
    #[inline(always)]
    pub fn compare0_clear(&mut self) -> COMPARE0_CLEAR_W {
        COMPARE0_CLEAR_W { w: self }
    }
    #[doc = "Bit 1 - Shortcut between event COMPARE\\[1\\]
and task CLEAR"]
    #[inline(always)]
    pub fn compare1_clear(&mut self) -> COMPARE1_CLEAR_W {
        COMPARE1_CLEAR_W { w: self }
    }
    #[doc = "Bit 2 - Shortcut between event COMPARE\\[2\\]
and task CLEAR"]
    #[inline(always)]
    pub fn compare2_clear(&mut self) -> COMPARE2_CLEAR_W {
        COMPARE2_CLEAR_W { w: self }
    }
    #[doc = "Bit 3 - Shortcut between event COMPARE\\[3\\]
and task CLEAR"]
    #[inline(always)]
    pub fn compare3_clear(&mut self) -> COMPARE3_CLEAR_W {
        COMPARE3_CLEAR_W { w: self }
    }
    #[doc = "Bit 4 - Shortcut between event COMPARE\\[4\\]
and task CLEAR"]
    #[inline(always)]
    pub fn compare4_clear(&mut self) -> COMPARE4_CLEAR_W {
        COMPARE4_CLEAR_W { w: self }
    }
    #[doc = "Bit 5 - Shortcut between event COMPARE\\[5\\]
and task CLEAR"]
    #[inline(always)]
    pub fn compare5_clear(&mut self) -> COMPARE5_CLEAR_W {
        COMPARE5_CLEAR_W { w: self }
    }
    #[doc = "Bit 8 - Shortcut between event COMPARE\\[0\\]
and task STOP"]
    #[inline(always)]
    pub fn compare0_stop(&mut self) -> COMPARE0_STOP_W {
        COMPARE0_STOP_W { w: self }
    }
    #[doc = "Bit 9 - Shortcut between event COMPARE\\[1\\]
and task STOP"]
    #[inline(always)]
    pub fn compare1_stop(&mut self) -> COMPARE1_STOP_W {
        COMPARE1_STOP_W { w: self }
    }
    #[doc = "Bit 10 - Shortcut between event COMPARE\\[2\\]
and task STOP"]
    #[inline(always)]
    pub fn compare2_stop(&mut self) -> COMPARE2_STOP_W {
        COMPARE2_STOP_W { w: self }
    }
    #[doc = "Bit 11 - Shortcut between event COMPARE\\[3\\]
and task STOP"]
    #[inline(always)]
    pub fn compare3_stop(&mut self) -> COMPARE3_STOP_W {
        COMPARE3_STOP_W { w: self }
    }
    #[doc = "Bit 12 - Shortcut between event COMPARE\\[4\\]
and task STOP"]
    #[inline(always)]
    pub fn compare4_stop(&mut self) -> COMPARE4_STOP_W {
        COMPARE4_STOP_W { w: self }
    }
    #[doc = "Bit 13 - Shortcut between event COMPARE\\[5\\]
and task STOP"]
    #[inline(always)]
    pub fn compare5_stop(&mut self) -> COMPARE5_STOP_W {
        COMPARE5_STOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shortcuts between local events and tasks\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](index.html) module"]
pub struct SHORTS_SPEC;
impl crate::RegisterSpec for SHORTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shorts::R](R) reader structure"]
impl crate::Readable for SHORTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shorts::W](W) writer structure"]
impl crate::Writable for SHORTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for SHORTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
