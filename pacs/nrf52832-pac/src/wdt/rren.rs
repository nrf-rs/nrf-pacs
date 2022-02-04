#[doc = "Register `RREN` reader"]
pub struct R(crate::R<RREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RREN` writer"]
pub struct W(crate::W<RREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RREN_SPEC>;
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
impl From<crate::W<RREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable or disable RR\\[0\\]
register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR0_A {
    #[doc = "0: Disable RR\\[0\\]
register"]
    DISABLED = 0,
    #[doc = "1: Enable RR\\[0\\]
register"]
    ENABLED = 1,
}
impl From<RR0_A> for bool {
    #[inline(always)]
    fn from(variant: RR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR0` reader - Enable or disable RR\\[0\\]
register"]
pub struct RR0_R(crate::FieldReader<bool, RR0_A>);
impl RR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RR0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR0_A {
        match self.bits {
            false => RR0_A::DISABLED,
            true => RR0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RR0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RR0_A::ENABLED
    }
}
impl core::ops::Deref for RR0_R {
    type Target = crate::FieldReader<bool, RR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RR0` writer - Enable or disable RR\\[0\\]
register"]
pub struct RR0_W<'a> {
    w: &'a mut W,
}
impl<'a> RR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RR0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable RR\\[0\\]
register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR0_A::DISABLED)
    }
    #[doc = "Enable RR\\[0\\]
register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR0_A::ENABLED)
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
#[doc = "Enable or disable RR\\[1\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR1_A {
    #[doc = "0: Disable RR\\[1\\]
register"]
    DISABLED = 0,
    #[doc = "1: Enable RR\\[1\\]
register"]
    ENABLED = 1,
}
impl From<RR1_A> for bool {
    #[inline(always)]
    fn from(variant: RR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR1` reader - Enable or disable RR\\[1\\]
register"]
pub struct RR1_R(crate::FieldReader<bool, RR1_A>);
impl RR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RR1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR1_A {
        match self.bits {
            false => RR1_A::DISABLED,
            true => RR1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RR1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RR1_A::ENABLED
    }
}
impl core::ops::Deref for RR1_R {
    type Target = crate::FieldReader<bool, RR1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RR1` writer - Enable or disable RR\\[1\\]
register"]
pub struct RR1_W<'a> {
    w: &'a mut W,
}
impl<'a> RR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RR1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable RR\\[1\\]
register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR1_A::DISABLED)
    }
    #[doc = "Enable RR\\[1\\]
register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR1_A::ENABLED)
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
#[doc = "Enable or disable RR\\[2\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR2_A {
    #[doc = "0: Disable RR\\[2\\]
register"]
    DISABLED = 0,
    #[doc = "1: Enable RR\\[2\\]
register"]
    ENABLED = 1,
}
impl From<RR2_A> for bool {
    #[inline(always)]
    fn from(variant: RR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR2` reader - Enable or disable RR\\[2\\]
register"]
pub struct RR2_R(crate::FieldReader<bool, RR2_A>);
impl RR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RR2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR2_A {
        match self.bits {
            false => RR2_A::DISABLED,
            true => RR2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RR2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RR2_A::ENABLED
    }
}
impl core::ops::Deref for RR2_R {
    type Target = crate::FieldReader<bool, RR2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RR2` writer - Enable or disable RR\\[2\\]
register"]
pub struct RR2_W<'a> {
    w: &'a mut W,
}
impl<'a> RR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RR2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable RR\\[2\\]
register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR2_A::DISABLED)
    }
    #[doc = "Enable RR\\[2\\]
register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR2_A::ENABLED)
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
#[doc = "Enable or disable RR\\[3\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR3_A {
    #[doc = "0: Disable RR\\[3\\]
register"]
    DISABLED = 0,
    #[doc = "1: Enable RR\\[3\\]
register"]
    ENABLED = 1,
}
impl From<RR3_A> for bool {
    #[inline(always)]
    fn from(variant: RR3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR3` reader - Enable or disable RR\\[3\\]
register"]
pub struct RR3_R(crate::FieldReader<bool, RR3_A>);
impl RR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RR3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR3_A {
        match self.bits {
            false => RR3_A::DISABLED,
            true => RR3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RR3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RR3_A::ENABLED
    }
}
impl core::ops::Deref for RR3_R {
    type Target = crate::FieldReader<bool, RR3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RR3` writer - Enable or disable RR\\[3\\]
register"]
pub struct RR3_W<'a> {
    w: &'a mut W,
}
impl<'a> RR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RR3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable RR\\[3\\]
register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR3_A::DISABLED)
    }
    #[doc = "Enable RR\\[3\\]
register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR3_A::ENABLED)
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
#[doc = "Enable or disable RR\\[4\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR4_A {
    #[doc = "0: Disable RR\\[4\\]
register"]
    DISABLED = 0,
    #[doc = "1: Enable RR\\[4\\]
register"]
    ENABLED = 1,
}
impl From<RR4_A> for bool {
    #[inline(always)]
    fn from(variant: RR4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR4` reader - Enable or disable RR\\[4\\]
register"]
pub struct RR4_R(crate::FieldReader<bool, RR4_A>);
impl RR4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RR4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR4_A {
        match self.bits {
            false => RR4_A::DISABLED,
            true => RR4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RR4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RR4_A::ENABLED
    }
}
impl core::ops::Deref for RR4_R {
    type Target = crate::FieldReader<bool, RR4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RR4` writer - Enable or disable RR\\[4\\]
register"]
pub struct RR4_W<'a> {
    w: &'a mut W,
}
impl<'a> RR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RR4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable RR\\[4\\]
register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR4_A::DISABLED)
    }
    #[doc = "Enable RR\\[4\\]
register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR4_A::ENABLED)
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
#[doc = "Enable or disable RR\\[5\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR5_A {
    #[doc = "0: Disable RR\\[5\\]
register"]
    DISABLED = 0,
    #[doc = "1: Enable RR\\[5\\]
register"]
    ENABLED = 1,
}
impl From<RR5_A> for bool {
    #[inline(always)]
    fn from(variant: RR5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR5` reader - Enable or disable RR\\[5\\]
register"]
pub struct RR5_R(crate::FieldReader<bool, RR5_A>);
impl RR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RR5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR5_A {
        match self.bits {
            false => RR5_A::DISABLED,
            true => RR5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RR5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RR5_A::ENABLED
    }
}
impl core::ops::Deref for RR5_R {
    type Target = crate::FieldReader<bool, RR5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RR5` writer - Enable or disable RR\\[5\\]
register"]
pub struct RR5_W<'a> {
    w: &'a mut W,
}
impl<'a> RR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RR5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable RR\\[5\\]
register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR5_A::DISABLED)
    }
    #[doc = "Enable RR\\[5\\]
register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR5_A::ENABLED)
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
#[doc = "Enable or disable RR\\[6\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR6_A {
    #[doc = "0: Disable RR\\[6\\]
register"]
    DISABLED = 0,
    #[doc = "1: Enable RR\\[6\\]
register"]
    ENABLED = 1,
}
impl From<RR6_A> for bool {
    #[inline(always)]
    fn from(variant: RR6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR6` reader - Enable or disable RR\\[6\\]
register"]
pub struct RR6_R(crate::FieldReader<bool, RR6_A>);
impl RR6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RR6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR6_A {
        match self.bits {
            false => RR6_A::DISABLED,
            true => RR6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RR6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RR6_A::ENABLED
    }
}
impl core::ops::Deref for RR6_R {
    type Target = crate::FieldReader<bool, RR6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RR6` writer - Enable or disable RR\\[6\\]
register"]
pub struct RR6_W<'a> {
    w: &'a mut W,
}
impl<'a> RR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RR6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable RR\\[6\\]
register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR6_A::DISABLED)
    }
    #[doc = "Enable RR\\[6\\]
register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR6_A::ENABLED)
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
#[doc = "Enable or disable RR\\[7\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR7_A {
    #[doc = "0: Disable RR\\[7\\]
register"]
    DISABLED = 0,
    #[doc = "1: Enable RR\\[7\\]
register"]
    ENABLED = 1,
}
impl From<RR7_A> for bool {
    #[inline(always)]
    fn from(variant: RR7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RR7` reader - Enable or disable RR\\[7\\]
register"]
pub struct RR7_R(crate::FieldReader<bool, RR7_A>);
impl RR7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RR7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR7_A {
        match self.bits {
            false => RR7_A::DISABLED,
            true => RR7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RR7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RR7_A::ENABLED
    }
}
impl core::ops::Deref for RR7_R {
    type Target = crate::FieldReader<bool, RR7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RR7` writer - Enable or disable RR\\[7\\]
register"]
pub struct RR7_W<'a> {
    w: &'a mut W,
}
impl<'a> RR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RR7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable RR\\[7\\]
register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR7_A::DISABLED)
    }
    #[doc = "Enable RR\\[7\\]
register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR7_A::ENABLED)
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
    #[doc = "Bit 0 - Enable or disable RR\\[0\\]
register"]
    #[inline(always)]
    pub fn rr0(&self) -> RR0_R {
        RR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable or disable RR\\[1\\]
register"]
    #[inline(always)]
    pub fn rr1(&self) -> RR1_R {
        RR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable or disable RR\\[2\\]
register"]
    #[inline(always)]
    pub fn rr2(&self) -> RR2_R {
        RR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable or disable RR\\[3\\]
register"]
    #[inline(always)]
    pub fn rr3(&self) -> RR3_R {
        RR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable or disable RR\\[4\\]
register"]
    #[inline(always)]
    pub fn rr4(&self) -> RR4_R {
        RR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable or disable RR\\[5\\]
register"]
    #[inline(always)]
    pub fn rr5(&self) -> RR5_R {
        RR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable or disable RR\\[6\\]
register"]
    #[inline(always)]
    pub fn rr6(&self) -> RR6_R {
        RR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable or disable RR\\[7\\]
register"]
    #[inline(always)]
    pub fn rr7(&self) -> RR7_R {
        RR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable RR\\[0\\]
register"]
    #[inline(always)]
    pub fn rr0(&mut self) -> RR0_W {
        RR0_W { w: self }
    }
    #[doc = "Bit 1 - Enable or disable RR\\[1\\]
register"]
    #[inline(always)]
    pub fn rr1(&mut self) -> RR1_W {
        RR1_W { w: self }
    }
    #[doc = "Bit 2 - Enable or disable RR\\[2\\]
register"]
    #[inline(always)]
    pub fn rr2(&mut self) -> RR2_W {
        RR2_W { w: self }
    }
    #[doc = "Bit 3 - Enable or disable RR\\[3\\]
register"]
    #[inline(always)]
    pub fn rr3(&mut self) -> RR3_W {
        RR3_W { w: self }
    }
    #[doc = "Bit 4 - Enable or disable RR\\[4\\]
register"]
    #[inline(always)]
    pub fn rr4(&mut self) -> RR4_W {
        RR4_W { w: self }
    }
    #[doc = "Bit 5 - Enable or disable RR\\[5\\]
register"]
    #[inline(always)]
    pub fn rr5(&mut self) -> RR5_W {
        RR5_W { w: self }
    }
    #[doc = "Bit 6 - Enable or disable RR\\[6\\]
register"]
    #[inline(always)]
    pub fn rr6(&mut self) -> RR6_W {
        RR6_W { w: self }
    }
    #[doc = "Bit 7 - Enable or disable RR\\[7\\]
register"]
    #[inline(always)]
    pub fn rr7(&mut self) -> RR7_W {
        RR7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable register for reload request registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rren](index.html) module"]
pub struct RREN_SPEC;
impl crate::RegisterSpec for RREN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rren::R](R) reader structure"]
impl crate::Readable for RREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rren::W](W) writer structure"]
impl crate::Writable for RREN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RREN to value 0x01"]
impl crate::Resettable for RREN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
