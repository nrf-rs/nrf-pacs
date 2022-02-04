#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable or disable interrupt for event TRIGGERED\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED0_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED0` reader - Enable or disable interrupt for event TRIGGERED\\[0\\]"]
pub struct TRIGGERED0_R(crate::FieldReader<bool, TRIGGERED0_A>);
impl TRIGGERED0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGERED0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED0_A {
        match self.bits {
            false => TRIGGERED0_A::DISABLED,
            true => TRIGGERED0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TRIGGERED0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TRIGGERED0_A::ENABLED
    }
}
impl core::ops::Deref for TRIGGERED0_R {
    type Target = crate::FieldReader<bool, TRIGGERED0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGERED0` writer - Enable or disable interrupt for event TRIGGERED\\[0\\]"]
pub struct TRIGGERED0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED0_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event TRIGGERED\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED1_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED1` reader - Enable or disable interrupt for event TRIGGERED\\[1\\]"]
pub struct TRIGGERED1_R(crate::FieldReader<bool, TRIGGERED1_A>);
impl TRIGGERED1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGERED1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED1_A {
        match self.bits {
            false => TRIGGERED1_A::DISABLED,
            true => TRIGGERED1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TRIGGERED1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TRIGGERED1_A::ENABLED
    }
}
impl core::ops::Deref for TRIGGERED1_R {
    type Target = crate::FieldReader<bool, TRIGGERED1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGERED1` writer - Enable or disable interrupt for event TRIGGERED\\[1\\]"]
pub struct TRIGGERED1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED1_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event TRIGGERED\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED2_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED2_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED2` reader - Enable or disable interrupt for event TRIGGERED\\[2\\]"]
pub struct TRIGGERED2_R(crate::FieldReader<bool, TRIGGERED2_A>);
impl TRIGGERED2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGERED2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED2_A {
        match self.bits {
            false => TRIGGERED2_A::DISABLED,
            true => TRIGGERED2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TRIGGERED2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TRIGGERED2_A::ENABLED
    }
}
impl core::ops::Deref for TRIGGERED2_R {
    type Target = crate::FieldReader<bool, TRIGGERED2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGERED2` writer - Enable or disable interrupt for event TRIGGERED\\[2\\]"]
pub struct TRIGGERED2_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED2_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED2_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event TRIGGERED\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED3_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED3_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED3` reader - Enable or disable interrupt for event TRIGGERED\\[3\\]"]
pub struct TRIGGERED3_R(crate::FieldReader<bool, TRIGGERED3_A>);
impl TRIGGERED3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGERED3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED3_A {
        match self.bits {
            false => TRIGGERED3_A::DISABLED,
            true => TRIGGERED3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TRIGGERED3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TRIGGERED3_A::ENABLED
    }
}
impl core::ops::Deref for TRIGGERED3_R {
    type Target = crate::FieldReader<bool, TRIGGERED3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGERED3` writer - Enable or disable interrupt for event TRIGGERED\\[3\\]"]
pub struct TRIGGERED3_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED3_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED3_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event TRIGGERED\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED4_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED4_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED4` reader - Enable or disable interrupt for event TRIGGERED\\[4\\]"]
pub struct TRIGGERED4_R(crate::FieldReader<bool, TRIGGERED4_A>);
impl TRIGGERED4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGERED4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED4_A {
        match self.bits {
            false => TRIGGERED4_A::DISABLED,
            true => TRIGGERED4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TRIGGERED4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TRIGGERED4_A::ENABLED
    }
}
impl core::ops::Deref for TRIGGERED4_R {
    type Target = crate::FieldReader<bool, TRIGGERED4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGERED4` writer - Enable or disable interrupt for event TRIGGERED\\[4\\]"]
pub struct TRIGGERED4_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED4_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED4_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event TRIGGERED\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED5_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED5_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED5` reader - Enable or disable interrupt for event TRIGGERED\\[5\\]"]
pub struct TRIGGERED5_R(crate::FieldReader<bool, TRIGGERED5_A>);
impl TRIGGERED5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGERED5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED5_A {
        match self.bits {
            false => TRIGGERED5_A::DISABLED,
            true => TRIGGERED5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TRIGGERED5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TRIGGERED5_A::ENABLED
    }
}
impl core::ops::Deref for TRIGGERED5_R {
    type Target = crate::FieldReader<bool, TRIGGERED5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGERED5` writer - Enable or disable interrupt for event TRIGGERED\\[5\\]"]
pub struct TRIGGERED5_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED5_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED5_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event TRIGGERED\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED6_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED6_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED6` reader - Enable or disable interrupt for event TRIGGERED\\[6\\]"]
pub struct TRIGGERED6_R(crate::FieldReader<bool, TRIGGERED6_A>);
impl TRIGGERED6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGERED6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED6_A {
        match self.bits {
            false => TRIGGERED6_A::DISABLED,
            true => TRIGGERED6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TRIGGERED6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TRIGGERED6_A::ENABLED
    }
}
impl core::ops::Deref for TRIGGERED6_R {
    type Target = crate::FieldReader<bool, TRIGGERED6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGERED6` writer - Enable or disable interrupt for event TRIGGERED\\[6\\]"]
pub struct TRIGGERED6_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED6_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED6_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event TRIGGERED\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED7_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED7_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED7` reader - Enable or disable interrupt for event TRIGGERED\\[7\\]"]
pub struct TRIGGERED7_R(crate::FieldReader<bool, TRIGGERED7_A>);
impl TRIGGERED7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGERED7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED7_A {
        match self.bits {
            false => TRIGGERED7_A::DISABLED,
            true => TRIGGERED7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TRIGGERED7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TRIGGERED7_A::ENABLED
    }
}
impl core::ops::Deref for TRIGGERED7_R {
    type Target = crate::FieldReader<bool, TRIGGERED7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGERED7` writer - Enable or disable interrupt for event TRIGGERED\\[7\\]"]
pub struct TRIGGERED7_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED7_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED7_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event TRIGGERED\\[8\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED8_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED8_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED8` reader - Enable or disable interrupt for event TRIGGERED\\[8\\]"]
pub struct TRIGGERED8_R(crate::FieldReader<bool, TRIGGERED8_A>);
impl TRIGGERED8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGERED8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED8_A {
        match self.bits {
            false => TRIGGERED8_A::DISABLED,
            true => TRIGGERED8_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TRIGGERED8_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TRIGGERED8_A::ENABLED
    }
}
impl core::ops::Deref for TRIGGERED8_R {
    type Target = crate::FieldReader<bool, TRIGGERED8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGERED8` writer - Enable or disable interrupt for event TRIGGERED\\[8\\]"]
pub struct TRIGGERED8_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED8_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED8_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event TRIGGERED\\[9\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED9_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED9_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED9` reader - Enable or disable interrupt for event TRIGGERED\\[9\\]"]
pub struct TRIGGERED9_R(crate::FieldReader<bool, TRIGGERED9_A>);
impl TRIGGERED9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGERED9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED9_A {
        match self.bits {
            false => TRIGGERED9_A::DISABLED,
            true => TRIGGERED9_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TRIGGERED9_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TRIGGERED9_A::ENABLED
    }
}
impl core::ops::Deref for TRIGGERED9_R {
    type Target = crate::FieldReader<bool, TRIGGERED9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGERED9` writer - Enable or disable interrupt for event TRIGGERED\\[9\\]"]
pub struct TRIGGERED9_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED9_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED9_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event TRIGGERED\\[10\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED10_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED10_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED10` reader - Enable or disable interrupt for event TRIGGERED\\[10\\]"]
pub struct TRIGGERED10_R(crate::FieldReader<bool, TRIGGERED10_A>);
impl TRIGGERED10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGERED10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED10_A {
        match self.bits {
            false => TRIGGERED10_A::DISABLED,
            true => TRIGGERED10_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TRIGGERED10_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TRIGGERED10_A::ENABLED
    }
}
impl core::ops::Deref for TRIGGERED10_R {
    type Target = crate::FieldReader<bool, TRIGGERED10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGERED10` writer - Enable or disable interrupt for event TRIGGERED\\[10\\]"]
pub struct TRIGGERED10_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED10_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED10_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event TRIGGERED\\[11\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED11_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED11_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED11` reader - Enable or disable interrupt for event TRIGGERED\\[11\\]"]
pub struct TRIGGERED11_R(crate::FieldReader<bool, TRIGGERED11_A>);
impl TRIGGERED11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGERED11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED11_A {
        match self.bits {
            false => TRIGGERED11_A::DISABLED,
            true => TRIGGERED11_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TRIGGERED11_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TRIGGERED11_A::ENABLED
    }
}
impl core::ops::Deref for TRIGGERED11_R {
    type Target = crate::FieldReader<bool, TRIGGERED11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGERED11` writer - Enable or disable interrupt for event TRIGGERED\\[11\\]"]
pub struct TRIGGERED11_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED11_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED11_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event TRIGGERED\\[12\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED12_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED12_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED12` reader - Enable or disable interrupt for event TRIGGERED\\[12\\]"]
pub struct TRIGGERED12_R(crate::FieldReader<bool, TRIGGERED12_A>);
impl TRIGGERED12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGERED12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED12_A {
        match self.bits {
            false => TRIGGERED12_A::DISABLED,
            true => TRIGGERED12_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TRIGGERED12_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TRIGGERED12_A::ENABLED
    }
}
impl core::ops::Deref for TRIGGERED12_R {
    type Target = crate::FieldReader<bool, TRIGGERED12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGERED12` writer - Enable or disable interrupt for event TRIGGERED\\[12\\]"]
pub struct TRIGGERED12_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED12_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED12_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event TRIGGERED\\[13\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED13_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED13_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED13` reader - Enable or disable interrupt for event TRIGGERED\\[13\\]"]
pub struct TRIGGERED13_R(crate::FieldReader<bool, TRIGGERED13_A>);
impl TRIGGERED13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGERED13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED13_A {
        match self.bits {
            false => TRIGGERED13_A::DISABLED,
            true => TRIGGERED13_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TRIGGERED13_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TRIGGERED13_A::ENABLED
    }
}
impl core::ops::Deref for TRIGGERED13_R {
    type Target = crate::FieldReader<bool, TRIGGERED13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGERED13` writer - Enable or disable interrupt for event TRIGGERED\\[13\\]"]
pub struct TRIGGERED13_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED13_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED13_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event TRIGGERED\\[14\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED14_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED14_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED14` reader - Enable or disable interrupt for event TRIGGERED\\[14\\]"]
pub struct TRIGGERED14_R(crate::FieldReader<bool, TRIGGERED14_A>);
impl TRIGGERED14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGERED14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED14_A {
        match self.bits {
            false => TRIGGERED14_A::DISABLED,
            true => TRIGGERED14_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TRIGGERED14_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TRIGGERED14_A::ENABLED
    }
}
impl core::ops::Deref for TRIGGERED14_R {
    type Target = crate::FieldReader<bool, TRIGGERED14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGERED14` writer - Enable or disable interrupt for event TRIGGERED\\[14\\]"]
pub struct TRIGGERED14_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED14_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED14_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event TRIGGERED\\[15\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED15_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED15_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED15` reader - Enable or disable interrupt for event TRIGGERED\\[15\\]"]
pub struct TRIGGERED15_R(crate::FieldReader<bool, TRIGGERED15_A>);
impl TRIGGERED15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIGGERED15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED15_A {
        match self.bits {
            false => TRIGGERED15_A::DISABLED,
            true => TRIGGERED15_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TRIGGERED15_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TRIGGERED15_A::ENABLED
    }
}
impl core::ops::Deref for TRIGGERED15_R {
    type Target = crate::FieldReader<bool, TRIGGERED15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGERED15` writer - Enable or disable interrupt for event TRIGGERED\\[15\\]"]
pub struct TRIGGERED15_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED15_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED15_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event TRIGGERED\\[0\\]"]
    #[inline(always)]
    pub fn triggered0(&self) -> TRIGGERED0_R {
        TRIGGERED0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event TRIGGERED\\[1\\]"]
    #[inline(always)]
    pub fn triggered1(&self) -> TRIGGERED1_R {
        TRIGGERED1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event TRIGGERED\\[2\\]"]
    #[inline(always)]
    pub fn triggered2(&self) -> TRIGGERED2_R {
        TRIGGERED2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event TRIGGERED\\[3\\]"]
    #[inline(always)]
    pub fn triggered3(&self) -> TRIGGERED3_R {
        TRIGGERED3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event TRIGGERED\\[4\\]"]
    #[inline(always)]
    pub fn triggered4(&self) -> TRIGGERED4_R {
        TRIGGERED4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event TRIGGERED\\[5\\]"]
    #[inline(always)]
    pub fn triggered5(&self) -> TRIGGERED5_R {
        TRIGGERED5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event TRIGGERED\\[6\\]"]
    #[inline(always)]
    pub fn triggered6(&self) -> TRIGGERED6_R {
        TRIGGERED6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event TRIGGERED\\[7\\]"]
    #[inline(always)]
    pub fn triggered7(&self) -> TRIGGERED7_R {
        TRIGGERED7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for event TRIGGERED\\[8\\]"]
    #[inline(always)]
    pub fn triggered8(&self) -> TRIGGERED8_R {
        TRIGGERED8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event TRIGGERED\\[9\\]"]
    #[inline(always)]
    pub fn triggered9(&self) -> TRIGGERED9_R {
        TRIGGERED9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for event TRIGGERED\\[10\\]"]
    #[inline(always)]
    pub fn triggered10(&self) -> TRIGGERED10_R {
        TRIGGERED10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable or disable interrupt for event TRIGGERED\\[11\\]"]
    #[inline(always)]
    pub fn triggered11(&self) -> TRIGGERED11_R {
        TRIGGERED11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable or disable interrupt for event TRIGGERED\\[12\\]"]
    #[inline(always)]
    pub fn triggered12(&self) -> TRIGGERED12_R {
        TRIGGERED12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for event TRIGGERED\\[13\\]"]
    #[inline(always)]
    pub fn triggered13(&self) -> TRIGGERED13_R {
        TRIGGERED13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for event TRIGGERED\\[14\\]"]
    #[inline(always)]
    pub fn triggered14(&self) -> TRIGGERED14_R {
        TRIGGERED14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable or disable interrupt for event TRIGGERED\\[15\\]"]
    #[inline(always)]
    pub fn triggered15(&self) -> TRIGGERED15_R {
        TRIGGERED15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event TRIGGERED\\[0\\]"]
    #[inline(always)]
    pub fn triggered0(&mut self) -> TRIGGERED0_W {
        TRIGGERED0_W { w: self }
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event TRIGGERED\\[1\\]"]
    #[inline(always)]
    pub fn triggered1(&mut self) -> TRIGGERED1_W {
        TRIGGERED1_W { w: self }
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event TRIGGERED\\[2\\]"]
    #[inline(always)]
    pub fn triggered2(&mut self) -> TRIGGERED2_W {
        TRIGGERED2_W { w: self }
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event TRIGGERED\\[3\\]"]
    #[inline(always)]
    pub fn triggered3(&mut self) -> TRIGGERED3_W {
        TRIGGERED3_W { w: self }
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event TRIGGERED\\[4\\]"]
    #[inline(always)]
    pub fn triggered4(&mut self) -> TRIGGERED4_W {
        TRIGGERED4_W { w: self }
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event TRIGGERED\\[5\\]"]
    #[inline(always)]
    pub fn triggered5(&mut self) -> TRIGGERED5_W {
        TRIGGERED5_W { w: self }
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event TRIGGERED\\[6\\]"]
    #[inline(always)]
    pub fn triggered6(&mut self) -> TRIGGERED6_W {
        TRIGGERED6_W { w: self }
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event TRIGGERED\\[7\\]"]
    #[inline(always)]
    pub fn triggered7(&mut self) -> TRIGGERED7_W {
        TRIGGERED7_W { w: self }
    }
    #[doc = "Bit 8 - Enable or disable interrupt for event TRIGGERED\\[8\\]"]
    #[inline(always)]
    pub fn triggered8(&mut self) -> TRIGGERED8_W {
        TRIGGERED8_W { w: self }
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event TRIGGERED\\[9\\]"]
    #[inline(always)]
    pub fn triggered9(&mut self) -> TRIGGERED9_W {
        TRIGGERED9_W { w: self }
    }
    #[doc = "Bit 10 - Enable or disable interrupt for event TRIGGERED\\[10\\]"]
    #[inline(always)]
    pub fn triggered10(&mut self) -> TRIGGERED10_W {
        TRIGGERED10_W { w: self }
    }
    #[doc = "Bit 11 - Enable or disable interrupt for event TRIGGERED\\[11\\]"]
    #[inline(always)]
    pub fn triggered11(&mut self) -> TRIGGERED11_W {
        TRIGGERED11_W { w: self }
    }
    #[doc = "Bit 12 - Enable or disable interrupt for event TRIGGERED\\[12\\]"]
    #[inline(always)]
    pub fn triggered12(&mut self) -> TRIGGERED12_W {
        TRIGGERED12_W { w: self }
    }
    #[doc = "Bit 13 - Enable or disable interrupt for event TRIGGERED\\[13\\]"]
    #[inline(always)]
    pub fn triggered13(&mut self) -> TRIGGERED13_W {
        TRIGGERED13_W { w: self }
    }
    #[doc = "Bit 14 - Enable or disable interrupt for event TRIGGERED\\[14\\]"]
    #[inline(always)]
    pub fn triggered14(&mut self) -> TRIGGERED14_W {
        TRIGGERED14_W { w: self }
    }
    #[doc = "Bit 15 - Enable or disable interrupt for event TRIGGERED\\[15\\]"]
    #[inline(always)]
    pub fn triggered15(&mut self) -> TRIGGERED15_W {
        TRIGGERED15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable or disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
