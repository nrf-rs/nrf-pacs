#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[0\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED0_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TRIGGERED0_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED0` reader - Write '1' to enable interrupt for TRIGGERED\\[0\\]
event"]
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[0\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED0_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TRIGGERED0_AW> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED0` writer - Write '1' to enable interrupt for TRIGGERED\\[0\\]
event"]
pub struct TRIGGERED0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED0_AW::SET)
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[1\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED1_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TRIGGERED1_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED1` reader - Write '1' to enable interrupt for TRIGGERED\\[1\\]
event"]
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[1\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED1_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TRIGGERED1_AW> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED1` writer - Write '1' to enable interrupt for TRIGGERED\\[1\\]
event"]
pub struct TRIGGERED1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED1_AW::SET)
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[2\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED2_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TRIGGERED2_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED2` reader - Write '1' to enable interrupt for TRIGGERED\\[2\\]
event"]
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[2\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED2_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TRIGGERED2_AW> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED2` writer - Write '1' to enable interrupt for TRIGGERED\\[2\\]
event"]
pub struct TRIGGERED2_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED2_AW::SET)
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[3\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED3_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TRIGGERED3_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED3` reader - Write '1' to enable interrupt for TRIGGERED\\[3\\]
event"]
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[3\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED3_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TRIGGERED3_AW> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED3` writer - Write '1' to enable interrupt for TRIGGERED\\[3\\]
event"]
pub struct TRIGGERED3_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED3_AW::SET)
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[4\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED4_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TRIGGERED4_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED4` reader - Write '1' to enable interrupt for TRIGGERED\\[4\\]
event"]
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[4\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED4_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TRIGGERED4_AW> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED4` writer - Write '1' to enable interrupt for TRIGGERED\\[4\\]
event"]
pub struct TRIGGERED4_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED4_AW::SET)
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[5\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED5_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TRIGGERED5_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED5` reader - Write '1' to enable interrupt for TRIGGERED\\[5\\]
event"]
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[5\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED5_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TRIGGERED5_AW> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED5` writer - Write '1' to enable interrupt for TRIGGERED\\[5\\]
event"]
pub struct TRIGGERED5_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED5_AW::SET)
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[6\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED6_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TRIGGERED6_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED6` reader - Write '1' to enable interrupt for TRIGGERED\\[6\\]
event"]
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[6\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED6_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TRIGGERED6_AW> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED6` writer - Write '1' to enable interrupt for TRIGGERED\\[6\\]
event"]
pub struct TRIGGERED6_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED6_AW::SET)
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[7\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED7_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TRIGGERED7_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED7` reader - Write '1' to enable interrupt for TRIGGERED\\[7\\]
event"]
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[7\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED7_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TRIGGERED7_AW> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED7` writer - Write '1' to enable interrupt for TRIGGERED\\[7\\]
event"]
pub struct TRIGGERED7_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED7_AW::SET)
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[8\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED8_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TRIGGERED8_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED8` reader - Write '1' to enable interrupt for TRIGGERED\\[8\\]
event"]
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[8\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED8_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TRIGGERED8_AW> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED8` writer - Write '1' to enable interrupt for TRIGGERED\\[8\\]
event"]
pub struct TRIGGERED8_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED8_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED8_AW::SET)
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[9\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED9_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TRIGGERED9_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED9` reader - Write '1' to enable interrupt for TRIGGERED\\[9\\]
event"]
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[9\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED9_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TRIGGERED9_AW> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED9` writer - Write '1' to enable interrupt for TRIGGERED\\[9\\]
event"]
pub struct TRIGGERED9_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED9_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED9_AW::SET)
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[10\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED10_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TRIGGERED10_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED10` reader - Write '1' to enable interrupt for TRIGGERED\\[10\\]
event"]
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[10\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED10_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TRIGGERED10_AW> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED10` writer - Write '1' to enable interrupt for TRIGGERED\\[10\\]
event"]
pub struct TRIGGERED10_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED10_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED10_AW::SET)
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[11\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED11_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TRIGGERED11_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED11` reader - Write '1' to enable interrupt for TRIGGERED\\[11\\]
event"]
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[11\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED11_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TRIGGERED11_AW> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED11` writer - Write '1' to enable interrupt for TRIGGERED\\[11\\]
event"]
pub struct TRIGGERED11_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED11_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED11_AW::SET)
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[12\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED12_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TRIGGERED12_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED12` reader - Write '1' to enable interrupt for TRIGGERED\\[12\\]
event"]
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[12\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED12_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TRIGGERED12_AW> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED12` writer - Write '1' to enable interrupt for TRIGGERED\\[12\\]
event"]
pub struct TRIGGERED12_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED12_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED12_AW::SET)
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[13\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED13_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TRIGGERED13_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED13` reader - Write '1' to enable interrupt for TRIGGERED\\[13\\]
event"]
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[13\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED13_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TRIGGERED13_AW> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED13` writer - Write '1' to enable interrupt for TRIGGERED\\[13\\]
event"]
pub struct TRIGGERED13_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED13_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED13_AW::SET)
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[14\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED14_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TRIGGERED14_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED14` reader - Write '1' to enable interrupt for TRIGGERED\\[14\\]
event"]
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[14\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED14_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TRIGGERED14_AW> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED14` writer - Write '1' to enable interrupt for TRIGGERED\\[14\\]
event"]
pub struct TRIGGERED14_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED14_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED14_AW::SET)
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[15\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED15_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TRIGGERED15_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED15` reader - Write '1' to enable interrupt for TRIGGERED\\[15\\]
event"]
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
#[doc = "Write '1' to enable interrupt for TRIGGERED\\[15\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED15_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TRIGGERED15_AW> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED15` writer - Write '1' to enable interrupt for TRIGGERED\\[15\\]
event"]
pub struct TRIGGERED15_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED15_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TRIGGERED15_AW::SET)
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
    #[doc = "Bit 0 - Write '1' to enable interrupt for TRIGGERED\\[0\\]
event"]
    #[inline(always)]
    pub fn triggered0(&self) -> TRIGGERED0_R {
        TRIGGERED0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for TRIGGERED\\[1\\]
event"]
    #[inline(always)]
    pub fn triggered1(&self) -> TRIGGERED1_R {
        TRIGGERED1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for TRIGGERED\\[2\\]
event"]
    #[inline(always)]
    pub fn triggered2(&self) -> TRIGGERED2_R {
        TRIGGERED2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for TRIGGERED\\[3\\]
event"]
    #[inline(always)]
    pub fn triggered3(&self) -> TRIGGERED3_R {
        TRIGGERED3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for TRIGGERED\\[4\\]
event"]
    #[inline(always)]
    pub fn triggered4(&self) -> TRIGGERED4_R {
        TRIGGERED4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for TRIGGERED\\[5\\]
event"]
    #[inline(always)]
    pub fn triggered5(&self) -> TRIGGERED5_R {
        TRIGGERED5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for TRIGGERED\\[6\\]
event"]
    #[inline(always)]
    pub fn triggered6(&self) -> TRIGGERED6_R {
        TRIGGERED6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for TRIGGERED\\[7\\]
event"]
    #[inline(always)]
    pub fn triggered7(&self) -> TRIGGERED7_R {
        TRIGGERED7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write '1' to enable interrupt for TRIGGERED\\[8\\]
event"]
    #[inline(always)]
    pub fn triggered8(&self) -> TRIGGERED8_R {
        TRIGGERED8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for TRIGGERED\\[9\\]
event"]
    #[inline(always)]
    pub fn triggered9(&self) -> TRIGGERED9_R {
        TRIGGERED9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Write '1' to enable interrupt for TRIGGERED\\[10\\]
event"]
    #[inline(always)]
    pub fn triggered10(&self) -> TRIGGERED10_R {
        TRIGGERED10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Write '1' to enable interrupt for TRIGGERED\\[11\\]
event"]
    #[inline(always)]
    pub fn triggered11(&self) -> TRIGGERED11_R {
        TRIGGERED11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Write '1' to enable interrupt for TRIGGERED\\[12\\]
event"]
    #[inline(always)]
    pub fn triggered12(&self) -> TRIGGERED12_R {
        TRIGGERED12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Write '1' to enable interrupt for TRIGGERED\\[13\\]
event"]
    #[inline(always)]
    pub fn triggered13(&self) -> TRIGGERED13_R {
        TRIGGERED13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Write '1' to enable interrupt for TRIGGERED\\[14\\]
event"]
    #[inline(always)]
    pub fn triggered14(&self) -> TRIGGERED14_R {
        TRIGGERED14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Write '1' to enable interrupt for TRIGGERED\\[15\\]
event"]
    #[inline(always)]
    pub fn triggered15(&self) -> TRIGGERED15_R {
        TRIGGERED15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for TRIGGERED\\[0\\]
event"]
    #[inline(always)]
    pub fn triggered0(&mut self) -> TRIGGERED0_W {
        TRIGGERED0_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for TRIGGERED\\[1\\]
event"]
    #[inline(always)]
    pub fn triggered1(&mut self) -> TRIGGERED1_W {
        TRIGGERED1_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for TRIGGERED\\[2\\]
event"]
    #[inline(always)]
    pub fn triggered2(&mut self) -> TRIGGERED2_W {
        TRIGGERED2_W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for TRIGGERED\\[3\\]
event"]
    #[inline(always)]
    pub fn triggered3(&mut self) -> TRIGGERED3_W {
        TRIGGERED3_W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for TRIGGERED\\[4\\]
event"]
    #[inline(always)]
    pub fn triggered4(&mut self) -> TRIGGERED4_W {
        TRIGGERED4_W { w: self }
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for TRIGGERED\\[5\\]
event"]
    #[inline(always)]
    pub fn triggered5(&mut self) -> TRIGGERED5_W {
        TRIGGERED5_W { w: self }
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for TRIGGERED\\[6\\]
event"]
    #[inline(always)]
    pub fn triggered6(&mut self) -> TRIGGERED6_W {
        TRIGGERED6_W { w: self }
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for TRIGGERED\\[7\\]
event"]
    #[inline(always)]
    pub fn triggered7(&mut self) -> TRIGGERED7_W {
        TRIGGERED7_W { w: self }
    }
    #[doc = "Bit 8 - Write '1' to enable interrupt for TRIGGERED\\[8\\]
event"]
    #[inline(always)]
    pub fn triggered8(&mut self) -> TRIGGERED8_W {
        TRIGGERED8_W { w: self }
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for TRIGGERED\\[9\\]
event"]
    #[inline(always)]
    pub fn triggered9(&mut self) -> TRIGGERED9_W {
        TRIGGERED9_W { w: self }
    }
    #[doc = "Bit 10 - Write '1' to enable interrupt for TRIGGERED\\[10\\]
event"]
    #[inline(always)]
    pub fn triggered10(&mut self) -> TRIGGERED10_W {
        TRIGGERED10_W { w: self }
    }
    #[doc = "Bit 11 - Write '1' to enable interrupt for TRIGGERED\\[11\\]
event"]
    #[inline(always)]
    pub fn triggered11(&mut self) -> TRIGGERED11_W {
        TRIGGERED11_W { w: self }
    }
    #[doc = "Bit 12 - Write '1' to enable interrupt for TRIGGERED\\[12\\]
event"]
    #[inline(always)]
    pub fn triggered12(&mut self) -> TRIGGERED12_W {
        TRIGGERED12_W { w: self }
    }
    #[doc = "Bit 13 - Write '1' to enable interrupt for TRIGGERED\\[13\\]
event"]
    #[inline(always)]
    pub fn triggered13(&mut self) -> TRIGGERED13_W {
        TRIGGERED13_W { w: self }
    }
    #[doc = "Bit 14 - Write '1' to enable interrupt for TRIGGERED\\[14\\]
event"]
    #[inline(always)]
    pub fn triggered14(&mut self) -> TRIGGERED14_W {
        TRIGGERED14_W { w: self }
    }
    #[doc = "Bit 15 - Write '1' to enable interrupt for TRIGGERED\\[15\\]
event"]
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
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
